//! Tests for the validator module — ported from `tests/test_validator.py`.

use skref::validator::validate;
use skref::{Options, validate_with_options};
use std::fs;
use tempfile::tempdir;

fn write_skill(dir: &std::path::Path, name: &str, body: &str) -> std::path::PathBuf {
    let skill_dir = dir.join(name);
    fs::create_dir(&skill_dir).unwrap();
    fs::write(skill_dir.join("SKILL.md"), body).unwrap();
    skill_dir
}

#[test]
fn valid_skill() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\n---\n# My Skill\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn nonexistent_path() {
    let tmp = tempdir().unwrap();
    let errors = validate(&tmp.path().join("nonexistent"));
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("does not exist"));
}

#[test]
fn not_a_directory() {
    let tmp = tempdir().unwrap();
    let file_path = tmp.path().join("file.txt");
    fs::write(&file_path, "test").unwrap();
    let errors = validate(&file_path);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Not a directory"));
}

#[test]
fn missing_skill_md() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    let errors = validate(&skill_dir);
    assert_eq!(errors.len(), 1);
    assert!(errors[0].contains("Missing required file: SKILL.md"));
}

#[test]
fn invalid_name_uppercase() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "MySkill",
        "---\nname: MySkill\ndescription: A test skill\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("lowercase")));
}

#[test]
fn name_too_long() {
    let tmp = tempdir().unwrap();
    let long_name = "a".repeat(70);
    let body = format!("---\nname: {long_name}\ndescription: A test skill\n---\nBody\n");
    let skill_dir = write_skill(tmp.path(), &long_name, &body);
    let errors = validate(&skill_dir);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("exceeds") && e.contains("character limit"))
    );
}

#[test]
fn name_leading_hyphen() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "-my-skill",
        "---\nname: -my-skill\ndescription: A test skill\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("cannot start or end with a hyphen"))
    );
}

#[test]
fn name_consecutive_hyphens() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my--skill",
        "---\nname: my--skill\ndescription: A test skill\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("consecutive hyphens")));
}

#[test]
fn name_invalid_characters() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my_skill",
        "---\nname: my_skill\ndescription: A test skill\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("invalid characters")));
}

#[test]
fn name_directory_mismatch() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "wrong-name",
        "---\nname: correct-name\ndescription: A test skill\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("must match skill name")));
}

#[test]
fn unexpected_fields() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nunknown_field: should not be here\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("Unexpected fields")));
}

#[test]
fn claude_fields_rejected_by_default() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nmodel: inherit\ndisable-model-invocation: true\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("Unexpected fields")));
}

#[test]
fn claude_fields_accepted_with_option() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nmodel: inherit\ndisable-model-invocation: true\nwhen_to_use: when testing\n---\nBody\n",
    );
    let opts = Options {
        allow_claude_fields: true,
    };
    assert_eq!(
        validate_with_options(&skill_dir, opts),
        Vec::<String>::new()
    );
}

#[test]
fn unknown_field_still_rejected_with_option() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nmodel: inherit\nbogus: nope\n---\nBody\n",
    );
    let opts = Options {
        allow_claude_fields: true,
    };
    let errors = validate_with_options(&skill_dir, opts);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("Unexpected fields") && e.contains("bogus")),
        "expected `bogus` to still be rejected, got: {errors:?}"
    );
}

#[test]
fn valid_with_all_fields() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nlicense: MIT\nmetadata:\n  author: Test\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn allowed_tools_accepted() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\nallowed-tools: Bash(jq:*) Bash(git:*)\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn i18n_chinese_name() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "技能",
        "---\nname: 技能\ndescription: A skill with Chinese name\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn i18n_russian_name_with_hyphens() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "мой-навык",
        "---\nname: мой-навык\ndescription: A skill with Russian name\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn i18n_russian_lowercase_valid() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "навык",
        "---\nname: навык\ndescription: A skill with Russian lowercase name\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn i18n_russian_uppercase_rejected() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "НАВЫК",
        "---\nname: НАВЫК\ndescription: A skill with Russian uppercase name\n---\nBody\n",
    );
    let errors = validate(&skill_dir);
    assert!(errors.iter().any(|e| e.contains("lowercase")));
}

#[test]
fn description_too_long() {
    let tmp = tempdir().unwrap();
    let long_desc = "x".repeat(1100);
    let body = format!("---\nname: my-skill\ndescription: {long_desc}\n---\nBody\n");
    let skill_dir = write_skill(tmp.path(), "my-skill", &body);
    let errors = validate(&skill_dir);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("exceeds") && e.contains("1024"))
    );
}

#[test]
fn valid_compatibility() {
    let tmp = tempdir().unwrap();
    let skill_dir = write_skill(
        tmp.path(),
        "my-skill",
        "---\nname: my-skill\ndescription: A test skill\ncompatibility: Requires Python 3.11+\n---\nBody\n",
    );
    assert_eq!(validate(&skill_dir), Vec::<String>::new());
}

#[test]
fn compatibility_too_long() {
    let tmp = tempdir().unwrap();
    let long_compat = "x".repeat(550);
    let body = format!(
        "---\nname: my-skill\ndescription: A test skill\ncompatibility: {long_compat}\n---\nBody\n"
    );
    let skill_dir = write_skill(tmp.path(), "my-skill", &body);
    let errors = validate(&skill_dir);
    assert!(
        errors
            .iter()
            .any(|e| e.contains("exceeds") && e.contains("500"))
    );
}

#[test]
fn nfkc_normalization() {
    // Directory uses the composed form 'café'; SKILL.md uses the decomposed
    // form 'cafe' + U+0301. They must match after NFKC normalization.
    let tmp = tempdir().unwrap();
    let decomposed_name = "cafe\u{0301}";
    let composed_name = "café";
    let body = format!("---\nname: {decomposed_name}\ndescription: A test skill\n---\nBody\n");
    let skill_dir = write_skill(tmp.path(), composed_name, &body);
    let errors = validate(&skill_dir);
    assert_eq!(
        errors,
        Vec::<String>::new(),
        "Expected no errors, got: {errors:?}"
    );
}
