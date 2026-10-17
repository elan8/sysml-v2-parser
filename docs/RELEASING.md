# Preparing and cutting a parser release

The next prepared release is **v0.58.0**, with **AST schema 284** and **Rust 1.88** as the MSRV.
The release-preparation PR depends on the issue #200 implementation in
[PR #202](https://github.com/elan8/sysml-v2-parser/pull/202). Merge that implementation first,
then land the release-preparation changes on `main`. Preparing the PR does not publish a crate
or create a tag.

## Preparation

1. Check `Cargo.toml`, the root `Cargo.lock` and `tools/parser_oracle/Cargo.lock` agree on the
   crate version. The latter is an independent tool package with a path dependency on the parser.
   Keep the AST schema independent of the crate version; do not bump it for documentation changes.
2. Move completed changelog entries into the release section, retaining an empty `Unreleased`
   section. Check the release date at the actual cut. Update README, the migration guide and
   current conformance/recovery documentation. Historical audit plans are context, not a support claim.
3. Fetch the pinned SysML release with `scripts/fetch-sysml-v2-release.sh` or its PowerShell
   counterpart. `docs/conformance-target` remains the grammar authority.
4. Run the commands below and confirm all PR checks, including the parser oracle, dependency
   audit and Rust 1.88 build, pass. Inspect the L1/L2 scorecard and package contents.

```sh
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
RUSTDOCFLAGS="-D warnings" cargo doc --no-deps --all-features
python tools/generate_reserved_keywords.py --check
cargo snapshot check
RUN_STRICT_LIBRARY_GATES=1 CONFORMANCE_SCORECARD_L2=1 cargo test --all-features -- --include-ignored
cargo package --locked
```

Set `SYSML_V2_RELEASE_DIR` when fixtures are outside the default `sysml-v2-release` directory.
PowerShell users should set each environment variable through `$env:NAME` before the Cargo
command. Package verification builds the packaged source; do not replace it with `--no-verify`
when validating release readiness. Use `cargo package --list` to inspect the archive inventory.

## Release execution

After the release-preparation PR is merged, a maintainer checks out the intended `main` commit,
confirms the manifest version is `0.58.0`, and creates and pushes the matching `v0.58.0` tag.
Do this as a separate publishing action; do not tag an unmerged preparation branch.

[The release workflow](../.github/workflows/release.yml) runs on `v*` tags. It validates,
packages, publishes to crates.io using the repository secret, and creates a GitHub Release with
auto-generated notes and the crate archive. Its package step uses `--no-verify`; the local
verified package check above supplies that additional preparation gate. A manual workflow run
validates/packages but skips the tag-gated publish and GitHub Release steps.

After publishing, verify the crates.io version, GitHub release assets and docs.rs build. Then
update downstream version or revision pins in their own PRs, using
[the v0.58.0 migration guide](MIGRATION_0_58.md). Do not infer compatibility of a particular
Spec42 or domain-library release from the parser's version number alone.
