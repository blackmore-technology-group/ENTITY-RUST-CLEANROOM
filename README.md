# ENTITY-RUST-CLEANROOM

[![Clean-room verification](https://github.com/blackmore-technology-group/ENTITY-RUST-CLEANROOM/actions/workflows/cleanroom-verify.yml/badge.svg)](https://github.com/blackmore-technology-group/ENTITY-RUST-CLEANROOM/actions/workflows/cleanroom-verify.yml)
[![License](https://img.shields.io/github/license/blackmore-technology-group/ENTITY-RUST-CLEANROOM)](LICENSE)

**Document class:** BTG-controlled reproducibility baseline  
**Frozen campaign target:** ENTITY v3.4.2 Global Passport  
**Current supported ENTITY runtime:** v3.4.3  
**BTDU component in v3.4.3:** Blackmore Technology Data Universe (BTDU) 3.4.2 unchanged

> This repository is maintained and controlled by Blackmore Technology Group Limited (BTG). It is cross-language reproducibility evidence. It is **not** an unrelated third-party implementation and must not be cited as independent external validation.

[ENTITY](https://github.com/blackmore-technology-group/ENTITY) · [Current v3.4.3 release](https://github.com/blackmore-technology-group/ENTITY/releases/tag/v3.4.3) · [Documentation model](https://blackmore-technology-group.github.io/ENTITY-DOCS/reference/documentation-model.html) · [External verification challenge](https://github.com/blackmore-technology-group/ENTITY/issues/55) · [Rust reproduction task](https://github.com/blackmore-technology-group/ENTITY/issues/48)

## What this repository verifies

This Rust implementation exercises several **BTG-controlled** published campaigns, including earlier clean-room material, v3.2 adoption, v3.3 Verifiable Reality and the frozen **v3.4.2 Global Passport campaign**.

The v3.4.2 label identifies the exact sealed campaign target. It does **not** mean v3.4.2 remains the current supported runtime.

CI verifies sealed-kit checksums before executing the language-native classifiers and retains workflow evidence.

## Frozen v3.4.2 Global Passport campaign

`src/bin/passport_v34.rs` is bound to:

`passport-conformance-kit-v342/ENTITY_V3_4_2_GLOBAL_PASSPORT_CLEANROOM_KIT.min.json`

Exact commitments remain unchanged:

- release/campaign context: **ENTITY v3.4.2 — Canonical BTDU Release**;
- sealed vectors: **26/26 PASS**;
- sealed-kit SHA-256: `ced70113f1d153627eb972b11adbf20e502ed086e0b13e8abf1dc5adc4c2e716`;
- canonical campaign result SHA-256: `45af773554a7191c1b49a75c636a1106afb1de36d788bb00d7af56097b8d1b0e`;
- required `overall_valid: true`.

Run the frozen campaign:

```bash
cargo run --locked --release --bin passport_v34
```

Historical material from earlier campaigns remains for provenance/reproducibility and is not rewritten.

## Other controlled campaigns

```bash
cargo run --locked --release --bin entity-rust-cleanroom -- test
cargo run --locked --release --bin adoption_v32
cargo run --locked --release --bin reality_v33
```

See [`.github/workflows/cleanroom-verify.yml`](.github/workflows/cleanroom-verify.yml) for the complete CI procedure.

## Where this fits now

ENTITY v3.4.3 is the current supported runtime. Its BTDU component remains **3.4.2 unchanged**. BTDU means **Blackmore Technology Data Universe** and is runtime/component architecture.

The frozen **ENTITY Protocol 1.0** external clean-room campaign is a separate target. BTDU, ADAM and NIKI are not additional Protocol 1.0 implementation requirements unless the sealed Protocol 1.0 material explicitly says otherwise.

## Verification boundary

A passing result shows that this **BTG-controlled Rust implementation** classifies the exact frozen campaign consistently with the published v3.4.2 target.

It does **not** by itself establish:

- an independently authored implementation;
- full independent live interoperability or sovereign recovery;
- objective external-world truth;
- regulatory compliance;
- legal title or accounting fair value;
- automatic ownership or economic entitlement.

The stronger external milestone remains an unrelated implementation authored and controlled outside BTG from the permitted public Protocol 1.0 material, followed by the required interoperability/recovery gates.

## Contributing

Reproducible build failures, portability fixes, language-idiomatic improvements, test corrections, specification ambiguities and independently authored counterexamples are useful. Independent conformance evidence should live in a repository controlled outside BTG.

## License

Apache License 2.0. See [LICENSE](LICENSE).
