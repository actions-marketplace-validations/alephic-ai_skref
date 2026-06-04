//! Reference library for Agent Skills.
//!
//! `skref` is a Rust port of the Python [`skills-ref`] reference library. It
//! provides three operations over skill directories that each contain a
//! `SKILL.md` file:
//!
//! * [`validate`] — check a skill directory for correctness.
//! * [`read_properties`] — parse a skill's frontmatter into [`SkillProperties`].
//! * [`to_prompt`] — render the `<available_skills>` XML block for agent prompts.
//!
//! ```no_run
//! use std::path::Path;
//! use skref::{validate, read_properties, to_prompt};
//!
//! let problems = validate(Path::new("my-skill"));
//! if problems.is_empty() {
//!     let props = read_properties(Path::new("my-skill")).unwrap();
//!     println!("Skill: {} - {}", props.name, props.description);
//! }
//!
//! let prompt = to_prompt(&[Path::new("skill-a"), Path::new("skill-b")]).unwrap();
//! println!("{prompt}");
//! ```
//!
//! [`skills-ref`]: https://github.com/agentskills/agentskills/tree/main/skills-ref

pub mod errors;
pub mod models;
pub mod options;
pub mod parser;
pub mod prompt;
pub mod validator;
pub mod yaml;

pub use errors::{Result, SkillError};
pub use models::SkillProperties;
pub use options::Options;
pub use parser::{find_skill_md, parse_frontmatter, read_properties, read_properties_with_options};
pub use prompt::to_prompt;
pub use validator::{
    CLAUDE_FIELDS, validate, validate_metadata, validate_metadata_with_options,
    validate_with_options,
};
