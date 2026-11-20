# Coordinated release versions

Sapid 0.7.2 supports Glory 0.7.1 and later compatible 0.7 patches.
The tested engine versions are Apace 0.3.0, Takia 0.15.1 and Wingy 0.31.1.
Use the five sibling source repositories. Frozen publication checks use the
Glory revision in deploy/dependencies.json.

All packages require Rust 1.95 and workspaces use resolver 3. Cargo.lock files
in library, example and benchmark workspaces are ignored; the separate website
repository retains its lockfiles. Check scripts generate a missing local lock once
and then use --locked. To explicitly refresh an existing resolution, run cargo update.

Dependencies were checked against crates.io on 2026-10-30 and updated to the
latest versions compatible with Rust 1.95 and their shared public types.
Stylo uses the registry release 0.23.0 and Selectors 0.42.0.
Compatibility exceptions: Boa 0.22 requires ICU 2.3;
anyrender_svg 0.15 requires usvg 0.48.1; Parley/Vello share skrifa 0.44;
rusqlite_migration 2.6 requires rusqlite 0.40. Cap-std/cap-fs-ext 3.4.6,
and rfd 0.16.1 use the available compatible registry versions. Historical benchmark
comparison dependencies remain on the versions they measure.

Normal development CI checks sibling main branches. Frozen publication checks
use exact release tags and published engine sources. Registry-only consumers
independently verify the packaged dependency graph; sibling-source checks alone
do not establish registry compatibility.

The MSRV matrix checks both the product's pinned Glory revision and upstream main.
Only the disposable `upstream-main` CI checkout retargets Sapid's two Glory version
requirements to the checked-out facade version, before cache/lock preparation.
`scripts/prepare_upstream_msrv.py` refuses existing lockfiles and foreign dependency
paths. Product-pin and publication checks retain the committed dependency requirements;
an upstream compatibility result does not validate a new release dependency.
See [licensing](licensing.md) for Apache-2.0 and retained third-party notices.
