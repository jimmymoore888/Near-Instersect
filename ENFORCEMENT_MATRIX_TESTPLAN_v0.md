# ENFORCEMENT_MATRIX_TESTPLAN_v0
## Proof Plan: Matrix Row → Test → Expected Result → Events

This document is the executable proof companion to `ENFORCEMENT_MATRIX_v0.md`.

Goal:
- every invariant row has at least one deterministic test
- every reject path yields a canonical reason_code
- every allow path yields canonical events
- audit reconstruction is possible from events alone

Scope v0:
- Control Plane: LAW validation + registry + deployment gates
- Data Plane: runtime invariant asserts (minimal)
- OIM: oracle boundary, staleness freeze, cooldown, bounded rebalance

---

# 0) Test Conventions

Test ID format:
- CP-* = Control Plane
- RT-* = Runtime
- OIM-* = OIM-specific
- MOD* = Module-specific

Each test defines:
- Setup
- Action
- Expected outcome (ALLOW/REJECT or PASS/REVERT)
- Expected reason_code (if reject/revert)
- Expected events (minimum set)

Where feasible:
- prefer unit tests at contract level
- add integration tests for deploy flow

---

# 1) Control Plane Tests (LAW + Registry + Deployment)

## CP-01 — LAW Schema Invalid → REJECT (LAW_001_SCHEMA_INVALID)
Setup:
- Create LAW profile missing required field(s) or wrong types
Action:
- submit create_token() with invalid LAW profile
Expected:
- REJECT at EP-1 (compile gate)
Reason Code:
- LAW_001_SCHEMA_INVALID
Events:
- LawValidationStarted
- LawValidationRejected { reason_code=LAW_001_SCHEMA_INVALID }

---

## CP-02 — LAW Integrity Fail (checksum/signature) → REJECT (LAW_002_INTEGRITY_FAIL)
Setup:
- LAW profile with incorrect checksum/signature (if configured)
Action:
- submit create_token()
Expected:
- REJECT at EP-1
Reason Code:
- LAW_002_INTEGRITY_FAIL
Events:
- LawValidationStarted
- LawValidationRejected { reason_code=LAW_002_INTEGRITY_FAIL }

---

## CP-03 — Doctrine Missing Mandatory OIM → REJECT (RCD_002_OIM_MISSING or LAW_003_DOCTRINE_FAIL)
Setup:
- Valid LAW profile
- Module stack excludes Module 8 (OIM)
Action:
- submit create_token()
Expected:
- REJECT at EP-1 or EP-2 depending on implementation point
Reason Code:
- Prefer: RCD_002_OIM_MISSING
- Alternative: LAW_003_DOCTRINE_FAIL
Events:
- LawValidationStarted
- LawValidationRejected OR DeploymentRejected

---

## CP-04 — Doctrine Stack Violation (duplication >2 or count mismatch) → REJECT (RCD_003_STACK_RULE_VIOLATION)
Setup:
- Valid LAW profile
- Module stack violates stacking rules (e.g., same module 3 times)
Action:
- submit create_token()
Expected:
- REJECT
Reason Code:
- RCD_003_STACK_RULE_VIOLATION
Events:
- LawValidationRejected OR DeploymentRejected { reason_code=RCD_003_STACK_RULE_VIOLATION }

---

## CP-05 — Registry: Module Not Found → REJECT (LAW_004_REGISTRY_FAIL or RCD_004_MODULE_NOT_ACTIVE)
Setup:
- LAW profile valid
- module_stack references non-existent module_id
Action:
- create_token()
Expected:
- REJECT
Reason Code:
- LAW_004_REGISTRY_FAIL (preferred at validation stage)
Events:
- LawValidationRejected { reason_code=LAW_004_REGISTRY_FAIL }

---

## CP-06 — Registry: Module Inactive → REJECT (RCD_004_MODULE_NOT_ACTIVE)
Setup:
- Registry contains module but status=SUSPENDED/DEPRECATED
Action:
- create_token() references that module
Expected:
- REJECT at EP-2 deploy gate
Reason Code:
- RCD_004_MODULE_NOT_ACTIVE
Events:
- DeploymentRejected { reason_code=RCD_004_MODULE_NOT_ACTIVE }

---

