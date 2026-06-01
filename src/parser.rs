//! YAML frontmatter parsing for `SKILL.md` files.

use std::path::{Path, PathBuf};

use crate::errors::{Result, SkillError};
use crate::models::SkillProperties;
use crate::yaml::{FmValue, Frontmatter, parse_mapping};

/// Find the `SKILL.md` file in a skill directory.
///
/// Prefers `SKILL.md` (uppercase) but accepts `skill.md` (lowercase).
/// Returns `None` if neither exists.
pub fn find_skill_md(skill_dir: &Path) -> Option<PathBuf> {
    for name in ["SKILL.md", "skill.md"] {
        let path = skill_dir.join(name);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

/// Parse YAML frontmatter from `SKILL.md` content.
///
/// Returns the parsed frontmatter mapping and the trimmed Markdown body.
///
/// # Errors
///
/// Returns [`SkillError::Parse`] if the frontmatter is missing, not closed,
/// invalid YAML, or not a mapping.
pub fn parse_frontmatter(content: &str) -> Result<(Frontmatter, String)> {
    if !content.starts_with("---") {
        return Err(SkillError::parse(
            "SKILL.md must start with YAML frontmatter (---)",
        ));
    }

    // Equivalent to Python's content.split("---", 2): at most three pieces.
    let parts: Vec<&str> = content.splitn(3, "---").collect();
    if parts.len() < 3 {
        return Err(SkillError::parse(
            "SKILL.md frontmatter not properly closed with ---",
        ));
    }

    let frontmatter_str = parts[1];
    let body = parts[2].trim().to_string();

    match parse_mapping(frontmatter_str) {
        Ok(Some(frontmatter)) => Ok((frontmatter, body)),
        Ok(None) => Err(SkillError::parse(
            "SKILL.md frontmatter must be a YAML mapping",
        )),
        Err(e) => Err(SkillError::parse(format!(
            "Invalid YAML in frontmatter: {e}"
        ))),
    }
}

/// Read skill properties from `SKILL.md` frontmatter.
///
/// Parses the frontmatter and returns properties. This does **not** perform
/// full validation — use [`crate::validator::validate`] for that.
///
/// # Errors
///
/// * [`SkillError::Parse`] if `SKILL.md` is missing or has invalid YAML.
/// * [`SkillError::Validation`] if required fields (`name`, `description`) are
///   missing or not non-empty strings.
pub fn read_properties(skill_dir: &Path) -> Result<SkillProperties> {
    let skill_md = find_skill_md(skill_dir).ok_or_else(|| {
        SkillError::parse(format!("SKILL.md not found in {}", skill_dir.display()))
    })?;

    let content = std::fs::read_to_string(&skill_md)
        .map_err(|e| SkillError::parse(format!("Could not read {}: {e}", skill_md.display())))?;
    let (metadata, _body) = parse_frontmatter(&content)?;

    if !metadata.contains_key("name") {
        return Err(SkillError::validation(
            "Missing required field in frontmatter: name",
        ));
    }
    if !metadata.contains_key("description") {
        return Err(SkillError::validation(
            "Missing required field in frontmatter: description",
        ));
    }

    let name = match metadata.get("name").and_then(FmValue::as_str) {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            return Err(SkillError::validation(
                "Field 'name' must be a non-empty string",
            ));
        }
    };
    let description = match metadata.get("description").and_then(FmValue::as_str) {
        Some(s) if !s.trim().is_empty() => s.trim().to_string(),
        _ => {
            return Err(SkillError::validation(
                "Field 'description' must be a non-empty string",
            ));
        }
    };

    let license = metadata
        .get("license")
        .and_then(FmValue::as_str)
        .map(str::to_string);
    let compatibility = metadata
        .get("compatibility")
        .and_then(FmValue::as_str)
        .map(str::to_string);
    let allowed_tools = metadata
        .get("allowed-tools")
        .and_then(FmValue::as_str)
        .map(str::to_string);

    let mut props = SkillProperties {
        name,
        description,
        license,
        compatibility,
        allowed_tools,
        metadata: Vec::new(),
    };

    if let Some(FmValue::Map(entries)) = metadata.get("metadata") {
        props.metadata = entries
            .iter()
            .map(|(k, v)| (k.clone(), v.to_flat_string()))
            .collect();
    }

    Ok(props)
}
