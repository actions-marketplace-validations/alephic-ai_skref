# Security Policy

## Supported versions

Security fixes are applied to the latest released version. Please make sure you
are on the most recent release before reporting an issue.

## Reporting a vulnerability

Please **do not** open a public GitHub issue for security vulnerabilities.

Instead, report them privately using GitHub's
[private vulnerability reporting](https://github.com/alephic-ai/skref/security/advisories/new)
("Report a vulnerability" under the repository's **Security** tab).

Please include:

- A description of the vulnerability and its impact.
- Steps to reproduce, ideally with a minimal `SKILL.md` or input.
- The `skref` version and your platform.

## What to expect

- We aim to acknowledge reports within **3 business days**.
- We will keep you informed about the progress of a fix and coordinate
  disclosure timing with you.
- With your permission, we are happy to credit you in the release notes once a
  fix ships.

## Scope

`skref` reads and validates local `SKILL.md` files and emits text. It performs
no network access and executes no skill code itself. The most relevant concerns
are therefore around parsing untrusted input (malformed frontmatter, adversarial
Unicode, resource exhaustion). Reports demonstrating crashes, hangs, or
incorrect validation results on untrusted input are in scope and welcome.
