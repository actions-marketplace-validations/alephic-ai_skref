//! Options that tune validation and property reading beyond the base spec.

/// Behavior toggles shared by [`validate_with_options`](crate::validate_with_options)
/// and [`read_properties_with_options`](crate::read_properties_with_options).
///
/// The default (all-`false`) keeps `skref` faithful to the Python `skills-ref`
/// base Agent Skills spec.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Options {
    /// Accept Claude Code's extra frontmatter fields during validation and
    /// surface them in `read_properties` output. See
    /// [`CLAUDE_FIELDS`](crate::validator::CLAUDE_FIELDS).
    pub allow_claude_fields: bool,
}
