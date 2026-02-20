# OIM Interface (v1)

## Types

InflationIndex:
- index_id: String
- period: String
- value_bps: u32
- posted_at: u64

OimMode:
- ORACLE
- FIXED_HURDLE

OimConfig:
- mode
- oracle_account (ORACLE only)
- fixed_hurdle_bps_annual (FIXED only)
- measurement_window_days
- min_real_return_bps
- rebalance_cooldown_sec
- max_oracle_age_sec
- safety_cap_bps
- growth_cap_bps
- liquidity_cap_bps
- max_rebalance_step_bps

OimStatus:
- HEALTHY
- BEHIND
- ORACLE_STALE

OimState:
- last_index
- last_rebalance_at
- real_return_score_bps
- status

## Views
- get_oim_config()
- get_oim_state()

## Oracle Method
post_inflation_index(index_id, period, value_bps)

Rules:
- predecessor == oracle_account
- emit OIM_INDEX_POSTED

## Control Plane
oim_rebalance()

Rules:
- enforce cooldown
- enforce oracle freshness (if ORACLE)
- compute score + status
- rebalance Safety <-> Growth if BEHIND
- emit OIM_STATUS_UPDATED
- emit OIM_REBALANCED

## Events
- OIM_INDEX_POSTED
- OIM_STATUS_UPDATED
- OIM_REBALANCED
