//! Data models for Agent Skills.

use serde_json::{Map, Value};

use crate::yaml::FmValue;

/// Properties parsed from a skill's `SKILL.md` frontmatter.
///
/// Mirrors the reference `SkillProperties` dataclass.
///
/// * `name` — skill name in kebab-case (required)
/// * `description` — what the skill does and when to use it (required)
/// * `license` — license for the skill (optional)
/// * `compatibility` — compatibility information (optional)
/// * `allowed_tools` — tool patterns the skill requires (optional, experimental)
/// * `metadata` — key-value pairs for client-specific properties. Order is
///   preserved. Omitted from [`to_dict`](SkillProperties::to_dict) when empty.
/// * `claude` — Claude Code's extra frontmatter fields, captured only when
///   reading with [`Options::allow_claude_fields`](crate::Options). Order is
///   the spec order of [`CLAUDE_FIELDS`](crate::validator::CLAUDE_FIELDS).
///   Values keep their YAML structure (lists, nested maps). Empty otherwise.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct SkillProperties {
    pub name: String,
    pub description: String,
    pub license: Option<String>,
    pub compatibility: Option<String>,
    pub allowed_tools: Option<String>,
    pub metadata: Vec<(String, String)>,
    pub claude: Vec<(String, FmValue)>,
}

/// Convert an [`FmValue`] to JSON, preserving list/map structure. Scalars are
/// already strings (the strictyaml-style model), so booleans render as
/// `"True"`/`"False"` consistently with the rest of the library.
fn fmvalue_to_json(value: &FmValue) -> Value {
    match value {
        FmValue::Str(s) => Value::String(s.clone()),
        FmValue::Seq(items) => Value::Array(items.iter().map(fmvalue_to_json).collect()),
        FmValue::Map(entries) => {
            let mut map = Map::new();
            for (k, v) in entries {
                map.insert(k.clone(), fmvalue_to_json(v));
            }
            Value::Object(map)
        }
    }
}

impl SkillProperties {
    /// Convert to a JSON object, excluding `None`/empty values.
    ///
    /// Field order matches the reference implementation: `name`,
    /// `description`, then any of `license`, `compatibility`,
    /// `allowed-tools`, `metadata` that are present. Note that
    /// `allowed_tools` is emitted under the hyphenated key `allowed-tools`.
    /// Any captured `claude` fields follow, each under its own frontmatter key.
    pub fn to_dict(&self) -> Value {
        let mut result = Map::new();
        result.insert("name".into(), Value::String(self.name.clone()));
        result.insert(
            "description".into(),
            Value::String(self.description.clone()),
        );

        if let Some(license) = &self.license {
            result.insert("license".into(), Value::String(license.clone()));
        }
        if let Some(compatibility) = &self.compatibility {
            result.insert("compatibility".into(), Value::String(compatibility.clone()));
        }
        if let Some(allowed_tools) = &self.allowed_tools {
            result.insert("allowed-tools".into(), Value::String(allowed_tools.clone()));
        }
        if !self.metadata.is_empty() {
            let mut meta = Map::new();
            for (k, v) in &self.metadata {
                meta.insert(k.clone(), Value::String(v.clone()));
            }
            result.insert("metadata".into(), Value::Object(meta));
        }
        for (k, v) in &self.claude {
            result.insert(k.clone(), fmvalue_to_json(v));
        }

        Value::Object(result)
    }

    /// Serialize [`to_dict`](SkillProperties::to_dict) as pretty JSON
    /// (2-space indent), matching `json.dumps(..., indent=2)`.
    pub fn to_json(&self) -> String {
        serde_json::to_string_pretty(&self.to_dict()).expect("SkillProperties serializes")
    }
}
