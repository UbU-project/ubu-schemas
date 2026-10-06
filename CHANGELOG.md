# Changelog

## Unreleased

- P1B-61 A: Add `precondition` to both candidate-kind enums, constrain its normalized proposal to a canonical tree or an explicit existing/proposed tree pair, and add synthetic valid/invalid fixtures.

- P1B-60 A: Correct UniverseState summaries: required non-empty `source_summary` and optional nullable `confidence_summary` are strings. The schema had declared both as objects while every implementation wrote strings. Canonical fixtures, object-valued refusal fixtures and a whole-state round-trip fixture expose the contract to compatibility tests.

- P1B-59 A: A measured number is a first-class fact.
  - `core/universe-state-mutation` gains `set_numeric` and `clear_numeric`. `payload` is required for every operation except `clear_fact` and `clear_numeric`, which now refuse one. The payload of `set_numeric` must be a number.
  - `core/universe-state-mutation` gains optional `provenance_kind` (`asserted`, `measured`, `derived`, `proposed`), refused on the two clears. **`note` is removed**: it was accepted and stored nowhere. A mutation that carries `note` is now invalid.
  - `core/universe-state` gains an optional `fact_provenance` map keyed by full target, each value `{kind, recorded_at}` and nothing else. It is not required, so existing documents stay valid. The first commit of this section named it `provenance`; a stored UniverseState payload already carries the object envelope under that key, so it was renamed before anything read it.
  - `core/precondition` gains `at_least`, `at_most`, `greater_than` and `less_than`, each requiring a `numeric_values` target and a numeric `expected`. The invalid fixture `unknown-predicate` used `greater_than`, which is now a predicate; it uses an unknown name, and its old content is the invalid fixture `comparison-on-facts-target`.
  - A `clear_fact` with a `payload` was valid against the schema and refused by `ubu-core`. The schema now refuses it too.

## 0.1.9

- S17 (`UBU-D0242`): Added optional Task `effects` object with a nullable `success_probability` and a `mutations` list referencing the core UniverseState mutation schema.

## 0.1.8

- S16 (`UBU-D0242`): Added optional Task `preconditions` referencing the recursive core precondition schema.

## 0.1.7

- S15 (`UBU-D0241`): Reshaped `core/universe-state` into the four-collection facts container and added UniverseState mutation-item and precondition schemas with fixtures.

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

## P1B-67

Add universe_target to candidate and worker-authority enums, with two valid
and three invalid invented fixtures for its name-only proposal contract.
