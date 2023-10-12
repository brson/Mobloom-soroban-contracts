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
// dao-wasm-hash -> 7f9630c0227d7a495bda4f7c39616dfb38d670fff7152c1aaac3f18bcf27e6ec

// soroban contract install --wasm ../token/target/wasm32-unknown-unknown/release/soroban_token_contract.wasm
// token-wasm-hash -> cf2318b87338b80ce75d0276244f9be3a131d74656a7fecd1d92da0eb8ab09e3

// soroban --vv contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_deployer_contract.wasm \
//     --source juico \
//     --network futurenet
// deployer_contract_id -> CC77VGFNAWBQX3HZGF2WNJY2WESGZP3I3RV7YWO5HK5W4APSX37AZK2C

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/governance.wasm \
//     --source juico \
//     --network futurenet
// dao_contract_id -> CC6CIJRZ3AVR4BRNOZJWIXVY6S7NWYAOBZ2KMDZ5FTVEJ53L2DBUAW6B

// soroban contract deploy \
//     --wasm target/wasm32-unknown-unknown/release/soroban_token_contract.wasm \
//     --source juico \
//     --network futurenet
// token_contract_id -> CC7KV5EAMCGAKSSBBFQ74GAF3ZCHPEIAMQTI5EJ45VUQH32LZ5JIIY23

// ------------------------------------ FOR RESTORATION ------------------------------------

// soroban --vv contract restore \
//     --id CAGGNI3F7IORBOFKEHOPVD2RLCSRTAQADUF6CVARTT2JHAZOYE2ARBKA \
//     --source juico \
//     --network futurenet

// soroban --vv contract invoke \
//     --id CAGGNI3F7IORBOFKEHOPVD2RLCSRTAQADUF6CVARTT2JHAZOYE2ARBKA \
//     --source juico \
//     --network futurenet -- -h

// soroban --vv contract restore \
//     --wasm-hash 0c66a365fa1d10b8aa21dcfa8f5158a51982001d0be154119cf493832ec13408 \
//     --source juico \
//     --network futurenet
