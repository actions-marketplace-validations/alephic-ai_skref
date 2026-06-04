# PDF form-filling conventions

Bundled reference material for the `pdf-processing` skill. Loaded on demand
when the agent needs the details, keeping the main `SKILL.md` small.

## Field mapping

Provide a JSON object mapping each PDF form field name to its value:

```json
{
  "full_name": "Ada Lovelace",
  "date": "1843-10-01",
  "agree": true
}
```

- Text fields take strings.
- Checkboxes take booleans (`true` = checked).
- Unknown field names are ignored with a warning.

## Flattening

After filling, optionally flatten the form so values are no longer editable.
