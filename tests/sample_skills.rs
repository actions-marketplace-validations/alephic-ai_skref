//! Integration tests over the bundled skill fixtures in `tests/fixtures/`.
//!
//! * `sample-skills/` — base-spec-valid skills: must pass `validate`, expose
//!   readable properties, and render into a prompt block.
//! * `invalid-skills/` — each must produce at least one validation error.
//! * `claude-skills/` — valid only with `allow_claude_fields = true`; rejected
//!   under the base spec.

use std::path::{Path, PathBuf};

use skref::{read_properties, to_prompt, validate};

/// Root of the on-disk test fixtures (`tests/fixtures/`).
fn fixtures_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests")
        .join("fixtures")
}

fn sample_skills_dir() -> PathBuf {
    fixtures_dir().join("sample-skills")
}

/// Skill directories (those containing a SKILL.md) directly under `root`.
fn skill_dirs_in(root: &Path) -> Vec<PathBuf> {
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(root)
        .unwrap_or_else(|e| panic!("reading {}: {e}", root.display()))
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .filter(|p| p.is_dir() && skref::find_skill_md(p).is_some())
        .collect();
    dirs.sort();
    dirs
}

#[test]
fn valid_samples_validate_cleanly() {
    let dirs = skill_dirs_in(&sample_skills_dir());
    assert!(!dirs.is_empty(), "expected at least one valid sample skill");

    for dir in dirs {
        let errors = validate(&dir, false);
        assert!(
            errors.is_empty(),
            "expected {} to be valid, got: {errors:?}",
            dir.display()
        );

        // Properties should be readable and well-formed.
        let props = read_properties(&dir, false).expect("read_properties succeeds");
        assert!(!props.name.is_empty());
        assert!(!props.description.is_empty());
    }
}

#[test]
fn valid_samples_render_to_prompt() {
    let dirs = skill_dirs_in(&sample_skills_dir());
    let prompt = to_prompt(&dirs).expect("to_prompt succeeds");
    assert!(prompt.starts_with("<available_skills>"));
    assert!(prompt.trim_end().ends_with("</available_skills>"));
    assert_eq!(prompt.matches("<skill>").count(), dirs.len());
    for dir in &dirs {
        let props = read_properties(dir, false).unwrap();
        // Names are simple kebab/i18n strings with no XML-special characters.
        assert!(
            prompt.contains(&props.name),
            "prompt should mention {}",
            props.name
        );
    }
}

#[test]
fn invalid_samples_fail_validation() {
    let invalid_root = fixtures_dir().join("invalid-skills");
    let dirs = skill_dirs_in(&invalid_root);
    assert!(
        !dirs.is_empty(),
        "expected at least one invalid sample skill"
    );

    for dir in dirs {
        let errors = validate(&dir, false);
        assert!(
            !errors.is_empty(),
            "expected {} to be invalid",
            dir.display()
        );
    }
}

#[test]
fn claude_samples_validate_only_with_flag() {
    let dirs = skill_dirs_in(&fixtures_dir().join("claude-skills"));
    assert!(
        !dirs.is_empty(),
        "expected at least one Claude sample skill"
    );

    for dir in dirs {
        // Valid once Claude fields are allowed...
        assert!(
            validate(&dir, true).is_empty(),
            "expected {} to be valid with --allow-claude-fields, got: {:?}",
            dir.display(),
            validate(&dir, true)
        );
        // ...and rejected under the base spec.
        assert!(
            !validate(&dir, false).is_empty(),
            "expected {} to be rejected by the base spec",
            dir.display()
        );
        // The Claude fields are captured into properties.
        let props = read_properties(&dir, true).expect("read_properties succeeds");
        assert!(
            !props.claude.is_empty(),
            "expected {} to capture Claude fields",
            dir.display()
        );
    }
}

#[test]
fn claude_everything_skill_uses_every_field() {
    let dir = fixtures_dir()
        .join("claude-skills")
        .join("claude-everything");
    let props = read_properties(&dir, true).unwrap();

    // Every Claude-specific field is present (locks the fixture to CLAUDE_FIELDS).
    let keys: Vec<&str> = props.claude.iter().map(|(k, _)| k.as_str()).collect();
    for field in skref::CLAUDE_FIELDS {
        assert!(
            keys.contains(&field),
            "claude-everything is missing Claude field `{field}`"
        );
    }

    // The optional base-spec fields are populated too.
    assert!(props.license.is_some());
    assert!(props.allowed_tools.is_some());
    assert!(props.compatibility.is_some());
    assert!(!props.metadata.is_empty());
}
