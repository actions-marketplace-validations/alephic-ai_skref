# skref

[![CI](https://github.com/alephic-ai/skref/actions/workflows/ci.yml/badge.svg)](https://github.com/alephic-ai/skref/actions/workflows/ci.yml)

A fast Rust CLI for working with [Agent Skills](https://github.com/agentskills/agentskills) — the open `SKILL.md` format for extending AI agents.

`skref` is a Rust port of the Python [`skills-ref`](https://github.com/agentskills/agentskills/tree/main/skills-ref) reference library. It validates skills, reads their properties, and renders the `<available_skills>` prompt block — as a single static binary with no runtime dependencies.

> Like the original `skills-ref`, this is a reference implementation meant for demonstration and tooling. The behavior tracks the [Agent Skills specification](https://github.com/agentskills/agentskills/blob/main/docs/specification.mdx).

## Install

### With Cargo

```bash
cargo install skref
```

### Prebuilt binaries

Prefer not to compile? Every [release](https://github.com/alephic-ai/skref/releases) ships
prebuilt binaries for Linux (x86_64, aarch64), macOS (Intel and Apple Silicon), and Windows
(x86_64). Install the latest with the one-line installer:

```bash
# macOS / Linux
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/alephic-ai/skref/releases/latest/download/skref-installer.sh | sh
```

```powershell
# Windows
powershell -ExecutionPolicy ByPass -c "irm https://github.com/alephic-ai/skref/releases/latest/download/skref-installer.ps1 | iex"
```

Or download the archive for your platform from the [releases page](https://github.com/alephic-ai/skref/releases) and extract the `skref` binary onto your `PATH`.

### From source

For an unreleased revision:

```bash
cargo install --git https://github.com/alephic-ai/skref
# or, from a checkout:
cargo install --path .
```

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

#### Claude Code frontmatter fields

[Claude Code](https://code.claude.com/docs/en/skills#frontmatter-reference) layers extra
frontmatter fields on top of the base spec (`when_to_use`, `argument-hint`, `arguments`,
`disable-model-invocation`, `user-invocable`, `disallowed-tools`, `model`, `effort`,
`context`, `agent`, `hooks`, `paths`, `shell`). They are rejected by default. Pass
`--allow-claude-fields` to accept them in `validate` and surface them in
`read-properties` output:

```bash
skref validate path/to/skill --allow-claude-fields
skref read-properties path/to/skill --allow-claude-fields
```

### Examples

```console
$ skref validate tests/fixtures/sample-skills/pdf-processing
Valid skill: tests/fixtures/sample-skills/pdf-processing

$ skref read-properties tests/fixtures/sample-skills/pdf-processing
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

$ skref to-prompt tests/fixtures/sample-skills/hello-world
<available_skills>
<skill>
<name>
hello-world
</name>
<description>
A minimal example skill that greets the user. ...
</description>
<location>
/abs/path/tests/fixtures/sample-skills/hello-world/SKILL.md
</location>
</skill>
</available_skills>
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
| _other fields_ | Rejected as unexpected (unless `--allow-claude-fields` permits the Claude Code set). |

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
          to-prompt: "false"           # set "true" to also print <available_skills>
          allow-claude-fields: "false" # set "true" to accept Claude Code frontmatter fields
```

Inputs:

- `path` (default `.`) — directory scanned recursively; every directory containing a `SKILL.md` is validated.
- `fail-on-error` (default `true`) — fail the job if any skill is invalid.
- `to-prompt` (default `false`) — also print the `<available_skills>` block for the valid skills.
- `allow-claude-fields` (default `false`) — also accept Claude Code's extra frontmatter fields during validation.

See [`.github/workflows/validate-skills.yml`](.github/workflows/validate-skills.yml) for a working example against the bundled samples.

> **Note:** the Action downloads the prebuilt `skref` binary that matches the runner platform from the corresponding GitHub Release, so it starts in seconds. If no matching release exists for the referenced ref (e.g. when pinning to a branch), it transparently falls back to compiling from source with `cargo install`.

## Sample skills

All test fixtures live under [`tests/fixtures/`](tests/fixtures).

[`tests/fixtures/sample-skills/`](tests/fixtures/sample-skills) contains valid skills used by the test suite and the Action demo:

- `hello-world` — the minimal valid skill.
- `pdf-processing` — a richer skill bundling a script and a reference doc.
- `git-commit-helper` — uses the experimental `allowed-tools` field.
- `перевод` — demonstrates i18n (non-Latin) skill names.

[`tests/fixtures/claude-skills/`](tests/fixtures/claude-skills) contains skills that use Claude Code's extra frontmatter fields, valid only with `--allow-claude-fields`:

- `claude-everything` — uses every supported field (the six base fields plus all thirteen Claude Code extensions).
- `claude-pr-reviewer` — a realistic subset.

[`tests/fixtures/invalid-skills/`](tests/fixtures/invalid-skills) contains intentionally-broken skills used to test that validation fails.

Discover more skills to try at [skills.sh](https://www.skills.sh/).

## Development

```bash
cargo test          # unit + integration tests (ported from skills-ref + samples)
cargo fmt --all
cargo clippy --all-targets -- -D warnings
```

## License

Apache-2.0. See [LICENSE](LICENSE).
