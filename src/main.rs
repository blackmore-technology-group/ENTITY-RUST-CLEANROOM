use aes_gcm::{Aes256Gcm, KeyInit, aead::Aead, Nonce};
use base64::{Engine as _, engine::general_purpose::STANDARD as B64};
use ed25519_dalek::{Signature, Verifier, VerifyingKey, pkcs8::DecodePublicKey};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{env, fs, path::{Path, PathBuf}};

const REQUIRED: &[&str] = &[
 "transaction_record","manifests","asset_provenance","rights","licence","usage_receipt",
 "license_settlement","value_record","digital_commodity","corporate_authorization","capital",
 "share_settlement","event_ledger","external_trust_anchors"
];

fn canonical(v:&Value)->String{
 match v {
  Value::Null=>"null".into(), Value::Bool(b)=>b.to_string(), Value::Number(n)=>n.to_string(),
  Value::String(s)=>serde_json::to_string(s).unwrap(),
  Value::Array(a)=>format!("[{}]",a.iter().map(canonical).collect::<Vec<_>>().join(",")),
  Value::Object(m)=>{let mut ks=m.keys().collect::<Vec<_>>();ks.sort();format!("{{{}}}",ks.into_iter().map(|k|format!("{}:{}",serde_json::to_string(k).unwrap(),canonical(&m[k]))).collect::<Vec<_>>().join(","))}
 }
}
fn sha_bytes(b:&[u8])->String{hex::encode(Sha256::digest(b))}
fn sha_text(s:&str)->String{sha_bytes(s.as_bytes())}
fn sval<'a>(v:&'a Value,k:&str)->&'a str{v.get(k).and_then(Value::as_str).unwrap_or("")}
fn ival(v:&Value,k:&str)->i64{v.get(k).and_then(Value::as_i64).unwrap_or(0)}
fn add(err:&mut Vec<String>, code:&str){if !err.iter().any(|x|x==code){err.push(code.into());}}
fn verify_signature(bundle:&Value)->bool{
 let sig=&bundle["signature"];
 if sval(sig,"alg")!="Ed25519"{return false}
 let der=match B64.decode(sval(sig,"public_key_spki_der_b64")){Ok(x)=>x,Err(_)=>return false};
 let key=match VerifyingKey::from_public_key_der(&der){Ok(x)=>x,Err(_)=>return false};
 let sb=match B64.decode(sval(sig,"sig_b64")){Ok(x)=>x,Err(_)=>return false};
 let signature=match Signature::from_slice(&sb){Ok(x)=>x,Err(_)=>return false};
 let header=json!({"schema":sval(bundle,"schema"),"transaction_id":sval(bundle,"transaction_id"),"issuer_entity_id":sval(bundle,"issuer_entity_id"),"transaction_root_sha256":sval(bundle,"transaction_root_sha256")});
 key.verify(canonical(&header).as_bytes(),&signature).is_ok()
}

