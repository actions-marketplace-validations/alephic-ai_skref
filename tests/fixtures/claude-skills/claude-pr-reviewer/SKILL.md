---
name: claude-pr-reviewer
description: Review a pull request for correctness and style, then summarize the findings. Use when the user asks for a PR review, mentions reviewing a diff, or wants feedback on changes before merging.
allowed-tools: Bash(git:*) Read Grep
when_to_use: When the user asks to review a PR or a diff and wants a concise findings summary.
argument-hint: "[pr-number]"
model: inherit
disable-model-invocation: true
---

# Claude PR Reviewer

A skill that reviews a pull request and reports correctness and style findings.

## Workflow

1. Read the diff with `git diff` and the surrounding context.
2. Flag correctness bugs first, then style and clarity nits.
3. Summarize findings grouped by severity.

It mixes base-spec fields with Claude Code fields, so it validates only when
`--allow-claude-fields` is set.
