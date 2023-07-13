use soroban_sdk::{contracttype, Address};

#[contracttype]
pub struct CoreState {
    pub governance_token: Address,
}

#[contracttype]
pub enum CoreStorageKeys {
    CoreState,
}
