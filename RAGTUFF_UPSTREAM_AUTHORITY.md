# RAGTUFF_UPSTREAM_AUTHORITY.md

## Purpose

Near-Intersect is the execution wrapper.

This file declares RagTuff as the **upstream authority** that governs what Near-Intersect is allowed to deploy.

---

## Upstream Authority

RagTuff is the constitutional constraint layer:
- defines invariants
- defines refusal posture and failure modes
- defines canonical law structures and profiles

Near-Intersect does not redefine RagTuff.
Near-Intersect enforces RagTuff by compiling and deploying only what is compliant.

---

## Deployment Permission Rule

Deployment is allowed **only** from RagTuff-anchored `LAW v1` profiles.

A valid deployment input MUST:
- conform 1:1 to `LAW_v1_SCHEMA`
- include RagTuff anchoring data that binds the profile to RagTuff canonical law:
  - canonical reference(s)
  - framework commit
  - framework hash
  - any additional integrity anchors required

If the profile is not RagTuff-anchored, deployment MUST be rejected.

---

## Determinism and Verification

Near-Intersect enforces determinism:
- Same input law profile → same compiled output
- Hashes emitted by compilation must bind to:
  - the law profile
  - the schema
  - the invariant set
  - the RagTuff anchors

Any non-determinism or missing anchors is a compile/deploy failure.

---

## Execution Boundary

Near-Intersect may execute on-chain actions only through compiled contracts that satisfy the upstream law.

Near-Intersect does not:
- bypass RagTuff constraints
- introduce discretionary execution paths
- accept “manual overrides” that violate law
- deploy from unanchored or non-compliant profiles

---

## Relationship Summary

- RagTuff = upstream law / invariants / constraints
- Near-Intersect = wrapper that enforces law via deterministic compilation and deploy gating

Together:
**Law before automation, enforced at launch.**

---
