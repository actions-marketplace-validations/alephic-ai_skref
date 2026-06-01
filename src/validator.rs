//! Skill validation logic.

use std::path::Path;

use unicode_normalization::UnicodeNormalization;

use crate::parser::{find_skill_md, parse_frontmatter};
use crate::yaml::{FmValue, Frontmatter};

pub const MAX_SKILL_NAME_LENGTH: usize = 64;
pub const MAX_DESCRIPTION_LENGTH: usize = 1024;
pub const MAX_COMPATIBILITY_LENGTH: usize = 500;

/// Allowed frontmatter fields per the Agent Skills spec.
pub const ALLOWED_FIELDS: [&str; 6] = [
    "name",
    "description",
    "license",
    "allowed-tools",
    "metadata",
    "compatibility",
];

/// NFKC-normalize a string (matches Python's `unicodedata.normalize("NFKC", ...)`).
fn nfkc(s: &str) -> String {
    s.nfkc().collect()
}

/// Number of Unicode scalar values, matching Python's `len(str)`.
fn char_len(s: &str) -> usize {
    s.chars().count()
}

/// Validate skill name format and directory match.
///
/// Skill names support i18n characters (Unicode letters) plus hyphens. Names
/// must be lowercase and cannot start or end with a hyphen.
fn validate_name(name: &FmValue, skill_dir: Option<&Path>) -> Vec<String> {
    let mut errors = Vec::new();

    let raw = match name.as_str() {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            errors.push("Field 'name' must be a non-empty string".to_string());
            return errors;
        }
    };

    let name = nfkc(raw.trim());

    if char_len(&name) > MAX_SKILL_NAME_LENGTH {
        errors.push(format!(
            "Skill name '{name}' exceeds {MAX_SKILL_NAME_LENGTH} character limit ({} chars)",
            char_len(&name)
        ));
    }

    if name != name.to_lowercase() {
        errors.push(format!("Skill name '{name}' must be lowercase"));
    }

    if name.starts_with('-') || name.ends_with('-') {
        errors.push("Skill name cannot start or end with a hyphen".to_string());
    }

    if name.contains("--") {
        errors.push("Skill name cannot contain consecutive hyphens".to_string());
    }

    if !name.chars().all(|c| c.is_alphanumeric() || c == '-') {
        errors.push(format!(
            "Skill name '{name}' contains invalid characters. \
             Only letters, digits, and hyphens are allowed."
        ));
    }

    if let Some(dir) = skill_dir {
        let dir_name_raw = dir
            .file_name()
            .map(|n| n.to_string_lossy().into_owned())
            .unwrap_or_default();
        let dir_name = nfkc(&dir_name_raw);
        if dir_name != name {
            errors.push(format!(
                "Directory name '{dir_name_raw}' must match skill name '{name}'"
            ));
        }
    }

    errors
}

/// Validate description format.
fn validate_description(description: &FmValue) -> Vec<String> {
    let mut errors = Vec::new();

    let desc = match description.as_str() {
        Some(s) if !s.trim().is_empty() => s,
        _ => {
            errors.push("Field 'description' must be a non-empty string".to_string());
            return errors;
        }
    };

    if char_len(desc) > MAX_DESCRIPTION_LENGTH {
        errors.push(format!(
            "Description exceeds {MAX_DESCRIPTION_LENGTH} character limit ({} chars)",
            char_len(desc)
        ));
    }

    errors
}

/// Validate compatibility format.
fn validate_compatibility(compatibility: &FmValue) -> Vec<String> {
    let mut errors = Vec::new();

    let compat = match compatibility.as_str() {
        Some(s) => s,
        None => {
            errors.push("Field 'compatibility' must be a string".to_string());
            return errors;
        }
    };

    if char_len(compat) > MAX_COMPATIBILITY_LENGTH {
        errors.push(format!(
            "Compatibility exceeds {MAX_COMPATIBILITY_LENGTH} character limit ({} chars)",
            char_len(compat)
        ));
    }

    errors
}

/// Validate that only allowed fields are present.
fn validate_metadata_fields(metadata: &Frontmatter) -> Vec<String> {
    let mut errors = Vec::new();

    let allowed: std::collections::BTreeSet<String> =
        ALLOWED_FIELDS.iter().map(|s| s.to_string()).collect();
    let extra: Vec<String> = metadata.keys().difference(&allowed).cloned().collect();

    if !extra.is_empty() {
        // `extra` is already sorted (BTreeSet difference yields sorted order).
        let allowed_repr: Vec<String> = {
            let mut v: Vec<&str> = ALLOWED_FIELDS.to_vec();
            v.sort_unstable();
            v.iter().map(|s| format!("'{s}'")).collect()
        };
        errors.push(format!(
            "Unexpected fields in frontmatter: {}. Only [{}] are allowed.",
            extra.join(", "),
            allowed_repr.join(", ")
        ));
    }

    errors
}

/// Validate parsed skill metadata.
///
/// This is the core validation routine that works on already-parsed
/// frontmatter, avoiding duplicate file I/O. Returns the list of validation
/// error messages; an empty list means valid.
pub fn validate_metadata(metadata: &Frontmatter, skill_dir: Option<&Path>) -> Vec<String> {
    let mut errors = Vec::new();
    errors.extend(validate_metadata_fields(metadata));

    match metadata.get("name") {
        None => errors.push("Missing required field in frontmatter: name".to_string()),
        Some(name) => errors.extend(validate_name(name, skill_dir)),
    }

    match metadata.get("description") {
        None => errors.push("Missing required field in frontmatter: description".to_string()),
        Some(description) => errors.extend(validate_description(description)),
    }

    if let Some(compatibility) = metadata.get("compatibility") {
        errors.extend(validate_compatibility(compatibility));
    }

    errors
}

/// Validate a skill directory.
///
/// Returns the list of validation error messages; an empty list means the
/// skill is valid.
pub fn validate(skill_dir: &Path) -> Vec<String> {
    if !skill_dir.exists() {
        return vec![format!("Path does not exist: {}", skill_dir.display())];
    }

    if !skill_dir.is_dir() {
        return vec![format!("Not a directory: {}", skill_dir.display())];
    }

    let skill_md = match find_skill_md(skill_dir) {
        Some(p) => p,
        None => return vec!["Missing required file: SKILL.md".to_string()],
    };

    let content = match std::fs::read_to_string(&skill_md) {
        Ok(c) => c,
        Err(e) => return vec![format!("Could not read {}: {e}", skill_md.display())],
    };

    match parse_frontmatter(&content) {
        Ok((metadata, _body)) => validate_metadata(&metadata, Some(skill_dir)),
        Err(e) => vec![e.to_string()],
    }
}
