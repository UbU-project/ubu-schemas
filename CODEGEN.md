# Code Generation

TypeScript types are generated from the JSON Schema contracts in `schemas/`.

Rust types are intentionally not generated from these schemas in this repository. The hand-written Rust serde types in downstream crates remain authoritative for runtime implementation details until that boundary is explicitly revised.

## TypeScript

Run:

```sh
scripts/generate-typescript.sh
```

The generator builds an in-memory local `$id` registry from `schemas/` and rewrites absolute `$id` references into a local bundle before invoking the Node JSON Schema to TypeScript toolchain. It must not fetch `https://schemas.ubunow.net` or any other remote schema.

Generated files are ephemeral by default:

- Ignored: `generated/typescript/*`
- Committed: `generated/typescript/README.md`

TODO: decide whether generated TypeScript declarations should become committed release artifacts after Phase 1 consumers stabilize.

