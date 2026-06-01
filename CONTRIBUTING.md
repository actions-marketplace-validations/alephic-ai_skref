# Contributing to skref

Thanks for your interest in improving `skref`! This document covers how to get
set up, the quality bar for changes, and a few conventions specific to this
project.

## Getting started

`skref` is a standard Cargo project. You need a recent stable Rust toolchain
(edition 2024, Rust 1.85+):

```bash
rustup update stable
git clone https://github.com/alephic-ai/skref
cd skref
cargo build
```

## Quality bar

Every change must pass the same checks CI runs. Please run them locally before
opening a pull request:

```bash
cargo fmt --all
cargo clippy --all-targets -- -D warnings
cargo test --all
```

- **Formatting** is enforced (`cargo fmt --all -- --check` in CI).
- **Clippy** runs with `-D warnings` — warnings fail the build.
- **Tests** must pass, including the doctests in `src/lib.rs`.

## Fidelity to the reference implementation

`skref` is a Rust port of the Python
[`skills-ref`](https://github.com/agentskills/agentskills/tree/main/skills-ref)
library and tracks the
[Agent Skills specification](https://github.com/agentskills/agentskills/blob/main/docs/specification.mdx).
**Behavior should stay faithful to both.** When you change validation or parsing
logic, make sure it still matches the reference semantics — see the "Fidelity
notes" in [`AGENTS.md`](AGENTS.md) for the subtle cases (frontmatter splitting,
NFKC normalization, HTML escaping, JSON output formatting).

The tests in `tests/parser.rs`, `tests/prompt.rs`, and `tests/validator.rs` are
1:1 ports of the Python test suite. If you change observable behavior, update
both the implementation and the corresponding tests, and explain in your PR why
the divergence from the reference is intended.

## Pull requests

- Keep PRs focused; one logical change per PR is easiest to review.
- Add or update tests for any behavior change.
- Write a clear description of *what* changed and *why*.

## Reporting bugs and requesting features

Open an issue at https://github.com/alephic-ai/skref/issues. For bugs, please
include the `skref` version, your platform, and a minimal `SKILL.md` that
reproduces the problem.

## License

By contributing, you agree that your contributions will be licensed under the
[Apache-2.0](LICENSE) license that covers this project.
