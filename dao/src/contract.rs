use crate::storage::core::CoreState;

use crate::utils::core::{can_init_contract, set_core_state};

use soroban_sdk::{
    contractimpl, vec, Address, Bytes, BytesN, Env, IntoVal, Val, Symbol, Vec, contract, symbol_short, String, Map
};
pub trait DaoContractTrait {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: String,
        gov_token_symbol: String,
        voting_power: u32,
        proposal_power: u32,
        shareholders: Vec<(Address, i128)>,
    ) -> (Address, Val);
}

#[contract]
pub struct DaoContract;

#[contractimpl]
impl DaoContractTrait for DaoContract {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: String,
        gov_token_symbol: String,
        voting_power: u32,
        proposal_power: u32,
        shareholders: Vec<(Address, i128)>,
    ) -> (Address, Val) {
        can_init_contract(&env);
        // Deploy the contract using the installed WASM code with given hash.
        let id = env
            .deployer()
            .with_current_contract(gov_token_salt)
            .deploy(token_wasm_hash);
        
        let init_fn: Symbol = Symbol::new(&env, "initialize");
        let admin: Val = env.current_contract_address().to_val();
        let init_args: Vec<Val> = vec![
            &env,
            admin,
            18u32.into(),
            gov_token_name.into(),
            gov_token_symbol.into()
        ];

        // Invoke the init function with the given arguments.
        let res: Val = env.invoke_contract(&id, &init_fn, init_args);

        let mint_fn: Symbol = symbol_short!("mint");
        let authorize_fn: Symbol = symbol_short!("set_auth");

        let set_proposal_power_fn: Symbol = symbol_short!("set_p_pow");
        let set_voting_power_fn: Symbol = symbol_short!("set_v_pow");
        let proposal_power_res: Val = env.invoke_contract(&id, &set_proposal_power_fn, vec![&env, proposal_power.into_val(&env)]);
        let voting_power_res: Val = env.invoke_contract(&id, &set_voting_power_fn, vec![&env, voting_power.into_val(&env)]);
        for shareholder in shareholders {
            match shareholder {
                (shareholder_address, amount) => {
                    let shareholder_address_raw: Val = shareholder_address.to_val();

                    let auth_args: Vec<Val> = vec![&env, shareholder_address_raw, true.into_val(&env)];
                    let auth_res: Val = env.invoke_contract(&id, &authorize_fn, auth_args);

                    let mint_args: Vec<Val> =
                        vec![&env, shareholder_address_raw, amount.into_val(&env)];
                    let mint_res: Val = env.invoke_contract(&id, &mint_fn, mint_args);
                }
            }
        }
        set_core_state(
            &env,
            &CoreState {
                governance_token: id.clone(),
            },
        );

        (id, res)
    }
    
}
