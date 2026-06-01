//! Minimal YAML frontmatter value model.
//!
//! The reference library parses frontmatter with `strictyaml`, whose default
//! schema treats every scalar as a string. We reproduce that behaviour here:
//! after parsing with `serde_yaml`, every scalar leaf is coerced to its
//! Python-`str()`-equivalent string form, while mappings and sequences keep
//! their structure. Top-level frontmatter is always a mapping.

use std::collections::BTreeSet;

/// A frontmatter value, normalized so that all scalars are strings.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FmValue {
    Str(String),
    Seq(Vec<FmValue>),
    Map(Vec<(String, FmValue)>),
}

impl FmValue {
    /// Return the string if this is a scalar, else `None`.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            FmValue::Str(s) => Some(s),
            _ => None,
        }
    }

    /// Flatten any value to a string, mirroring Python's `str(value)` closely
    /// enough for skill metadata (scalars are exact; containers fall back to a
    /// readable rendering).
    pub fn to_flat_string(&self) -> String {
        match self {
            FmValue::Str(s) => s.clone(),
            FmValue::Seq(items) => {
                let inner: Vec<String> = items
                    .iter()
                    .map(|i| format!("'{}'", i.to_flat_string()))
                    .collect();
                format!("[{}]", inner.join(", "))
            }
            FmValue::Map(entries) => {
                let inner: Vec<String> = entries
                    .iter()
                    .map(|(k, v)| format!("'{}': '{}'", k, v.to_flat_string()))
                    .collect();
                format!("{{{}}}", inner.join(", "))
            }
        }
    }
}

/// Top-level frontmatter: an ordered mapping of string keys to values.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Frontmatter {
    entries: Vec<(String, FmValue)>,
}

impl Frontmatter {
    /// Look up a field by key.
    pub fn get(&self, key: &str) -> Option<&FmValue> {
        self.entries.iter().find(|(k, _)| k == key).map(|(_, v)| v)
    }

    /// Whether a field is present.
    pub fn contains_key(&self, key: &str) -> bool {
        self.entries.iter().any(|(k, _)| k == key)
    }

    /// The set of top-level field names.
    pub fn keys(&self) -> BTreeSet<String> {
        self.entries.iter().map(|(k, _)| k.clone()).collect()
    }
}

/// Convert a `serde_yaml` scalar to its Python `str()`-equivalent string.
fn scalar_to_string(value: &serde_yaml::Value) -> Option<String> {
    use serde_yaml::Value;
    match value {
        Value::Null => Some("None".to_string()),
        Value::Bool(b) => Some(if *b {
            "True".to_string()
        } else {
            "False".to_string()
        }),
        Value::String(s) => Some(s.clone()),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                Some(i.to_string())
            } else if let Some(u) = n.as_u64() {
                Some(u.to_string())
            } else {
                // Rust's Debug formatting renders 1.0 as "1.0", matching
                // Python's str(1.0); Display would render "1".
                n.as_f64().map(|f| format!("{f:?}"))
            }
        }
        _ => None,
    }
}

/// Recursively convert a parsed `serde_yaml::Value` into an [`FmValue`],
/// coercing every scalar to a string.
fn convert(value: &serde_yaml::Value) -> FmValue {
    use serde_yaml::Value;
    match value {
        Value::Sequence(items) => FmValue::Seq(items.iter().map(convert).collect()),
        Value::Mapping(map) => {
            let mut entries = Vec::with_capacity(map.len());
            for (k, v) in map {
                // Non-scalar keys are rare; render them flatly.
                let key = scalar_to_string(k).unwrap_or_else(|| convert(k).to_flat_string());
                entries.push((key, convert(v)));
            }
            FmValue::Map(entries)
        }
        scalar => FmValue::Str(scalar_to_string(scalar).unwrap_or_default()),
    }
}

/// Parse a YAML frontmatter string into a [`Frontmatter`] mapping.
///
/// Returns `Ok(None)` when the document is valid YAML but not a mapping (the
/// caller turns that into the "must be a YAML mapping" parse error). Returns
/// `Err` when the YAML itself is malformed.
pub fn parse_mapping(frontmatter: &str) -> std::result::Result<Option<Frontmatter>, String> {
    let value: serde_yaml::Value = serde_yaml::from_str(frontmatter).map_err(|e| e.to_string())?;

    match convert(&value) {
        FmValue::Map(entries) => Ok(Some(Frontmatter { entries })),
        _ => Ok(None),
    }
}
