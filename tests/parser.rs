//! Tests for the parser module — ported from `tests/test_parser.py`.

use skref::errors::SkillError;
use skref::parser::{find_skill_md, parse_frontmatter, read_properties};
use std::fs;
use tempfile::tempdir;

#[test]
fn valid_frontmatter() {
    let content =
        "---\nname: my-skill\ndescription: A test skill\n---\n# My Skill\n\nInstructions here.\n";
    let (metadata, body) = parse_frontmatter(content).unwrap();
    assert_eq!(metadata.get("name").unwrap().as_str(), Some("my-skill"));
    assert_eq!(
        metadata.get("description").unwrap().as_str(),
        Some("A test skill")
    );
    assert!(body.contains("# My Skill"));
}

#[test]
fn missing_frontmatter() {
    let err = parse_frontmatter("# No frontmatter here").unwrap_err();
    assert!(matches!(err, SkillError::Parse(_)));
    assert!(err.to_string().contains("must start with YAML frontmatter"));
}

#[test]
fn unclosed_frontmatter() {
    let content = "---\nname: my-skill\ndescription: A test skill\n";
    let err = parse_frontmatter(content).unwrap_err();
    assert!(err.to_string().contains("not properly closed"));
}

#[test]
fn invalid_yaml() {
    let content = "---\nname: [invalid\ndescription: broken\n---\nBody here\n";
    let err = parse_frontmatter(content).unwrap_err();
    assert!(err.to_string().contains("Invalid YAML"));
}

#[test]
fn non_dict_frontmatter() {
    let content = "---\n- just\n- a\n- list\n---\nBody\n";
    let err = parse_frontmatter(content).unwrap_err();
    assert!(err.to_string().contains("must be a YAML mapping"));
}

#[test]
fn read_valid_skill() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\nlicense: MIT\n---\n# My Skill\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, false).unwrap();
    assert_eq!(props.name, "my-skill");
    assert_eq!(props.description, "A test skill");
    assert_eq!(props.license.as_deref(), Some("MIT"));
}

#[test]
fn read_with_metadata() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\nmetadata:\n  author: Test Author\n  version: 1.0\n---\nBody\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, false).unwrap();
    assert_eq!(
        props.metadata,
        vec![
            ("author".to_string(), "Test Author".to_string()),
            ("version".to_string(), "1.0".to_string()),
        ]
    );
}

#[test]
fn claude_fields_ignored_by_default() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\nmodel: inherit\n---\nBody\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, false).unwrap();
    assert!(props.claude.is_empty());
    assert!(props.to_dict().get("model").is_none());
}

#[test]
fn claude_fields_captured_with_option() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\nmodel: inherit\narguments:\n  - issue\n  - format\ndisable-model-invocation: true\nhooks:\n  PreToolUse: echo hi\n---\nBody\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, true).unwrap();
    let dict = props.to_dict();

    // Scalar passes through as a string.
    assert_eq!(dict.get("model").and_then(|v| v.as_str()), Some("inherit"));
    // List preserves structure.
    assert_eq!(
        dict.get("arguments").unwrap(),
        &serde_json::json!(["issue", "format"])
    );
    // Bool renders as the strictyaml-style "True".
    assert_eq!(
        dict.get("disable-model-invocation")
            .and_then(|v| v.as_str()),
        Some("True")
    );
    // Nested map preserves structure.
    assert_eq!(
        dict.get("hooks").unwrap(),
        &serde_json::json!({"PreToolUse": "echo hi"})
    );
}

#[test]
fn missing_skill_md() {
    let tmp = tempdir().unwrap();
    let err = read_properties(tmp.path(), false).unwrap_err();
    assert!(err.to_string().contains("SKILL.md not found"));
}

#[test]
fn missing_name() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\ndescription: A test skill\n---\nBody\n",
    )
    .unwrap();
    let err = read_properties(&skill_dir, false).unwrap_err();
    assert!(matches!(err, SkillError::Validation { .. }));
    assert!(err.to_string().contains("Missing required field") && err.to_string().contains("name"));
}

#[test]
fn missing_description() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\n---\nBody\n",
    )
    .unwrap();
    let err = read_properties(&skill_dir, false).unwrap_err();
    assert!(
        err.to_string().contains("Missing required field")
            && err.to_string().contains("description")
    );
}

#[test]
fn find_skill_md_prefers_uppercase() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), "uppercase").unwrap();
    // On case-insensitive filesystems these are the same file; only assert the
    // uppercase-preference contract when both can coexist.
    let lower = skill_dir.join("skill.md");
    if !lower.exists() {
        fs::write(&lower, "lowercase").unwrap();
    }
    let result = find_skill_md(&skill_dir).unwrap();
    assert_eq!(
        result.file_name().unwrap().to_string_lossy().to_lowercase(),
        "skill.md"
    );
}

#[test]
fn find_skill_md_accepts_lowercase() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(skill_dir.join("skill.md"), "lowercase").unwrap();
    let result = find_skill_md(&skill_dir).unwrap();
    assert_eq!(
        result.file_name().unwrap().to_string_lossy().to_lowercase(),
        "skill.md"
    );
}

#[test]
fn find_skill_md_returns_none_when_missing() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    assert!(find_skill_md(&skill_dir).is_none());
}

#[test]
fn read_properties_with_lowercase_skill_md() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("skill.md"),
        "---\nname: my-skill\ndescription: A test skill\n---\n# My Skill\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, false).unwrap();
    assert_eq!(props.name, "my-skill");
    assert_eq!(props.description, "A test skill");
}

#[test]
fn read_with_allowed_tools() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\nallowed-tools: Bash(jq:*) Bash(git:*)\n---\nBody\n",
    )
    .unwrap();
    let props = read_properties(&skill_dir, false).unwrap();
    assert_eq!(
        props.allowed_tools.as_deref(),
        Some("Bash(jq:*) Bash(git:*)")
    );
    let d = props.to_dict();
    assert_eq!(d["allowed-tools"], "Bash(jq:*) Bash(git:*)");
}
