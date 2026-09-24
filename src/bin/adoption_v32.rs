use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{collections::BTreeSet, fs};

const KIT: &str = "adoption-conformance-kit/ENTITY_V3_2_ADOPTION_CLEANROOM_KIT.min.json";
const KIT_SHA256: &str = "44e7a00f910c89aced3b3c1b5e9cba486809ca313e9bfb4b7bc9266095c10c14";
const EXPECTED_RESULT: &str = "1eb59e09ab08da86bfd8584df4a64ba331f7bbbce3d236b9b94f351606c90e18";
const PRIMITIVES: [&str; 5] = ["ENTITY", "AUTHORITY", "RIGHT", "EVENT", "VALUE"];
const LIFECYCLE: [&str; 13] = ["DCO","INSTRUMENT","LISTING","DISCLOSURE","ORDER_RFQ_AUCTION","PRICE_DISCOVERY","TRADE","CLEARING","SETTLEMENT","ENTITLEMENT","USAGE","DERIVED_OUTPUT","ECONOMIC_CONSEQUENCE"];

fn sha(data: &[u8]) -> String { format!("{:x}", Sha256::digest(data)) }
fn hex64(v: Option<&Value>) -> bool { v.and_then(Value::as_str).is_some_and(|s| s.len()==64 && s.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))) }
fn b(r:&Value,k:&str)->bool { r.get(k).and_then(Value::as_bool)==Some(true) }
fn f(r:&Value,k:&str)->bool { r.get(k).and_then(Value::as_bool)==Some(false) }
fn s<'a>(r:&'a Value,k:&str)->&'a str { r.get(k).and_then(Value::as_str).unwrap_or("") }
fn arr_eq(r:&Value,k:&str, want:&[&str])->bool { r.get(k).and_then(Value::as_array).is_some_and(|a| a.len()==want.len() && a.iter().zip(want).all(|(x,w)|x.as_str()==Some(*w))) }

fn valid_rules(r:&Value)->bool {
    let Some(rs)=r.get("rights").and_then(Value::as_array) else{return false};
    if rs.is_empty(){return false}
    rs.iter().all(|q| {
        matches!(s(q,"effect"),"ALLOW"|"REQUIRE"|"PROHIBIT") && q.get("actions").and_then(Value::as_array).is_some_and(|a| {
            if a.is_empty(){return false}; let xs:Vec<_>=a.iter().filter_map(Value::as_str).collect();
            xs.len()==a.len() && xs.iter().all(|x| !x.is_empty() && *x==x.to_uppercase()) && xs.windows(2).all(|w|w[0]<w[1])
        })
    })
}
fn valid(r:&Value)->bool {
    match s(r,"schema") {
        "entity-v3-rights-passport-v1" => arr_eq(r,"core_primitives",&PRIMITIVES) && valid_rules(r) && b(r,"provider_custody_is_not_authority") && b(r,"underlying_data_not_silently_transferred") && b(r,"legal_effect_is_deployment_specific"),
        "entity-v3-custody-locator-v1" => ["AWS_S3","AZURE_BLOB","GOOGLE_CLOUD_STORAGE","SNOWFLAKE","DATABRICKS","POSTGRESQL","SQL_SERVER","LOCAL_FILESYSTEM","HTTP_API"].contains(&s(r,"provider")) && hex64(r.get("content_sha256")) && f(r,"provider_is_authority") && f(r,"credentials_included") && f(r,"entity_identity_changes_with_provider"),
        "entity-v3-standards-mapping-v1" => ["ODRL","W3C_VC","DID","GAIA_X","IDS"].contains(&s(r,"source_standard")) && hex64(r.get("source_sha256")) && f(r,"silent_semantic_equivalence") && b(r,"external_standard_is_not_entity_authority"),
        "entity-v3-external-credential-evidence-v1" => s(r,"source_standard")=="W3C_VC" && hex64(r.get("credential_sha256")) && b(r,"credential_is_evidence_not_entity_authority"),
        "entity-v3-resolver-deployment-v1" => s(r,"mode")=="FEDERATED" && r.get("minimum_resolvers").and_then(Value::as_i64).unwrap_or(0)>=2 && b(r,"resolver_is_not_authority") && b(r,"single_provider_dependency_prohibited") && b(r,"fail_closed"),
        "entity-v3-exchange-adoption-profile-v1" => b(r,"market_engine_preserved") && b(r,"rights_are_traded_not_bytes") && arr_eq(r,"market_lifecycle",&LIFECYCLE),
        "entity-v3-adoption-profile-status-v1" => arr_eq(r,"core_primitives",&PRIMITIVES) && f(r,"core_semantics_changed") && b(r,"market_engine_preserved"),
        "entity-v3-legal-classification-assertion-v1" => !s(r,"asserted_by").is_empty() && !s(r,"classification").is_empty() && b(r,"classification_is_assertion_not_protocol_legal_truth"),
        _ => false,
    }
}
fn canonical(v:&Value)->String {
    match v {
        Value::Object(m)=>{ let mut keys:Vec<_>=m.keys().collect(); keys.sort(); format!("{{{}}}",keys.into_iter().map(|k|format!("{}:{}",serde_json::to_string(k).unwrap(),canonical(&m[k]))).collect::<Vec<_>>().join(",")) },
        Value::Array(a)=>format!("[{}]",a.iter().map(canonical).collect::<Vec<_>>().join(",")),
        _=>serde_json::to_string(v).unwrap(),
    }
}
fn main(){
    let raw=fs::read(KIT).expect("sealed v3.2 kit"); assert_eq!(sha(&raw),KIT_SHA256,"sealed kit SHA-256 mismatch");
    let kit:Value=serde_json::from_slice(&raw).unwrap(); let vectors=kit["vectors"].as_array().unwrap(); let mut rows=Vec::new(); let mut passed=0usize;
    for v in vectors { let accepted=valid(&v["record"]); let expected=s(v,"expect"); let ok=accepted==(expected=="VALID"); if ok{passed+=1}; rows.push(json!({"name":s(v,"name"),"accepted":accepted,"expected":expected,"ok":ok})); }
    rows.sort_by_key(|x|x["name"].as_str().unwrap().to_string());
    let invariants=kit["profile"]["adoption_invariants"].clone(); let summary=json!({"schema":"entity-v3.2-adoption-cleanroom-result-v1","profile":"ENTITY-ADOPTION-LAYER","adoption_invariants":invariants,"vectors":rows});
    let result=sha(canonical(&summary).as_bytes()); let overall=passed==16 && result==EXPECTED_RESULT;
    println!("{}",serde_json::to_string_pretty(&json!({"implementation":"rust","kit_sha256":KIT_SHA256,"vectors_passed":passed,"vectors_total":16,"result_sha256":result,"expected_result_sha256":EXPECTED_RESULT,"overall_valid":overall})).unwrap());
    if !overall{std::process::exit(1)}
}
