//! Tests for the prompt module — ported from `tests/test_prompt.py`.

use skref::prompt::to_prompt;
use std::fs;
use std::path::PathBuf;
use tempfile::tempdir;

#[test]
fn empty_list() {
    let empty: Vec<PathBuf> = Vec::new();
    let result = to_prompt(&empty).unwrap();
    assert_eq!(result, "<available_skills>\n</available_skills>");
}

#[test]
fn single_skill() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("my-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: my-skill\ndescription: A test skill\n---\nBody\n",
    )
    .unwrap();
    let result = to_prompt(&[skill_dir]).unwrap();
    assert!(result.contains("<available_skills>"));
    assert!(result.contains("</available_skills>"));
    assert!(result.contains("<name>\nmy-skill\n</name>"));
    assert!(result.contains("<description>\nA test skill\n</description>"));
    assert!(result.contains("<location>"));
    assert!(result.contains("SKILL.md"));
}

#[test]
fn multiple_skills() {
    let tmp = tempdir().unwrap();
    let skill_a = tmp.path().join("skill-a");
    fs::create_dir(&skill_a).unwrap();
    fs::write(
        skill_a.join("SKILL.md"),
        "---\nname: skill-a\ndescription: First skill\n---\nBody\n",
    )
    .unwrap();
    let skill_b = tmp.path().join("skill-b");
    fs::create_dir(&skill_b).unwrap();
    fs::write(
        skill_b.join("SKILL.md"),
        "---\nname: skill-b\ndescription: Second skill\n---\nBody\n",
    )
    .unwrap();

    let result = to_prompt(&[skill_a, skill_b]).unwrap();
    assert_eq!(result.matches("<skill>").count(), 2);
    assert_eq!(result.matches("</skill>").count(), 2);
    assert!(result.contains("skill-a"));
    assert!(result.contains("skill-b"));
}

#[test]
fn special_characters_escaped() {
    let tmp = tempdir().unwrap();
    let skill_dir = tmp.path().join("special-skill");
    fs::create_dir(&skill_dir).unwrap();
    fs::write(
        skill_dir.join("SKILL.md"),
        "---\nname: special-skill\ndescription: Use <foo> & <bar> tags\n---\nBody\n",
    )
    .unwrap();
    let result = to_prompt(&[skill_dir]).unwrap();
    assert!(result.contains("&lt;foo&gt;"));
    assert!(result.contains("&amp;"));
    assert!(result.contains("&lt;bar&gt;"));
    assert!(!result.contains("<foo>"));
    assert!(!result.contains("<bar>"));
}
