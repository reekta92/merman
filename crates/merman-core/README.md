# merman-core-clin

Clin's compatibility fork of upstream `merman-core 0.6.0`, based on
`91722fd1881ae37fd276788fe78a5cc7274d4cde`. It preserves that parser API and
semantic snapshot corpus while replacing archived `serde_yaml` with maintained
`yaml_serde 0.10.7`. Rust 1.90 is supported. Upstream MIT/Apache-2.0 licensing
and attribution are retained.

```toml
merman-core = { package = "merman-core-clin", version = "=0.6.0", default-features = false }
```

This registry dependency carries the maintained backend to consumers without a
root Cargo patch. The library import remains `merman_core`. Maintenance is scoped
to this compatibility package on `fix/maintained-yaml-msrv-1.90`; the fork's `main`
tracks upstream. Retire this package when the host can adopt a maintained upstream
release within its supported compiler and API requirements.

`merman-core` is the parser and semantic-model crate behind [merman](https://crates.io/crates/merman). Use it when you need Mermaid detection, metadata, semantic JSON, or typed render models without pulling in layout, SVG, or raster dependencies.

Most application code that wants rendered output should use the `merman` crate with the `render` feature instead.

## What It Provides

- Mermaid diagram detection and preprocessing, including front matter and directives.
- Strict and lenient parsing through `ParseOptions`.
- Semantic JSON via `Engine::parse_diagram_sync`.
- Typed render models via `Engine::parse_diagram_for_render_model_sync`.
- Metadata-only parsing for integrations that only need the diagram type, title, and effective config.
- Runtime-agnostic async APIs plus synchronous helpers for editor and CLI integrations.

## Parse To Semantic JSON

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_sync("flowchart TD; A[API] --> B[DB];", ParseOptions::strict())?
        .expect("diagram detected");

    assert_eq!(parsed.meta.diagram_type, "flowchart-v2");
    println!("{}", parsed.model);

    Ok(())
}
```

## Skip Detection When The Type Is Known

Markdown renderers often know the diagram type from the fence info string. Use the `*_as_sync` APIs to skip the detection pass.

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_as_sync(
            "sequence",
            "sequenceDiagram\nAlice->>Bob: Hello",
            ParseOptions::strict(),
        )?
        .expect("diagram detected");

    assert_eq!(parsed.meta.diagram_type, "sequence");
    Ok(())
}
```

Common internal ids include `flowchart-v2`, `sequence`, `classDiagram`, `stateDiagram`, `architecture`, `mindmap`, and `gantt`.

## Rendering Handoff

If the next step is layout or SVG rendering, prefer `Engine::parse_diagram_for_render_model_sync`. It returns a render-optimized typed model and avoids building a large public semantic JSON tree for diagrams with typed render support.

```rust
use merman_core::{Engine, ParseOptions};

fn main() -> Result<(), merman_core::Error> {
    let engine = Engine::new();
    let parsed = engine
        .parse_diagram_for_render_model_sync("flowchart TD; A --> B", ParseOptions::strict())?
        .expect("diagram detected");

    println!("{} -> {}", parsed.meta.diagram_type, parsed.model.kind());
    Ok(())
}
```

## Compatibility

`merman-core` tracks Mermaid 11.12.x behavior and treats upstream Mermaid as the compatibility target. The semantic JSON API is the stable parser-facing shape; typed render models are optimized for the renderer and may expose a different internal structure.
