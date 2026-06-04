---
name: pdf-processing
description: Extract text and tables from PDF files, fill PDF forms, and merge multiple PDFs. Use when working with PDF documents or when the user mentions PDFs, forms, or document extraction.
license: Apache-2.0
compatibility: Requires Python 3.11+ and the pypdf package
metadata:
  author: skref-examples
  version: "1.0"
---

# PDF Processing

Procedural knowledge for common PDF tasks.

## Extract text

Use `scripts/extract.py` to pull the text layer from a PDF:

```bash
python scripts/extract.py input.pdf
```

## Fill forms

Map field names to values and write a filled copy. See `references/forms.md`
for the field-mapping conventions.

## Merge

Concatenate several PDFs in order, preserving bookmarks where possible.
