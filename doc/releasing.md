# Releasing `skref`

Releases are driven by a `vX.Y.Z` git tag. Pushing the tag publishes to crates.io
**and** builds the prebuilt binaries / GitHub Release automatically — there is no
manual `cargo publish` step.

The tag version must match `Cargo.toml`'s `version` (the GitHub Action downloads
`releases/download/v$VERSION/…`, so a mismatch or a missing `v` prefix leaves the
binaries unreachable and forces the Action onto its slower source-build fallback).

Pick the version by what changed: additive features → minor bump (`1.0.0` → `1.1.0`);
breaking API changes → major bump; fixes only → patch.

## 1. Bump the version (via a PR)

`main` has branch protection requiring a PR review approval — you cannot push to it
directly, so even a one-line version bump goes through a PR.

```bash
git checkout -b release/X.Y.Z
# edit Cargo.toml: version = "X.Y.Z"
cargo build            # refreshes skref's entry in Cargo.lock
git commit -am "Release vX.Y.Z"
git push -u origin release/X.Y.Z
gh pr create --title "Release vX.Y.Z" --body "Bump version for the … release."
```

Get the PR approved and merged.

## 2. Tag the merge commit (this auto-publishes)

```bash
git checkout main && git pull
git tag vX.Y.Z
git push origin vX.Y.Z
```

The `vX.Y.Z` tag fires two workflows:

- [`publish-crate.yml`](../.github/workflows/publish-crate.yml) — publishes to crates.io
  using **Trusted Publishing** (OIDC, no stored token).
- [`release.yml`](../.github/workflows/release.yml) — `dist` (cargo-dist) builds the
  prebuilt binaries and the GitHub Release.

Both trigger on tags matching `**[0-9]+.[0-9]+.[0-9]+*`.

## 3. Move the `v1` major tag

The Action is consumed as `alephic-ai/skref@v1`. The moving `v1` tag must point at the
latest `v1.x.y` release commit so `@v1` resolves to it (it reads that commit's
`Cargo.toml` version and downloads the matching binaries):

```bash
git tag -f v1 vX.Y.Z
git push -f origin v1
```

The bare `v1` tag does **not** match the workflow trigger pattern, so re-pointing it
publishes nothing. When cutting a new major (`v2.0.0`), create/move a `v2` tag the same
way.

## Optional: dry-run the publish

```bash
gh workflow run publish-crate.yml -f dry_run=true
```
