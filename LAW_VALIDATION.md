# RagTuff LAW Validation in Near-Intersect
## LAW Validation (Constitutional Enforcement Gate)

This document defines **LAW validation** as the pre-execution constitutional gate.

**Core premise:** If LAW is not validated, automation does not execute.

---

## 1) What LAW Validation Is

LAW Validation is the deterministic process that converts a LAW_v1 profile from:
- a declaration of constraints
into
- an enforceable deployment gate.

It must be:
- deterministic
- auditable
- reject-by-default

---

## 2) Inputs / Outputs

### Inputs
- `law_profile` (LAW_v1)
- `law_profile_hash`
- `module_stack` (ordered list)
- `registry_state` (module status + compat + invariant maps)
- `creator_id`

### Outputs
- `ALLOW` with:
  - `compiled_enforcement_hash`
  - `deployment_receipt_hash`
- `REJECT` with:
  - `reason_code`
  - `details_hash`

---

## 3) Validation Stages (v1)

### Stage V-1: Schema Validity
- LAW_v1 schema version recognized
- required fields present
- types correct
- canonical serialization stable

Reject code: `LAW_001_SCHEMA_INVALID`

### Stage V-2: Integrity / Authority
- profile checksum / signature verified (if configured)
- version constraints enforced (no downgrade)

Reject code: `LAW_002_INTEGRITY_FAIL`

### Stage V-3: Doctrine Requirements
- mandatory modules present (OIM)
- module count and stacking rules enforced (Near-Intersect doctrine)
- deterministic ordering enforced (if required)

Reject code: `LAW_003_DOCTRINE_FAIL`

### Stage V-4: Registry Conformance
- every module exists in registry
- module status ACTIVE
- module LAW compatibility includes LAW_v1

Reject code: `LAW_004_REGISTRY_FAIL`

### Stage V-5: Enforcement Matrix Compile
- map LAW invariants → required enforcement points
- confirm module stack covers required invariants
- produce `compiled_enforcement_hash`

Reject code: `LAW_005_MATRIX_COMPILE_FAIL`

### Stage V-6: Deployment Gate Decision
- if all stages pass → ALLOW
- otherwise → REJECT (no partial deploy)

---

## 4) Enforcement Matrix (Minimum Concept)

The enforcement matrix is a deterministic table:

- rows: LAW invariants
- columns: enforcement points (compile gate, deploy gate, runtime invariants)
- cells: module responsibility + check type + event mapping

Minimum enforcement points:
- `COMPILE_GATE`
- `DEPLOY_GATE`
- `RUNTIME_ASSERT`

A LAW profile is enforceable only if matrix compile succeeds.

---

## 5) Validation Fees (Principle)

LAW Validation Fees exist because validation is:
- compute work (verification)
- risk work (gatekeeping)
- audit work (event trail)

Fee model (rail-agnostic):
- `LAW_VALIDATION_FEE = $25 – $150 equivalent`

Fees must be:
- predictable
- transparent
- deterministic

---

## 6) Required Events

- `LawValidationStarted(creator_id, law_profile_hash, module_stack_hash)`
- `LawValidationPassed(creator_id, law_profile_hash, compiled_enforcement_hash)`
- `LawValidationRejected(creator_id, law_profile_hash, reason_code, details_hash)`

Events are part of the constitutional audit trail.

---

## 7) Canonical Rejection Codes

- `LAW_001_SCHEMA_INVALID`
- `LAW_002_INTEGRITY_FAIL`
- `LAW_003_DOCTRINE_FAIL`
- `LAW_004_REGISTRY_FAIL`
- `LAW_005_MATRIX_COMPILE_FAIL`

---

## 8) Design Intent

LAW validation is the constitutional gate that prevents:
- discretionary drift
- bypass deployments
- non-compliant module stacks
- unverifiable automation

Near-Intersect is governed by RagTuff by construction, not by promise.
