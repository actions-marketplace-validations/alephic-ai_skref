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
- `tests/` — `parser.rs`, `prompt.rs`, `validator.rs` are 1:1 ports of the Python tests; `sample_skills.rs` exercises the bundled skills.
- `sample-skills/`, `examples/invalid-skills/` — fixtures.

## Quality

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test
```

## Fidelity notes

- The frontmatter split mirrors Python's `content.split("---", 2)`.
- Names are NFKC-normalized before validation and directory-name comparison.
- `html.escape(quote=True)` is reproduced in `prompt::html_escape` (escapes `& < > " '`).
- `read-properties` JSON uses 2-space indent and preserves insertion order to match `json.dumps(indent=2)`.
