# clin-rs compatibility branch

Branch `fix/maintained-yaml-msrv-1.90` is a minimal compatibility fork of
`Latias94/merman` at `91722fd1881ae37fd276788fe78a5cc7274d4cde` (the published
`merman-core 0.6.0` source). Package versions, Mermaid baseline, public APIs,
licenses and declared Rust 1.87 minimum remain unchanged. clin-rs consumes this
branch through an immutable Git revision, not a moving branch reference.

## YAML backend

The workspace's `serde_yaml` import name now aliases YAML Organization's
`yaml_serde = 0.10.7`. Its backend is maintained `libyaml-rs 0.3.0`, replacing
archived `serde_yaml` / `unsafe-libyaml`. This is not a pure-safe-Rust parser:
`libyaml-rs` still contains translated libyaml unsafe code. The change resolves
unmaintained dependency ownership; it does not claim elimination of every
possible parser vulnerability. No advisory is ignored and no Mermaid feature
is removed.

The alias keeps callers and data types unchanged. New tests were first run
against the original backend, then rerun after replacement. They cover tags,
complex YAML keys, aliases, numeric precision, quoted values, dates, multiline
strings, CRLF frontmatter, malformed/recursive aliases, and document streaming.
The existing semantic snapshot corpus remains unchanged.

## Proof

Run from the workspace with Rust 1.90:

```sh
cargo +1.90.0 test --locked -p merman-core
cargo +1.90.0 check --locked -p xtask
```

Verified: 525 core unit tests, complete semantic snapshot test, 3 YAML
compatibility tests, 2 doctests, and xtask compilation. These are parser/core
checks, not a claim that the entire upstream SVG DOM or benchmark suite ran.
clin-rs additionally runs its full tests, MSRV and unsuppressed cargo-deny gates
against this revision, including native Mermaid previews.

Prefer upstream adoption of the maintained backend. Until an MSRV-compatible
upstream release includes it, retain the pinned fork and these contract tests.
Do not update to an upstream version requiring Rust 1.95 merely to bypass the
compatibility requirement.