## CP-07 — Registry: LAW Incompatible → REJECT (RCD_005_LAW_INCOMPATIBLE)
Setup:
- module.law_compat excludes LAW_v1
Action:
- create_token()
Expected:
- REJECT
Reason Code:
- RCD_005_LAW_INCOMPATIBLE
Events:
- DeploymentRejected { reason_code=RCD_005_LAW_INCOMPATIBLE }

---

## CP-08 — Enforcement Matrix Compile Fail → REJECT (LAW_005_MATRIX_COMPILE_FAIL)
Setup:
- Valid LAW profile
- Module stack missing required invariant coverage mapping
Action:
- create_token()
Expected:
- REJECT at EP-1 (matrix compiler)
Reason Code:
- LAW_005_MATRIX_COMPILE_FAIL
Events:
- LawValidationRejected { reason_code=LAW_005_MATRIX_COMPILE_FAIL }

---

## CP-09 — Happy Path Deploy → ALLOW + Canonical Events
Setup:
- Valid LAW profile
- Module stack compliant (includes OIM)
- All modules ACTIVE and compatible
Action:
- create_token()
Expected:
- ALLOW
Outputs:
- compiled_enforcement_hash produced
Events (minimum):
- LawValidationStarted
- LawValidationPassed
- AssetDeployed

---

## CP-10 — No Bypass: Direct Init Attempt → FAIL
Setup:
- Attempt to instantiate asset contract without factory gate (if possible in test harness)
Action:
- call init directly (simulated)
Expected:
- FAIL/REVERT (no bypass)
Reason Code:
- RCD_002_NO_BYPASS (if implemented) or generic revert
Events:
- None required, but SHOULD emit DeploymentRejected if routed through factory

---

# 2) Module Runtime Tests (Minimal v0)

## MOD1-01 — FixedSupply: Mint Attempt → REVERT (MOD1_001_SUPPLY_INVALID)
Setup:
- Deployed asset with FixedSupply
Action:
- call mint() or any supply-increasing action
Expected:
- REVERT
Reason Code:
- MOD1_001_SUPPLY_INVALID (or RuntimeInvariantViolated invariant_id=M1-01)
Events:
- RuntimeInvariantViolated { invariant_id=M1-01 }

---

## MOD3-01 — TimeLock: Withdraw Outside Window → REVERT (MOD3_001_TIMELOCK_INVALID)
Setup:
- Deployed asset with TimeLock schedule
Action:
- attempt withdraw when window closed
Expected:
- REVERT
Reason Code:
- MOD3_001_TIMELOCK_INVALID (or RuntimeInvariantViolated invariant_id=M3-01)
Events:
- RuntimeInvariantViolated { invariant_id=M3-01 }

---

## MOD7-01 — PercentageDistribution Sum Invalid (at config) → REJECT (MOD7_001_DIST_INVALID)
Setup:
- Distribution config sums != 100%
Action:
- deploy or configure distribution
Expected:
- REJECT at compile/deploy, or revert at config call (depending on architecture)
Reason Code:
- MOD7_001_DIST_INVALID
Events:
- DeploymentRejected OR RuntimeInvariantViolated

---

# 3) OIM Tests (Module 8)

## OIM-01 — Oracle Posting Works Only for Oracle Account
Setup:
- OIM mode=ORACLE
- oracle_account set
Action:
- call post_inflation_index() from non-oracle
Expected:
- REVERT
Reason Code:
- OIM_004_ORACLE_PRIV
Events:
- None (reverted), or RuntimeInvariantViolated if emitted on fail

Then:
Action:
- call post_inflation_index() from oracle_account
Expected:
- PASS
Events:
- OIM_INDEX_POSTED

---

## OIM-02 — Oracle Cannot Trigger Rebalance
Setup:
- OIM mode=ORACLE
- oracle_account set
Action:
- oracle calls oim_rebalance()
Expected:
- REVERT
Reason Code:
- OIM_008_GOV_BOUNDARY
Events:
- RuntimeInvariantViolated { invariant_id=INV-OIM-08 } (recommended)

---

## OIM-03 — Oracle Staleness Freeze
Setup:
- OIM mode=ORACLE
- last_index posted_at older than max_oracle_age_sec
Action:
- governance calls oim_rebalance()
Expected:
- NO-OP or REVERT (freeze), status=ORACLE_STALE
Reason Code:
- OIM_005_ORACLE_STALE
Events:
- OIM_STATUS_UPDATED { status=ORACLE_STALE }

