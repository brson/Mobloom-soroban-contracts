use soroban_sdk::{Env, String};
use soroban_token_sdk::metadata;
use soroban_token_sdk::TokenUtils;

pub fn read_decimal(e: &Env) -> u32 {
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().decimal
}

pub fn read_name(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().name
}

pub fn read_symbol(e: &Env) -> String {
    let util = TokenUtils::new(e);
    util.metadata().get_metadata().symbol
}

pub fn write_metadata(e: &Env, metadata: metadata::TokenMetadata) {
    let util = TokenUtils::new(e);
    let meta = metadata::Metadata::new(e);
    util.metadata().set_metadata(&metadata);
}
