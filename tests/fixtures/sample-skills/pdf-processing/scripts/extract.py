#!/usr/bin/env python3
"""Extract the text layer from a PDF file and print it to stdout.

Example bundled script for the `pdf-processing` sample skill. Demonstrates how
a skill can ship executable helpers alongside its SKILL.md instructions.
"""

import sys


def main() -> int:
    if len(sys.argv) != 2:
        print("usage: extract.py <input.pdf>", file=sys.stderr)
        return 2

    try:
        from pypdf import PdfReader
    except ImportError:
        print("pypdf is required: pip install pypdf", file=sys.stderr)
        return 1

    reader = PdfReader(sys.argv[1])
    for page in reader.pages:
        print(page.extract_text() or "")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
