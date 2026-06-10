# ubu-schemas

Canonical JSON Schema contracts for UbU Phase 1.

This public repository defines the machine-readable Phase 1 schema surface for UbU:

- JSON Schema Draft 2020-12 schemas under `schemas/`
- Valid and invalid JSON fixtures under `fixtures/`
- Rust-based schema and fixture validation tools
- Node-based TypeScript generation scaffolding
- CI checks for formatting, tests, fixture validation, and generated TypeScript output

Version starts at `0.1.0`. The default branch is `main`.

## Contract Rules

- Schema `$id` values use `https://schemas.ubunow.net/phase1/<category>/<file-name>.schema.json`.
- Cross-file references use absolute `$id` URIs, never relative file paths.
- Schema filenames are lowercase kebab-case and end in `.schema.json`.
- IDs are prefixed UUIDv7 strings with a single `_` delimiter. See `schemas/common/id-registry.schema.json`.
- `AuthoritySource` is a closed enum. See `schemas/common/authority-source.schema.json`.
- Rust types are not generated from this repository.
- TypeScript types are generated from JSON Schema and are ephemeral by default.

## Quick Checks

```sh
scripts/validate-all.sh
```

Generated TypeScript output is written under `generated/typescript/` and ignored except for its README.

