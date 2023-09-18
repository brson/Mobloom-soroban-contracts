#![no_std]

use soroban_sdk::{contractimpl, contract, Address, BytesN, Env, Symbol, Vec, Val};

#[contract]
pub struct Deployer;

#[contractimpl]
impl Deployer {
    /// Deploy the contract wasm and after deployment invoke the init function
    /// of the contract with the given arguments. Returns the contract ID and
    /// result of the init function.
    pub fn deploy(
        env: Env,
        salt: BytesN<32>,
        wasm_hash: BytesN<32>,
        init_fn: Symbol,
        init_args: Vec<Val>,
    ) -> (Address, Val) {
        // Deploy the contract using the installed WASM code with given hash.
        let id = env
            .deployer()
            .with_current_contract(salt)
            .deploy(wasm_hash);
        // Invoke the init function with the given arguments.
        let res: Val = env.invoke_contract(&id, &init_fn, init_args);
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}

mod test;

// soroban contract install --wasm ../dao/target/wasm32-unknown-unknown/release/governance.wasm
// dao-wasm-hash -> 13e859532d9db5892e9743f57be8c054e43d101050598663ad5d51a17f8423b0

// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
// token-wasm-hash -> da729cbad2913399edabfc09a8dba948df7b830a5b855fdb196c7a48c3c9fbc1

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet
// deployer_contract_id -> CB5WUJCGKOURLISFS2QW5OCDJGCNXXEWYVGZSFPHKCS6LQIPBHT5JB2M

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/governance.wasm \
//     --source juico \
//     --network futurenet
// dao_contract_id -> CAH3OL4WGP25M75SI5DXSNRLNSDWFF5MHQ5CUEHLVFOERZ47ZDUYOMDI

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CCFQHSCOX7XW3ANGET2M6VDIHQCBPF4MJFHOKEBKX2CEOCNVW24P6YPQ