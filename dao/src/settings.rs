use soroban_sdk::{Env};

use crate::storage::proposal_storage::ProposalStorageKey;

// `percent` -> percent of quorum needed to pass proposal.
// from 0 to 100
pub fn set_quorum(env: &Env, percent: u32) {
    env.storage().persistent().set(&ProposalStorageKey::Quorum, &percent)
}

pub fn get_quorum(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::Quorum)
        .unwrap_or(0)
}

// set min duration of proposal
pub fn set_min_prop_duration(env: &Env, min_time_seconds: u32) {
    env.storage().persistent().set(&ProposalStorageKey::MinTime, &min_time_seconds)
}

pub fn get_min_prop_duration(env: &Env) -> u32 {
    env.storage()
        .persistent()
        .get(&ProposalStorageKey::MinTime)
        .unwrap_or(1)
}