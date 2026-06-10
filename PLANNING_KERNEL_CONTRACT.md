# Planning Kernel Contract

## 1. Planning Envelope Versioning

Planning request, planning response, repair request, and repair response envelopes must carry required `schema_version` and `request_id` fields.

The initial known planning envelope version set contains only:

- `planning-kernel-contract/0.1`

Unknown envelope versions must produce structured validation diagnostics rather than panics. Successful planning and repair responses echo the request `schema_version`. Phase 1 does not define a separate response envelope version field.
