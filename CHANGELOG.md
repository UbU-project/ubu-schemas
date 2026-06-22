# Changelog

## 0.1.6

- S14 (`UBU-D0240`): Enriched API risk reports with categorized, severity-tagged findings and added the structured human-complete plan-quality schema for the six plan-quality signals.

## 0.1.5

- S13: Removed the vestigial `candidate-score`, `repair-request`, `repair-response`, `validation-result`, and `skeleton-failure-diagnostic` planning schema stubs and their dedicated fixtures after verifying no retained live artifact references them.

## 0.1.4

- S12 (`UBU-D0239`): Added optional fixed and shifted-log-normal Task duration estimates and optional unsigned correlation-group memberships.

## 0.1.3

- S11 (`UBU-D0237`): Removed the deprecated thin `planning-request`, `planning-response`, and `task-spec` schema stubs and their dedicated fixtures after verifying no retained live artifact references them.

## 0.1.2

- S10 (`UBU-D0236`): Added `planning/affect-profile`, narrowed `core/snapshot` affect payloads to observation-only readings, corrected `mood_intensity` to `lower_is_better` in affect profiles, and added planning-response affect legitimization fields.

## 0.1.1

- S9: Added optional timed placement fields on `plan-step` and optional `supersedes_plan_id` on `plan`, keeping the timed Plan canonical without expanding later-phase planning fields.

## 0.1.0

- Initial UbU Phase 1 JSON Schema scaffold.
- S1a (`UBU-D0228`): Converted Phase 1 schema and fixture wire fields to canonical `snake_case`.
- S1b (`UBU-D0228`): Added generated TypeScript and fixture wire-casing checks to local validation.
- S2 (`UBU-D0227`): Closed persisted `Task.status` to canonical lifecycle states and enforced `moot_reason_code` rules.
- S3 (`UBU-D0229`): Expanded the ID registry and object references for Preference, Container, UniverseState, Identity, Relationship, and ExternalEvent; added canonical `UniverseState`.
- S4: Added planning envelope versioning for planning and repair request/response schemas.
- S5: Closed recalculation-trigger vocabulary and fixtures.
- S6: Added snapshot affect content fixtures and schema coverage.
- S7a (`UBU-D0230`): Added closed policy-summary guardrail members `local_only`, `no_cloud_llm`, and `no_external_export`.
- S7b (`UBU-D0230`): Added `compartment_boundary_decided` log event vocabulary and required payload provenance.
- S8 (`UBU-D0226`): Kept `AuthoritySource` as the pure authority-path enum and moved external/source distinctions into provenance `source` and `source_refs`.
