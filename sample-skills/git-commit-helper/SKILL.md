---
name: git-commit-helper
description: Write clear, conventional git commit messages and stage related changes together. Use when the user asks to commit, wants help phrasing a commit message, or mentions conventional commits.
allowed-tools: Bash(git:*) Bash(jq:*)
metadata:
  author: skref-examples
  category: developer-tools
---

# Git Commit Helper

Helps craft well-structured commits.

## Workflow

1. Run `git status` and `git diff --staged` to understand what is changing.
2. Group related changes into a single logical commit.
3. Write a message in the Conventional Commits style:

   ```
   <type>(<scope>): <summary>

   <body explaining what and why>
   ```

   Common types: `feat`, `fix`, `docs`, `refactor`, `test`, `chore`.
4. Keep the summary under 72 characters and in the imperative mood.
