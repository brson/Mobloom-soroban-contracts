use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct Shareholder {
    pub pub_key: Address,
    pub voting_power: u32,
    pub proposal_power: u32
}

#[contracttype]
pub enum ShareholderStorageKeys {
    Shareholder,
}