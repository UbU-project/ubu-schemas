# Contract

This repository is the canonical public contract for UbU Phase 1 JSON documents.

## Schema Standard

All schemas use JSON Schema Draft 2020-12.

Every schema must include:

- `$schema: https://json-schema.org/draft/2020-12/schema`
- `$id: https://schemas.ubunow.net/phase1/<category>/<file-name>.schema.json`
- `title`
- `type`
- `description`

Cross-file `$ref` values must use absolute `$id` URIs.

## Wire Field Naming

Per `UBU-D0228`, Phase 1 UbU wire fields are snake_case. Generated TypeScript follows those wire names rather than converting them to client-side casing. This convention is enforced by schema validation and generated-output checks.

## IDs

UbU IDs are prefixed strings with one `_` delimiter. The suffix is a lowercase, unhyphenated UUIDv7 rendered as 32 hex characters.

Pattern form:

```text
^[a-z]+_[0-9a-f]{12}7[0-9a-f]{3}[89ab][0-9a-f]{15}$
```

The canonical prefix mapping is defined in `schemas/common/id-registry.schema.json`. Per `UBU-D0229`, Phase 1 includes canonical IDs for Preference, Container, UniverseState, Identity, Relationship, and ExternalEvent in addition to the previously registered object types.

## UniverseState

Per `UBU-D0229`, `schemas/core/universe-state.schema.json` is the canonical Phase 1 UniverseState facts object for planning or API exchange. It is not a snapshot-view placeholder; snapshot views remain represented by `schemas/core/snapshot.schema.json`.

From P1B-59 three schemas describe a measured number as a first-class fact, and they change together:

- `schemas/core/universe-state-mutation.schema.json` has nine operations. `set_numeric` replaces a number and `clear_numeric` removes it, beside `set_fact` and `clear_fact`. `payload` is required for every operation except the two clears, which refuse one. The payload of `set_numeric` must be a number. The schema types no other payload; `ubu-core` enforces the rest.
- A mutation may carry `provenance_kind`: `asserted`, `measured`, `derived` or `proposed`. Absent means `asserted`. The two clears refuse it, because they write nothing for it to describe.
- A mutation has no `note`. The field was accepted and stored nowhere, and it is removed.
- `schemas/core/universe-state.schema.json` has an optional `fact_provenance` map beside the four value maps. Its keys are full targets, a collection name and then the dotted key, in the spelling a mutation's `target` uses. Each value has exactly two fields, `kind` and `recorded_at`. There is no confidence number and no free text. `fact_provenance` is not required, so a UniverseState with none stays valid.
- The map is `fact_provenance` and not `provenance`. A UniverseState in the store carries the ordinary object envelope under `provenance`, as every stored object does, and `ubu-store` rewrites that key on each write. Two things cannot share one key.
- `schemas/core/precondition.schema.json` has four numeric comparisons: `at_least`, `at_most`, `greater_than` and `less_than`. Each requires a `numeric_values` target and a number as `expected`.

The four provenance kinds are spelled out in two schemas, the mutation and the UniverseState. A change to one is a change to both.

## Authority Source

`AuthoritySource` is a closed enum in `schemas/common/authority-source.schema.json`:

- `user`
- `user_override`
- `delegated`
- `automation_worker`
- `policy`
- `system`

`user` means direct user authority. `user_override` is reserved for explicit override cases.

## Task Lifecycle

Per `UBU-D0227`, persisted `Task.status` is the canonical lifecycle state and is limited to `active`, `completed`, `failed`, and `moot`. Readiness and execution states are derived views and must not be persisted as canonical task status. A `moot` task requires `moot_reason_code`; non-moot lifecycle states forbid it.

## Planning Envelope Versioning

Planning request, planning response, repair request, and repair response envelopes carry required `schema_version` and `request_id` fields. The initial known planning envelope version set contains only `planning-kernel-contract/0.1`. Unknown versions must produce structured validation diagnostics rather than panics. Successful responses echo the request `schema_version`; Phase 1 does not define a separate response envelope version field.

## Lockstep Coupling

Phase 1 schemas are pre-1.0 and intentionally use `additionalProperties: false` for object contracts where appropriate. This creates lockstep with hand-written `ubu-core` serde types so schema drift fails early in CI and fixture review.

TODO: revisit this coupling at 1.0 and decide whether selected extension points should allow additional properties.

## Vocabulary

Policy-engine output uses `legitimization` and `adjudication`. The engine is the `Legitimizer`; the interception point is the `enforcement gate`.

`Decision` is reserved for UBU-D records and must not be used in schema titles or field names in this repository.
