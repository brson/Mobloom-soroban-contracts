use soroban_sdk::{Address, Env};

use crate::storage_types::DataKey;

pub fn has_administrator(e: &Env) -> bool {
    let key = DataKey::Admin;
    e.storage().persistent().has(&key)
}

pub fn read_administrator(e: &Env) -> Address {
    let key = DataKey::Admin;
    e.storage().persistent().get(&key).unwrap()
}

pub fn write_administrator(e: &Env, id: &Address) {
    let key = DataKey::Admin;
    e.storage().persistent().set(&key, id);
}
