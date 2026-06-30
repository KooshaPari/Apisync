# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Changed

- `src/lib.rs`: replace blanket `pub use module::*` glob re-exports with explicit
  named re-exports, giving downstream crates a stable, documented public API surface
  (audit finding L0/L5).
- `.github/workflows/release.yml`: switch trigger from `push: branches: [main]` to
  `push: tags: v*`, remove broken `promote` job referencing a 404 placeholder action,
  add `--locked` flag to `cargo build`/`cargo publish`, add Cargo.toml↔tag version
  verification step, add SBOM generation via `cargo-cyclonedx`, and fix garbled
  `${{ }}` template expressions (audit finding L9/L17).
- `.github/workflows/quality-gate.yml`: remove `continue-on-error: true` from the
  coverage threshold check so a coverage regression actually fails the gate
  (audit finding L11).
- `CODEOWNERS`: tombstone root file with a redirect comment; `.github/CODEOWNERS` is
  the single authoritative source (audit finding L37).
- `ADR.md`: add canonical-source header noting `docs/adr/` wins on conflict
  (audit finding L37).

[Unreleased]: https://github.com/KooshaPari/Apisync/compare/main...HEAD
