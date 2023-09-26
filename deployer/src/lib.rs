#![no_std]

use soroban_sdk::{contract, contractimpl, Address, BytesN, Env, Symbol, Val, Vec};

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
        let id = env.deployer().with_current_contract(salt).deploy(wasm_hash);
        // Invoke the init function with the given arguments.
        let res: Val = env.invoke_contract(&id, &init_fn, init_args);
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}

mod test;

// soroban contract install --wasm ../dao/target/wasm32-unknown-unknown/release/governance.wasm
// dao-wasm-hash -> 5c286c5f20ad37ee8992ddaa99973dfe52063074facb4ae109bc7810ffe3a95c

// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
// token-wasm-hash -> d97bdb32454f36648f8e291d9995c0985879a8fb78519c8a8cb4143c7c51f3ff

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet
// deployer_contract_id -> CBMFJGHFAGPVH2FXQQK7QDBLVFF3AQNZ5XRYSMOJKPHITV3GH4CPT6BX

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/governance.wasm \
//     --source juico \
//     --network futurenet
// dao_contract_id -> CDJW6WIVGMEFASX3REG5PEIHLDHMAMDDYU72537LJJ57QZZ5WQKQS5GL

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CB4NVEPGSB4XFDOJ2ONCIZVITWTGA4CBDBW6YA6QAG7JFH3QU5DRQPMJ
