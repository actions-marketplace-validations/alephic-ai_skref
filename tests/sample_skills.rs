//! Integration tests over the bundled `sample-skills/` directory.
//!
//! The valid samples (everything outside `sample-skills/invalid/`) must pass
//! `validate`, expose readable properties, and render into a prompt block. The
//! samples under `invalid/` must each produce at least one validation error.

use std::path::{Path, PathBuf};

use skref::{read_properties, to_prompt, validate};

fn sample_skills_dir() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("sample-skills")
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
        let errors = validate(&dir);
        assert!(
            errors.is_empty(),
            "expected {} to be valid, got: {errors:?}",
            dir.display()
        );

        // Properties should be readable and well-formed.
        let props = read_properties(&dir).expect("read_properties succeeds");
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
        let props = read_properties(dir).unwrap();
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
    let invalid_root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("examples")
        .join("invalid-skills");
    let dirs = skill_dirs_in(&invalid_root);
    assert!(
        !dirs.is_empty(),
        "expected at least one invalid sample skill"
    );

    for dir in dirs {
        let errors = validate(&dir);
        assert!(
            !errors.is_empty(),
            "expected {} to be invalid",
            dir.display()
        );
    }
}
