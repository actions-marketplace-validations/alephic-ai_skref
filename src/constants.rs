//! Frontmatter field-name constants, in a neutral module that both `parser`
//! and `validator` can import without depending on each other.

/// Allowed frontmatter fields per the Agent Skills spec.
pub const ALLOWED_FIELDS: [&str; 6] = [
    "name",
    "description",
    "license",
    "allowed-tools",
    "metadata",
    "compatibility",
];

/// Claude Code's extra frontmatter fields, layered on top of [`ALLOWED_FIELDS`]
/// when validation/reading opts in via `allow_claude_fields`. `name`,
/// `description`, and `allowed-tools` are shared with the base spec and so are
/// not repeated here.
///
/// See <https://code.claude.com/docs/en/skills#frontmatter-reference>.
pub const CLAUDE_FIELDS: [&str; 13] = [
    "when_to_use",
    "argument-hint",
    "arguments",
    "disable-model-invocation",
    "user-invocable",
    "disallowed-tools",
    "model",
    "effort",
    "context",
    "agent",
    "hooks",
    "paths",
    "shell",
];
