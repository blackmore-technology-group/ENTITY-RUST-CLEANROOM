use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{fs, path::Path};

fn canonical(v:&Value)->String{
 match v{
  Value::Null=>"null".into(), Value::Bool(b)=>b.to_string(), Value::Number(n)=>n.to_string(),
  Value::String(s)=>serde_json::to_string(s).unwrap(),
  Value::Array(a)=>format!("[{}]",a.iter().map(canonical).collect::<Vec<_>>().join(",")),
  Value::Object(m)=>{let mut ks=m.keys().collect::<Vec<_>>();ks.sort();format!("{{{}}}",ks.into_iter().map(|k|format!("{}:{}",serde_json::to_string(k).unwrap(),canonical(&m[k]))).collect::<Vec<_>>().join(","))}
 }
}
fn sha_bytes(b:&[u8])->String{hex::encode(Sha256::digest(b))}
fn sha_value(v:&Value)->String{sha_bytes(canonical(v).as_bytes())}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn arr<'a>(v:&'a Value,k:&str)->&'a Vec<Value>{v.get(k).and_then(Value::as_array).unwrap()}
fn hex64(v:&Value,k:&str)->bool{v.get(k).and_then(Value::as_str).map(|s|s.len()==64&&s.bytes().all(|c|c.is_ascii_digit()||(b'a'..=b'f').contains(&c))).unwrap_or(false)}
fn sorted_unique(a:&Vec<Value>)->bool{let s=a.iter().filter_map(Value::as_str).collect::<Vec<_>>();let mut z=s.clone();z.sort();z.dedup();s==z}

fn valid_record(r:&Value)->bool{
 let schema=r.get("schema").and_then(Value::as_str).unwrap_or("");
 match schema{
  "entity-v3-jurisdiction-profile-v1"=>r.get("legal_effect_is_deployment_specific").and_then(Value::as_bool)==Some(true)&&arr(r,"rules").iter().all(|x|{
    let e=x.get("effect").and_then(Value::as_str).unwrap_or("");["ALLOW","REQUIRE","PROHIBIT"].contains(&e)&&x.get("actions").and_then(Value::as_array).map(|a|!a.is_empty()).unwrap_or(false)}),
  "entity-v3-semantic-term-v1"=>{
    let k=r.get("kind").and_then(Value::as_str).unwrap_or("");["SCHEMA","RIGHT","EVENT","CAPABILITY","ASSET_CLASS","TRUST_FRAMEWORK","DISPUTE_AUTHORITY","ATTESTATION_CLASS"].contains(&k)&&hex64(r,"definition_sha256")&&r.get("status").and_then(Value::as_str)==Some("ACTIVE")},
  "entity-v3-topology-node-v1"=>{
    let c=r.get("topology_class").and_then(Value::as_str).unwrap_or("");["CORE","REGIONAL","EDGE","SATELLITE","OFFLINE"].contains(&c)&&r.get("infrastructure_membership_is_not_sovereign_authority").and_then(Value::as_bool)==Some(true)},
  "entity-v3-purpose-bound-access-v1"=>!arr(r,"purposes").is_empty()&&!arr(r,"actions").is_empty()&&r.get("max_uses").and_then(Value::as_i64).map(|n|n>=0).unwrap_or(false),
  "entity-v3-offline-envelope-v1"=>hex64(r,"payload_sha256")&&r.get("sequence").and_then(Value::as_i64).is_some()&&r.get("expires_at_ms").and_then(Value::as_i64).unwrap_or(0)>r.get("created_at_ms").and_then(Value::as_i64).unwrap_or(0),
  "entity-v3-crypto-transition-v1"=>r.get("downgrade_after_transition_prohibited").and_then(Value::as_bool)==Some(true)&&r.get("old_retire_at_ms").and_then(Value::as_i64).unwrap_or(0)>=r.get("dual_sign_from_ms").and_then(Value::as_i64).unwrap_or(0),
  "entity-v3-data-economic-capital-v1"=>r.get("information_bytes_are_not_declared_scarce").and_then(Value::as_bool)==Some(true)&&hex64(r,"provenance_root")&&hex64(r,"content_sha256"),
  "entity-v3-bounded-economic-interest-v1"=>{
    let a=arr(r,"actions");let s=arr(r,"scarcity_sources");
    let allowed=["RIGHT","ENTITLEMENT","CAPACITY","DURATION","JURISDICTION","USAGE_QUANTITY","DERIVATION","PARTICIPATION","TRANSFERABILITY"];
    !a.is_empty()&&sorted_unique(a)&&s.iter().any(|x|x.as_str()==Some("RIGHT"))&&s.iter().all(|x|x.as_str().map(|v|allowed.contains(&v)).unwrap_or(false))&&
    r.get("underlying_information_remains_nonrival").and_then(Value::as_bool)==Some(true)&&
    r.get("participation_bps").and_then(Value::as_i64).map(|n|(0..=10000).contains(&n)).unwrap_or(false)
  },
  _=>false
 }
}

