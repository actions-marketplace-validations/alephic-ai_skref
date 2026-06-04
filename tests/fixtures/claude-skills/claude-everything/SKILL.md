---
name: claude-everything
description: A reference skill that exercises every supported frontmatter field — the six base Agent Skills fields plus all thirteen Claude Code extensions. Use when testing skref's --allow-claude-fields support or as a worked example of the full Claude frontmatter surface.
license: Apache-2.0
compatibility: Requires Claude Code 2.x
allowed-tools: Read Grep Bash(git:*)
metadata:
  author: skref-examples
  version: "1.0"
when_to_use: When you need a fixture listing every Claude Code frontmatter field at once.
argument-hint: "[issue-number] [format]"
arguments: issue format
disable-model-invocation: true
user-invocable: true
disallowed-tools:
  - Bash(rm:*)
  - WebFetch
model: inherit
effort: high
context: fork
agent: general-purpose
hooks:
  PreToolUse: echo pre
  PostToolUse: echo post
paths:
  - "src/**/*.rs"
  - "docs/**/*.md"
shell: bash
---

# Claude Everything

A test fixture whose frontmatter uses every frontmatter field skref supports: the six
base Agent Skills fields and all thirteen Claude Code extensions.

It validates only when `--allow-claude-fields` is set, and is rejected under the base
Agent Skills spec.