---

## OIM-04 — Cooldown Enforcement
Setup:
- OIM configured with rebalance_cooldown_sec
- last_rebalance_at recent
Action:
- governance calls oim_rebalance() again within cooldown
Expected:
- REVERT or NO-OP
Reason Code:
- OIM_006_COOLDOWN
Events:
- (optional) OIM_STATUS_UPDATED, or RuntimeInvariantViolated

---

## OIM-05 — Bounded Rebalance (Caps + Step Limit)
Setup:
- OIM config with safety_cap_bps, growth_cap_bps, max_rebalance_step_bps
- status=BEHIND (set via deterministic accounting conditions)
Action:
- governance calls oim_rebalance()
Expected:
- allocation shifts Safety→Growth within caps and ≤ step limit
Events:
- OIM_STATUS_UPDATED { status=BEHIND or HEALTHY }
- OIM_REBALANCED { from_safety_bps, to_growth_bps }

Negative case:
- attempt to exceed caps via crafted inputs
Expected:
- REVERT
Reason Code:
- OIM_003_BOUNDS_FAIL
Events:
- RuntimeInvariantViolated { invariant_id=INV-OIM-03 }

---

## OIM-06 — Deterministic Accounting Only
Setup:
- Ensure scoring uses on-chain accounting paths only
Action:
- attempt to call any function that would read external price feeds (should not exist)
Expected:
- Not possible / compile-time absence
OR
- REVERT if stub exists
Reason Code:
- OIM_007_NONDET_SCORE
Events:
- RuntimeInvariantViolated { invariant_id=INV-OIM-07 } (if applicable)

---

## OIM-07 — Supply/Timelock Touch Forbidden
Setup:
- Deployed asset with OIM + FixedSupply + TimeLock
Action:
- attempt to route OIM to any supply/timelock mutation (should not exist)
Expected:
- Not possible / compile-time absence
OR
- REVERT if call path exists
Reason Codes:
- OIM_001_SUPPLY_TOUCH
- OIM_002_TIMELOCK_TOUCH
Events:
- RuntimeInvariantViolated

---

# 4) Event Reconstruction Tests

## EVT-01 — Rebuild Deployment Decision from Events
Setup:
- Run CP-09 (happy deploy)
Action:
- collect emitted events
Expected:
- from events alone, reconstruct:
  - law_profile_hash
  - module_stack_hash
  - compiled_enforcement_hash
  - allow decision

Minimum events required:
- LawValidationStarted
- LawValidationPassed
- AssetDeployed

---

## EVT-02 — Rebuild Rejection Decision from Events
Setup:
- Run CP-01 or CP-06
Action:
- collect emitted events
Expected:
- from events alone, reconstruct:
  - rejection reason_code
  - decision boundary (which gate)
  - affected law_profile_hash/module_stack_hash

Minimum events required:
- LawValidationRejected OR DeploymentRejected

---

# 5) Coverage Checklist (v0)

Control Plane Coverage:
- [ ] LAW_001_SCHEMA_INVALID (CP-01)
- [ ] LAW_002_INTEGRITY_FAIL (CP-02)
- [ ] LAW_003_DOCTRINE_FAIL / RCD_002_OIM_MISSING (CP-03)
- [ ] RCD_003_STACK_RULE_VIOLATION (CP-04)
- [ ] LAW_004_REGISTRY_FAIL (CP-05)
- [ ] RCD_004_MODULE_NOT_ACTIVE (CP-06)
- [ ] RCD_005_LAW_INCOMPATIBLE (CP-07)
- [ ] LAW_005_MATRIX_COMPILE_FAIL (CP-08)
- [ ] Happy deploy events (CP-09)

OIM Coverage:
- [ ] Oracle auth boundary (OIM-01)
- [ ] Governance boundary (OIM-02)
- [ ] Staleness freeze (OIM-03)
- [ ] Cooldown enforcement (OIM-04)
- [ ] Bounded rebalance (OIM-05)
- [ ] Deterministic-only scoring (OIM-06)
- [ ] Supply/timelock immutability (OIM-07)

Event Reconstruction:
- [ ] EVT-01
- [ ] EVT-02

---

END ENFORCEMENT_MATRIX_TESTPLAN_v0
