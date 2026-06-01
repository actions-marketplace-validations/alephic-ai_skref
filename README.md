# skref

[![CI](https://github.com/alephic-ai/skref/actions/workflows/ci.yml/badge.svg)](https://github.com/alephic-ai/skref/actions/workflows/ci.yml)

A fast Rust CLI and library for working with [Agent Skills](https://github.com/agentskills/agentskills) — the open `SKILL.md` format for extending AI agents.

`skref` is a Rust port of the Python [`skills-ref`](https://github.com/agentskills/agentskills/tree/main/skills-ref) reference library. It validates skills, reads their properties, and renders the `<available_skills>` prompt block — as a single static binary with no runtime dependencies.

> Like the original `skills-ref`, this is a reference implementation meant for demonstration and tooling. The behavior tracks the [Agent Skills specification](https://github.com/agentskills/agentskills/blob/main/docs/specification.mdx).

## Install

```bash
cargo install --path .          # from a checkout
# or
cargo install --git https://github.com/alephic-ai/skref skref
```

This produces a `skref` binary on your `PATH`.

## CLI

```bash
# Validate a skill directory (or a path directly to its SKILL.md)
skref validate path/to/skill

# Read skill properties as JSON
skref read-properties path/to/skill

# Generate the <available_skills> XML block for one or more skills
skref to-prompt path/to/skill-a path/to/skill-b
```

Exit codes: `0` on success, `1` on validation/parse errors.

### Examples

```console
$ skref validate sample-skills/pdf-processing
Valid skill: sample-skills/pdf-processing

$ skref read-properties sample-skills/pdf-processing
{
  "name": "pdf-processing",
  "description": "Extract text and tables from PDF files, ...",
  "license": "Apache-2.0",
  "compatibility": "Requires Python 3.11+ and the pypdf package",
  "metadata": {
    "author": "skref-examples",
    "version": "1.0"
  }
}

$ skref to-prompt sample-skills/hello-world
<available_skills>
<skill>
<name>
hello-world
</name>
<description>
A minimal example skill that greets the user. ...
</description>
<location>
/abs/path/sample-skills/hello-world/SKILL.md
</location>
</skill>
</available_skills>
```

## Library

`skref` is also a crate:

```rust
use std::path::Path;
use skref::{validate, read_properties, to_prompt};

let problems = validate(Path::new("my-skill"));
if problems.is_empty() {
    let props = read_properties(Path::new("my-skill"))?;
    println!("{} — {}", props.name, props.description);
}

let prompt = to_prompt(&[Path::new("skill-a"), Path::new("skill-b")])?;
println!("{prompt}");
# Ok::<(), skref::SkillError>(())
```

## The `SKILL.md` format

A skill is a directory containing a `SKILL.md` file with YAML frontmatter:

```markdown
---
name: my-skill
description: What the skill does and when to use it.
license: Apache-2.0          # optional
compatibility: Requires git  # optional, ≤ 500 chars
allowed-tools: Bash(git:*)   # optional, experimental
metadata:                    # optional, arbitrary key/value
  author: you
  version: "1.0"
---

# Instructions for the agent...
```

Validation rules enforced by `skref validate`:

| Field | Rule |
|-------|------|
| `name` | Required. ≤ 64 chars, lowercase (Unicode-aware), no leading/trailing or consecutive hyphens, only letters/digits/hyphens, **must match the directory name** (NFKC-normalized). |
| `description` | Required. Non-empty, ≤ 1024 chars. |
| `compatibility` | Optional. ≤ 500 chars. |
| `metadata` | Optional. Key/value mapping. |
| `license`, `allowed-tools` | Optional. |
| _other fields_ | Rejected as unexpected. |

International (i18n) skill names are supported, e.g. `технологии`, `技能`.

## GitHub Action

This repository **is** a GitHub Action. Validate every skill in any repo:

```yaml
# .github/workflows/skills.yml
name: Validate skills
on: [push, pull_request]
jobs:
  skills:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - uses: alephic-ai/skref@v1
        with:
          path: skills        # directory scanned recursively for SKILL.md
          fail-on-error: "true"
          to-prompt: "false"   # set "true" to also print <available_skills>
```

Inputs:

- `path` (default `.`) — directory scanned recursively; every directory containing a `SKILL.md` is validated.
- `fail-on-error` (default `true`) — fail the job if any skill is invalid.
- `to-prompt` (default `false`) — also print the `<available_skills>` block for the valid skills.

See [`.github/workflows/validate-skills.yml`](.github/workflows/validate-skills.yml) for a working example against the bundled samples.

> **Note:** the Action compiles `skref` from source on first use (`cargo install`), so the initial run takes a couple of minutes. Subsequent runs are fast thanks to the cargo cache. A future release will ship prebuilt binaries to remove the build step.

## Sample skills

[`sample-skills/`](sample-skills) contains valid skills used by the test suite and the Action demo:

- `hello-world` — the minimal valid skill.
- `pdf-processing` — a richer skill bundling a script and a reference doc.
- `git-commit-helper` — uses the experimental `allowed-tools` field.
- `перевод` — demonstrates i18n (non-Latin) skill names.

[`examples/invalid-skills/`](examples/invalid-skills) contains intentionally-broken skills used to test that validation fails.

Discover more skills to try at [skills.sh](https://www.skills.sh/).

## Development

```bash
cargo test          # unit + integration tests (ported from skills-ref + samples)
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## License

Apache-2.0. See [LICENSE](LICENSE).