fn verify_ledger(ledger:&Value, errors:&mut Vec<String>)->bool{
 let events=match ledger.get("events").and_then(Value::as_array){Some(x)=>x,None=>{add(errors,"LEDGER_SEQUENCE_INVALID");return false}};
 let mut prev="0".repeat(64);
 for (i,e) in events.iter().enumerate(){
  if ival(e,"sequence")!=(i as i64+1){add(errors,"LEDGER_SEQUENCE_INVALID");return false}
  if sval(e,"prev_hash")!=prev{add(errors,"LEDGER_PREV_HASH_INVALID");return false}
  let expected=sha_text(&(prev.clone()+":"+&canonical(&e["payload"])));
  if sval(e,"event_hash")!=expected{add(errors,"LEDGER_EVENT_HASH_INVALID");return false}
  prev=expected;
 }
 let cp=&ledger["checkpoint"];
 if ival(cp,"sequence")!=events.len() as i64 || sval(cp,"head_hash")!=prev{add(errors,"LEDGER_CHECKPOINT_INVALID");return false}
 true
}
fn verify_bundle(bundle:&Value)->Value{
 let mut errors=Vec::<String>::new(); let ev=&bundle["evidence"];
 let root=sha_text(&canonical(ev)); let root_valid=root==sval(bundle,"transaction_root_sha256"); if !root_valid{add(&mut errors,"ROOT_MISMATCH");}
 let signature_valid=verify_signature(bundle); if !signature_valid{add(&mut errors,"SIGNATURE_INVALID");}
 let mut required=true; for s in REQUIRED{if ev.get(*s).is_none(){required=false;add(&mut errors,&format!("MISSING_SECTION:{}",s));}}
 let mut cross=true;
 if required{
  let tr=&ev["transaction_record"];let p=&ev["asset_provenance"];let r=&ev["rights"];let l=&ev["licence"];let u=&ev["usage_receipt"];
  let s=&ev["license_settlement"];let v=&ev["value_record"];let d=&ev["digital_commodity"];let ca=&ev["corporate_authorization"];let c=&ev["capital"];let ss=&ev["share_settlement"];
  if sval(p,"asset_id")!=sval(tr,"asset_id"){cross=false;add(&mut errors,"PROVENANCE_ASSET_MISMATCH");}
  if sval(r,"asset_id")!=sval(tr,"asset_id"){cross=false;add(&mut errors,"RIGHTS_ASSET_MISMATCH");}
  if sval(r,"claimant_entity_id")!=sval(l,"grantor_entity_id"){cross=false;add(&mut errors,"RIGHTS_CLAIMANT_MISMATCH");}
  if sval(l,"asset_id")!=sval(tr,"asset_id")||sval(l,"grantor_entity_id")!=sval(tr,"grantor_entity_id")||sval(l,"licensee_entity_id")!=sval(tr,"licensee_entity_id")||sval(l,"rights_claim_id")!=sval(r,"claim_id"){cross=false;add(&mut errors,"LICENCE_LINK_MISMATCH");}
  if sval(u,"licence_id")!=sval(l,"licence_id")||sval(u,"asset_id")!=sval(tr,"asset_id")||sval(u,"user_entity_id")!=sval(l,"licensee_entity_id"){cross=false;add(&mut errors,"USAGE_LINK_MISMATCH");}
  if sval(u,"purpose")!=sval(l,"authorized_purpose"){cross=false;add(&mut errors,"USAGE_PURPOSE_UNAUTHORIZED");}
  if sval(s,"licence_id")!=sval(l,"licence_id"){cross=false;add(&mut errors,"SETTLEMENT_LINK_MISMATCH");}
  if sval(s,"payer_entity_id")!=sval(l,"licensee_entity_id")||sval(s,"payee_entity_id")!=sval(l,"grantor_entity_id"){cross=false;add(&mut errors,"SETTLEMENT_DIRECTION_INVALID");}
  if v.get("realized_external").and_then(Value::as_bool)==Some(true) && (sval(v,"settlement_id")!=sval(s,"settlement_id")||s.get("verified_external").and_then(Value::as_bool)!=Some(true)||ival(v,"amount_minor")>ival(s,"amount_minor")){cross=false;add(&mut errors,"VALUE_EXCEEDS_SETTLEMENT");}
  if sval(d,"asset_id")!=sval(tr,"asset_id")||sval(d,"usage_id")!=sval(u,"usage_id")||sval(d,"licence_id")!=sval(l,"licence_id")||sval(d,"settlement_id")!=sval(s,"settlement_id"){cross=false;add(&mut errors,"COMMODITY_LINK_MISMATCH");}
  if ival(d,"contribution_minor")>ival(s,"amount_minor"){cross=false;add(&mut errors,"COMMODITY_EXCEEDS_SETTLEMENT");}
  if sval(ca,"share_class_id")!=sval(c,"share_class_id")||sval(ca,"issuance_request_id")!=sval(c,"issuance_request_id"){cross=false;add(&mut errors,"CAPITAL_AUTH_MISMATCH");}
  if ival(&c["accounting"],"debit_minor")!=ival(&c["accounting"],"credit_minor"){cross=false;add(&mut errors,"CAPITAL_ACCOUNTING_UNBALANCED");}
  let pos:i64=c.get("positions").and_then(Value::as_array).map(|a|a.iter().map(|x|ival(x,"shares")).sum()).unwrap_or(0);
  if ival(c,"outstanding_before")+ival(c,"shares_issued")!=ival(c,"outstanding_after")||pos!=ival(c,"outstanding_after"){cross=false;add(&mut errors,"CAPITAL_SHARES_UNRECONCILED");}
  if sval(ss,"capital_event_id")!=sval(c,"capital_event_id"){cross=false;add(&mut errors,"SHARE_SETTLEMENT_LINK_MISMATCH");}
 } else {cross=false;}
 let ledger_valid=if required{verify_ledger(&ev["event_ledger"],&mut errors)}else{false};
 errors.sort(); let overall=root_valid&&signature_valid&&required&&cross&&ledger_valid&&errors.is_empty();
 json!({"schema":"entity-cleanroom-verification-result-v1","transaction_id":sval(bundle,"transaction_id"),"transaction_root_sha256":sval(bundle,"transaction_root_sha256"),"root_valid":root_valid,"signature_valid":signature_valid,"required_sections_valid":required,"cross_links_valid":cross,"ledger_valid":ledger_valid,"overall_valid":overall,"error_codes":errors})
}
fn result_hash(v:&Value)->String{sha_text(&canonical(v))}
fn verify_recovery(dir:&Path,key_file:&Path)->Value{
 let m:Value=serde_json::from_slice(&fs::read(dir.join("RECOVERY_MANIFEST.json")).unwrap()).unwrap();
 let bundle_bytes=fs::read(dir.join("TRANSACTION_BUNDLE.json")).unwrap(); let enc=fs::read(dir.join("STATE_BACKUP.enc")).unwrap();
 let key=hex::decode(fs::read_to_string(key_file).unwrap().trim()).unwrap();
 let mut unsigned=m.clone(); unsigned.as_object_mut().unwrap().remove("signature");
 let sig=&m["signature"]; let sig_ok=(||{
  let der=B64.decode(sval(sig,"public_key_spki_der_b64")).ok()?; let vk=VerifyingKey::from_public_key_der(&der).ok()?;
  let sb=B64.decode(sval(sig,"sig_b64")).ok()?; let sg=Signature::from_slice(&sb).ok()?;
  Some(vk.verify(canonical(&unsigned).as_bytes(),&sg).is_ok())
 })().unwrap_or(false);
 let hashes_ok=sha_bytes(&bundle_bytes)==sval(&m,"transaction_bundle_sha256")&&sha_bytes(&enc)==sval(&m,"encrypted_state_sha256")&&sha_bytes(&key)==sval(&m,"recovery_key_fingerprint_sha256");
 let nonce=B64.decode(sval(&m,"nonce_b64")).unwrap_or_default(); let cipher=Aes256Gcm::new_from_slice(&key).unwrap();
 let plain=cipher.decrypt(Nonce::from_slice(&nonce),enc.as_ref()).ok();
 let mut restored=String::new(); if let Some(p)=plain.as_ref(){if let Ok(state)=serde_json::from_slice::<Value>(p){restored=sha_text(&canonical(&state["evidence"]));}}
 let decrypt_ok=plain.is_some(); let overall=sig_ok&&hashes_ok&&decrypt_ok&&restored==sval(&m,"transaction_root_sha256");
 json!({"schema":"entity-cleanroom-recovery-result-v1","signature_valid":sig_ok,"hashes_valid":hashes_ok,"decrypt_valid":decrypt_ok,"restored_transaction_root_sha256":restored,"expected_transaction_root_sha256":sval(&m,"transaction_root_sha256"),"overall_valid":overall})
}
fn load(p:&Path)->Value{serde_json::from_slice(&fs::read(p).unwrap()).unwrap()}
fn run_campaign()->Value{
 let kit=PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("conformance-kit"); let manifest=load(&kit.join("vectors/VECTOR_MANIFEST.json"));
 let mut rows=Vec::new(); let mut passed=0usize;
 for v in manifest["vectors"].as_array().unwrap(){
  let name=sval(v,"name"); let b=load(&kit.join("vectors").join(sval(v,"file"))); let r=verify_bundle(&b); let h=result_hash(&r);
  let expected=&v["expected"]; let ok=r["overall_valid"]==expected["overall_valid"]&&r["error_codes"]==expected["error_codes"];
  if ok{passed+=1} rows.push(json!({"name":name,"ok":ok,"result_sha256":h,"result":r}));
 }
 let recovery=verify_recovery(&kit.join("vectors/recovery"),&kit.join("vectors/test_inputs/recovery_key.hex"));
 json!({"implementation":"rust","vectors_passed":passed,"vectors_total":manifest["vectors"].as_array().unwrap().len(),"recovery_pass":recovery["overall_valid"],"golden_root":manifest["valid_transaction_root_sha256"],"results":rows,"recovery":recovery})
}
fn main(){
 let args:Vec<String>=env::args().collect();
 if args.len()>1 && args[1]!="test"{let b=load(Path::new(&args[1]));let r=verify_bundle(&b);println!("{}",serde_json::to_string(&json!({"result_sha256":result_hash(&r),"result":r})).unwrap());std::process::exit(if r["overall_valid"]==Value::Bool(true){0}else{1});}
 let report=run_campaign(); println!("{}",serde_json::to_string_pretty(&report).unwrap());
 if report["vectors_passed"].as_u64()!=report["vectors_total"].as_u64()||report["recovery_pass"]!=Value::Bool(true){std::process::exit(1)}
}
