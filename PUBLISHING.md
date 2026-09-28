# crates.io publication

This repository is prepared for crates.io trusted publishing through `.github/workflows/publish-crates.yml`.

## One-time registry setup

crates.io Trusted Publishing cannot publish a brand-new crate name for the first time. The crate owner must therefore complete the **first release manually** after confirming that `entity-rust-cleanroom` is available and the package contents are correct.

After the first crate version exists:

1. Open the crate settings on crates.io.
2. Add a GitHub Actions Trusted Publisher.
3. Set repository owner to `blackmore-technology-group`.
4. Set repository to `ENTITY-RUST-CLEANROOM`.
5. Set workflow filename to `publish-crates.yml`.
6. Do not configure a GitHub environment unless the workflow is changed to use the same environment.
7. After the trusted path is proven, prefer Trusted-Publishing-only mode over reusable API tokens.

## Release gate

A release tag must be exactly `v<Cargo.toml package.version>`. The workflow:

- requests a short-lived GitHub OIDC identity;
- verifies the tag matches the Cargo package version;
- runs the locked test suite;
- runs `cargo package --locked`;
- exchanges OIDC identity for a short-lived crates.io publishing token using `rust-lang/crates-io-auth-action`;
- publishes with `cargo publish --locked`.

No long-lived crates.io token is stored in this repository.

## Evidence boundary

crates.io publication improves Rust discovery and installation of the BTG-controlled verifier. It does not constitute unrelated third-party validation or an independently authored ENTITY implementation.
