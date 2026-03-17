pub enum TruthState {
    Verified,
    Uncertain,
    Invalid,
}

pub fn assert_truth(state: TruthState) {
    match state {
        TruthState::Verified => {
            // continue execution
        }
        TruthState::Uncertain => {
            env::log_str("STATE_UNCERTAIN");
            env::panic_str("Execution halted: uncertainty detected");
        }
        TruthState::Invalid => {
            env::log_str("STATE_INVALID");
            env::panic_str("Execution halted: invalid data");
        }
    }
}
pub fn process_oracle_data(data: Option<OraclePayload>) {
    match data {
        Some(payload) if payload.is_valid() => {
            assert_truth(TruthState::Verified);
            // proceed safely
        }
        Some(_) => {
            assert_truth(TruthState::Invalid);
        }
        None => {
            assert_truth(TruthState::Uncertain);
        }
    }
}