fn verify_checksums(root:&Path)->bool{
 let text=fs::read_to_string(root.join("SHA256SUMS.txt")).unwrap();
 for line in text.trim_start_matches('\u{feff}').lines(){
  if line.trim().is_empty(){continue} let mut parts=line.splitn(2,"  ");
  let expected=parts.next().unwrap_or("");let rel=parts.next().unwrap_or("");
  if rel.is_empty()||sha_bytes(&fs::read(root.join(rel)).unwrap())!=expected{return false}
 }
 true
}

pub fn run_global_campaign(root:&Path)->Value{
 let checksums=verify_checksums(root);
 let profile=load(&root.join("ENTITY_GLOBAL_CLEANROOM_PROFILE.json"));
 let manifest=load(&root.join("vectors/VECTOR_MANIFEST.json"));
 let mut rows=Vec::<Value>::new();
 for e in manifest["vectors"].as_array().unwrap(){
  let file=e["file"].as_str().unwrap();let p=load(&root.join("vectors").join(file));
  let accepted=valid_record(&p["record"]);let expected=p["expect"].as_str().unwrap();
  rows.push(json!({"name":file.trim_end_matches(".json"),"accepted":accepted,"expected":expected,"ok":accepted==(expected=="VALID")}));
 }
 rows.sort_by(|a,b|a["name"].as_str().cmp(&b["name"].as_str()));
 let summary=json!({"schema":"entity-v3.1-global-cleanroom-result-v1","profile":"ENTITY-GLOBAL-INFRASTRUCTURE",
  "doctrine_invariants":profile["doctrine_invariants"].clone(),"vectors":rows});
 let result_sha256=sha_value(&summary);
 let valid=summary["vectors"].as_array().unwrap().iter().filter(|x|x["accepted"]==Value::Bool(true)).count();
 let invalid=summary["vectors"].as_array().unwrap().len()-valid;
 let all_ok=checksums&&summary["vectors"].as_array().unwrap().iter().all(|x|x["ok"]==Value::Bool(true))&&
  result_sha256==profile["expected_result_sha256"].as_str().unwrap()&&valid==profile["valid_vectors"].as_u64().unwrap() as usize&&invalid==profile["invalid_vectors"].as_u64().unwrap() as usize;
 json!({"implementation":"rust","checksums_pass":checksums,"vectors_passed":summary["vectors"].as_array().unwrap().iter().filter(|x|x["ok"]==Value::Bool(true)).count(),
  "vectors_total":summary["vectors"].as_array().unwrap().len(),"result_sha256":result_sha256,
  "expected_result_sha256":profile["expected_result_sha256"],"doctrine_invariants":profile["doctrine_invariants"],
  "overall_valid":all_ok,"results":summary["vectors"]})
}
