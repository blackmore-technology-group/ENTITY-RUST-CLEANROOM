# ENTITY-RUST-CLEANROOM

[![Clean-room verification](https://github.com/blackmore-technology-group/ENTITY-RUST-CLEANROOM/actions/workflows/cleanroom-verify.yml/badge.svg)](https://github.com/blackmore-technology-group/ENTITY-RUST-CLEANROOM/actions/workflows/cleanroom-verify.yml)
[![License](https://img.shields.io/github/license/blackmore-technology-group/ENTITY-RUST-CLEANROOM)](LICENSE)

**BTG-controlled Rust conformance baseline for ENTITY v3.4.2.**

> This repository is maintained and controlled by Blackmore Technology Group. It is cross-language reproducibility evidence. It is **not** an unrelated third-party implementation and must not be cited as independent external validation.

[ENTITY](https://github.com/blackmore-technology-group/ENTITY) · [v3.4.2 release](https://github.com/blackmore-technology-group/ENTITY/releases/tag/v3.4.2) · [Documentation portal](https://blackmore-technology-group.github.io/ENTITY-DOCS/) · [External verification challenge](https://github.com/blackmore-technology-group/ENTITY/issues/55) · [Rust reproduction task](https://github.com/blackmore-technology-group/ENTITY/issues/48)

## What this repository verifies

This Rust implementation exercises published ENTITY conformance campaigns, including:

- the frozen Protocol 1.0 / earlier clean-room baseline;
- the v3.2 adoption campaign;
- the v3.3 Verifiable Reality campaign;
- the **v3.4.2 Global Passport campaign**.

CI verifies sealed-kit checksums before executing the language-native classifiers and retains verification evidence as workflow artifacts.

## ENTITY v3.4.2 Global Passport campaign

The current Rust binary `src/bin/passport_v34.rs` is bound to:

`passport-conformance-kit-v342/ENTITY_V3_4_2_GLOBAL_PASSPORT_CLEANROOM_KIT.min.json`

Current campaign commitments:

- release context: **ENTITY v3.4.2 — Canonical BTDU Release**;
- sealed vectors: **26/26 PASS**;
- sealed-kit SHA-256: `ced70113f1d153627eb972b11adbf20e502ed086e0b13e8abf1dc5adc4c2e716`;
- canonical Rust campaign result SHA-256: `45af773554a7191c1b49a75c636a1106afb1de36d788bb00d7af56097b8d1b0e`.

Run the current Rust campaign:

```bash
cargo run --locked --release --bin passport_v34
```

The GitHub Actions workflow requires 26 passing vectors, the canonical result hash above, and `overall_valid: true`.

The repository also retains historical sealed material from earlier ENTITY campaigns for provenance and reproducibility. Those historical artifacts are not the current v3.4.2 Rust campaign target.

## Other controlled campaigns

```bash
cargo run --locked --release --bin entity-rust-cleanroom -- test
cargo run --locked --release --bin adoption_v32
cargo run --locked --release --bin reality_v33
```

See [`.github/workflows/cleanroom-verify.yml`](.github/workflows/cleanroom-verify.yml) for the complete CI procedure, toolchain setup, checksum verification, evidence capture and artifact retention.

## Where this fits in ENTITY

ENTITY v3.4.2 adds the Blackmore Technology Data Universe (BTDU) while preserving the Global Passport/profile architecture and the core protocol primitives:

`ENTITY → AUTHORITY → RIGHT → EVENT → VALUE`

The architecture is intended to preserve identity, authority, rights, evidence, provenance and portable economic state without making infrastructure possession equivalent to sovereign authority.

The main repository also publishes implementation packages for Healthcare, Finance, Manufacturing, AI, Robotics and Defence/Public-Unclassified.

If you are evaluating ENTITY rather than this Rust baseline specifically, start at the [ENTITY repository](https://github.com/blackmore-technology-group/ENTITY), the [documentation portal](https://blackmore-technology-group.github.io/ENTITY-DOCS/), or the [v3.4.2 external verification challenge](https://github.com/blackmore-technology-group/ENTITY/issues/55).

## Verification boundary

A passing result shows that this **BTG-controlled Rust implementation** classifies its sealed public campaign consistently with the published v3.4.2 target.

It does **not** establish:

- unrelated third-party validation;
- objective truth of an external-world claim;
- regulatory compliance for a deployment;
- legal title or accounting fair value;
- independent live interoperability merely because another BTG-controlled language converges on the same result;
- completion of the separate external hardware, physical multi-host, certified-device, independent-audit or independent-assessor gates.

The stronger external question remains open:

> Can an unrelated engineer or organization reproduce ENTITY semantics from public specifications and sealed test material without using BTG implementation code?

That is the purpose of the [ENTITY external verification challenge](https://github.com/blackmore-technology-group/ENTITY/issues/55).

## Contributing

Useful contributions include reproducible build failures, portability fixes, language-idiomatic improvements, test corrections, specification ambiguities and independently authored counterexamples.

If your goal is to produce **independent** conformance evidence, use a repository controlled outside BTG and follow the independence rules in the public verification challenge.

## License

Apache License 2.0. See [LICENSE](LICENSE).
