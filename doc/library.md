# Using `skref` as a library

`skref` is also a Rust crate. The same logic the CLI uses is exposed as a small
public API — `validate`, `read_properties`, and `to_prompt` — re-exported from
[`src/lib.rs`](../src/lib.rs).

```rust
use std::path::Path;
use skref::{validate, read_properties, to_prompt};

let problems = validate(Path::new("my-skill"));
if problems.is_empty() {
    let props = read_properties(Path::new("my-skill"))?;
    println!("{} — {}", props.name, props.description);
}

let prompt = to_prompt(&[Path::new("skill-a"), Path::new("skill-b")])?;
println!("{prompt}");
# Ok::<(), skref::SkillError>(())
```

For the full API reference, see [docs.rs/skref](https://docs.rs/skref).
