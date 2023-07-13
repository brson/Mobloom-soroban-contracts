use crate::errors::SCErrors;
use crate::storage::core::CoreState;

use crate::utils::core::{can_init_contract, set_core_state};

use soroban_sdk::{
    contractimpl, panic_with_error, vec, Address, Bytes, BytesN, Env, IntoVal, RawVal, Symbol, Vec,
};
pub trait DaoContractTrait {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: Bytes,
        gov_token_symbol: Bytes,
        shareholders: Vec<(Address, i128)>,
    ) -> (Address, RawVal);
}

pub struct DaoContract;

#[contractimpl]
impl DaoContractTrait for DaoContract {
    fn init(
        env: Env,
        gov_token_salt: BytesN<32>,
        token_wasm_hash: BytesN<32>,
        gov_token_name: Bytes,
        gov_token_symbol: Bytes,
        shareholders: Vec<(Address, i128)>,
    ) -> (Address, RawVal) {
        can_init_contract(&env);
        // Deploy the contract using the installed WASM code with given hash.
        let id = env
            .deployer()
            .with_current_contract(&gov_token_salt)
            .deploy(&token_wasm_hash);

        let init_fn: Symbol = Symbol::new(&env, "initialize");
        let admin: RawVal = env.current_contract_address().to_raw();
        let init_args: Vec<RawVal> = vec![
            &env,
            admin,
            18u32.into(),
            gov_token_name.into(),
            gov_token_symbol.into(),
        ];

        // Invoke the init function with the given arguments.
        let res: RawVal = env.invoke_contract(&id, &init_fn, init_args);

        let mint_fn: Symbol = Symbol::short("mint");
        for shareholder in shareholders {
            match shareholder {
                Ok((shareholder_address, amount)) => {
                    let shareholder_address_raw: RawVal = shareholder_address.to_raw();
                    let mint_args: Vec<RawVal> =
                        vec![&env, shareholder_address_raw, amount.into_val(&env)];
                    let _: RawVal = env.invoke_contract(&id, &mint_fn, mint_args);
                }
                Err(_) => {
                    // Handle the ConversionError as needed
                    // For example, log an error or perform fallback logic
                    panic_with_error!(&env, SCErrors::InitError);
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
