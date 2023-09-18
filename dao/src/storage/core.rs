use soroban_sdk::{contracttype, Address, Vec};

#[contracttype]
pub struct CoreState {
    pub governance_token: Address,
    pub shareholders: Vec<Address>
}

#[contracttype]
pub enum CoreStorageKeys {
    CoreState,
}
