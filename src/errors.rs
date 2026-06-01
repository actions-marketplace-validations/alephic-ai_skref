//! Skill-related error types.
//!
//! Mirrors the exception hierarchy of the Python `skills-ref` reference
//! library: a single [`SkillError`] enum stands in for the `SkillError`
//! base class and its `ParseError` / `ValidationError` subclasses.

use std::fmt;

/// Base error for all skill-related failures.
///
/// The two variants correspond to the Python `ParseError` and
/// `ValidationError` subclasses of `SkillError`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SkillError {
    /// Raised when `SKILL.md` parsing fails (missing file, malformed
    /// frontmatter, invalid YAML, etc.).
    Parse(String),

    /// Raised when skill properties are invalid.
    ///
    /// Carries one or more validation messages. When constructed from a
    /// single message, `errors` contains exactly that message — matching the
    /// behaviour of the reference `ValidationError(message)`.
    Validation {
        message: String,
        errors: Vec<String>,
    },
}

impl SkillError {
    /// Construct a [`SkillError::Parse`].
    pub fn parse(message: impl Into<String>) -> Self {
        SkillError::Parse(message.into())
    }

    /// Construct a [`SkillError::Validation`] from a single message.
    ///
    /// `errors` is seeded with the same message, matching the reference
    /// `ValidationError.__init__` default.
    pub fn validation(message: impl Into<String>) -> Self {
        let message = message.into();
        SkillError::Validation {
            errors: vec![message.clone()],
            message,
        }
    }

    /// Construct a [`SkillError::Validation`] carrying an explicit list of
    /// error messages. The `message` is the first error (or empty).
    pub fn validation_many(errors: Vec<String>) -> Self {
        let message = errors.first().cloned().unwrap_or_default();
        SkillError::Validation { message, errors }
    }

    /// The list of underlying error messages.
    pub fn errors(&self) -> Vec<String> {
        match self {
            SkillError::Parse(m) => vec![m.clone()],
            SkillError::Validation { errors, .. } => errors.clone(),
        }
    }
}

impl fmt::Display for SkillError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SkillError::Parse(m) => write!(f, "{m}"),
            SkillError::Validation { message, .. } => write!(f, "{message}"),
        }
    }
}

impl std::error::Error for SkillError {}

/// Convenience alias used throughout the crate.
pub type Result<T> = std::result::Result<T, SkillError>;
