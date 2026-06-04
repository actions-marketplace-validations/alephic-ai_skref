# Development

`skref` is a Rust port of the Python [`skills-ref`](https://github.com/agentskills/agentskills/tree/main/skills-ref) reference library. Behavior should stay faithful to that library and to the [Agent Skills specification](https://github.com/agentskills/agentskills/blob/main/docs/specification.mdx).

## Layout

- `src/lib.rs` — public API re-exports.
- `src/main.rs` — the `skref` CLI (clap).
- `src/parser.rs` — `find_skill_md`, `parse_frontmatter`, `read_properties`.
- `src/validator.rs` — `validate` / `validate_metadata` and the field rules.
- `src/prompt.rs` — `to_prompt` (`<available_skills>` XML).
- `src/models.rs` — `SkillProperties` + JSON output.
- `src/yaml.rs` — frontmatter value model. Mirrors `strictyaml`: every scalar is coerced to a string.
- `src/errors.rs` — `SkillError` (Parse / Validation).
- `tests/` — `parser.rs`, `prompt.rs`, `validator.rs` are 1:1 ports of the Python tests; `sample_skills.rs` exercises the on-disk fixtures.
- `tests/fixtures/` — `sample-skills/` (base-spec valid), `invalid-skills/` (intentionally broken), `claude-skills/` (valid only with `--allow-claude-fields`; `claude-everything` uses every supported field).

## Quality

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

## Releases

`dist` (cargo-dist) builds the prebuilt binaries. **Tag releases as `vX.Y.Z`** (matching
`Cargo.toml`'s version). The `action.yml` install step downloads
`releases/download/v$VERSION/…`, so a tag without the `v` prefix leaves the binaries
unreachable and forces the action onto its slower source-build fallback.

See [`doc/releasing.md`](doc/releasing.md) for the full step-by-step release process
(version-bump PR, autopublish on tag, and moving the `v1` tag).

## Fidelity notes

- The frontmatter split mirrors Python's `content.split("---", 2)`.
- Names are NFKC-normalized before validation and directory-name comparison.
- `html.escape(quote=True)` is reproduced in `prompt::html_escape` (escapes `& < > " '`).
- `read-properties` JSON uses 2-space indent and preserves insertion order to match `json.dumps(indent=2)`.
- `--allow-claude-fields` (CLI flag, library `allow_claude_fields` bool parameter, action input `allow-claude-fields`) is a `skref` extension beyond the Python `skills-ref`: it whitelists Claude Code's `CLAUDE_FIELDS` for `validate` and surfaces them in `read-properties`. Default behavior is unchanged, so base-spec fidelity holds when the flag is off.
