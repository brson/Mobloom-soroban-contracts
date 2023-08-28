#![no_std]

use soroban_sdk::{contractimpl, Address, BytesN, Env, RawVal, Symbol, Vec};

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
        init_args: Vec<RawVal>,
    ) -> (Address, RawVal) {
        // Deploy the contract using the installed WASM code with given hash.
        let id = env
            .deployer()
            .with_current_contract(&salt)
            .deploy(&wasm_hash);
        // Invoke the init function with the given arguments.
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);
        // Return the contract ID of the deployed contract and the result of
        // invoking the init result.
        (id, res)
    }
}

mod test;

// DEPLOYER CONTRCAT FUTURENET ID -> CC6IKYRGAYWJ6YWTAINLK75PH352ZMADMCVDEUEYAQQXGNZTXNDIZUFQ

// soroban contract install --wasm ../dao/target/wasm32-unknown-unknown/release/governance.wasm
// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm

// governance.wasm hash: 55bc5b52c43a1f4eff4da6c8cedd3b37c11b370d40aaf46ca82bbf9305dbd852
// soroban_token_contract.wasm hash: 60a73484b136e43ea40b716850a2d0ae470f00c9747bf908d28cf7a9e37bd6f6

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet

// soroban contract invoke \
//     --id CC6IKYRGAYWJ6YWTAINLK75PH352ZMADMCVDEUEYAQQXGNZTXNDIZUFQ \
//     --source juico \
//     --network futurenet \
//     -- deploy \
//     --salt 123 \
//     --wasm_hash 55bc5b52c43a1f4eff4da6c8cedd3b37c11b370d40aaf46ca82bbf9305dbd852 \
//     --init_fn init \
//     --init_args '[
//         {"bytes": "[208, 220, 176, 14, 34, 226, 1, 253, 110, 83, 177, 108, 63, 62, 47, 140, 186, 41, 115, 216, 41, 109, 168, 171, 43, 214, 47, 108, 181, 213, 106, 41]"},
//         {"bytes": "[96, 167, 52, 132, 177, 54, 228, 62, 164, 11, 113, 104, 80, 162, 208, 174, 71, 15, 0, 201, 116, 123, 249, 8, 210, 140, 247, 169, 227, 123, 214, 246]"},
//         {"bytes": "[11, 0]"},
//         {"bytes": "[11]"},
//         {"i128": 5},
//         {"i128": 5},
//         {"vec": [{"address": "GB4ZLIQWAWNH3VKEFD2LXCYL4WYHYOGRG333457ZRYSANSQM3AFPCX7E", "i128": 10}]}
//     ]'