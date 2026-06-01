//! Generate the `<available_skills>` XML prompt block for agent system prompts.

use std::path::Path;

use crate::errors::Result;
use crate::parser::{find_skill_md, read_properties};

/// Escape XML/HTML special characters, matching Python's `html.escape` with
/// its default `quote=True` (escapes `&`, `<`, `>`, `"`, and `'`).
///
/// `&` must be replaced first so the ampersands introduced by the other
/// replacements are not double-escaped.
fn html_escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#x27;")
}

/// Generate the `<available_skills>` XML block for inclusion in agent prompts.
///
/// This is the format Anthropic uses and recommends for Claude models. Skill
/// Clients may format skill information differently to suit their models.
///
/// Each skill contributes a `<skill>` element with its `name`, `description`,
/// and the absolute `location` of its `SKILL.md` file. The name and
/// description are HTML-escaped.
///
/// # Errors
///
/// Propagates any [`crate::errors::SkillError`] from reading a skill's
/// properties.
pub fn to_prompt(skill_dirs: &[impl AsRef<Path>]) -> Result<String> {
    if skill_dirs.is_empty() {
        return Ok("<available_skills>\n</available_skills>".to_string());
    }

    let mut lines = vec!["<available_skills>".to_string()];

    for skill_dir in skill_dirs {
        // Resolve to an absolute path so <location> is unambiguous. Fall back
        // to the path as given if it cannot be canonicalized.
        let dir = std::fs::canonicalize(skill_dir.as_ref())
            .unwrap_or_else(|_| skill_dir.as_ref().to_path_buf());
        let props = read_properties(&dir)?;

        lines.push("<skill>".to_string());
        lines.push("<name>".to_string());
        lines.push(html_escape(&props.name));
        lines.push("</name>".to_string());
        lines.push("<description>".to_string());
        lines.push(html_escape(&props.description));
        lines.push("</description>".to_string());

        let skill_md_path = find_skill_md(&dir);
        lines.push("<location>".to_string());
        lines.push(match skill_md_path {
            Some(p) => p.display().to_string(),
            None => "None".to_string(),
        });
        lines.push("</location>".to_string());

        lines.push("</skill>".to_string());
    }

    lines.push("</available_skills>".to_string());

    Ok(lines.join("\n"))
}
