#![cfg(test)]

use crate::{contract::DaoContract, DaoContractClient};
use soroban_sdk::{testutils::Address as _, Address, BytesN, Env, IntoVal, Vec};

// The contract that will be deployed by the deployer contract.
mod contract {
    soroban_sdk::contractimport!(
        file = "../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm"
    );
}

#[test]
fn test() {
    let env = Env::default();
    let client = DaoContractClient::new(&env, &env.register_contract(None, DaoContract));

    // Install the WASM code to be deployed from the deployer contract.
    let token_wasm_hash = env.install_contract_wasm(contract::WASM);
    let admin1 = Address::random(&env);

    // Deploy contract using deployer, and include an init function to call.
    let token_name = &"name".into_val(&env);
    let salt = BytesN::from_array(&env, &[0; 32]);
    let shareholders: Vec<(Address, i128)> = Vec::from_array(&env, [(admin1.clone(), 200000i128)]);
    let (contract_id, init_result) = client.init(
        &salt,
        &token_wasm_hash,
        &token_name,
        &"symbol".into_val(&env),
        &shareholders,
    );
    assert!(init_result.is_void());

    // Invoke contract to check that it is initialized.
    let client = contract::Client::new(&env, &contract_id);
    let sum = client.balance(&admin1);
    assert_eq!(sum, 200000);

    let name_of_token = client.name();
    assert_eq!(name_of_token, "name".into_val(&env));
}
