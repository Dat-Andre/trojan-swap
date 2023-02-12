use cosmwasm_schema::cw_serde;
use cosmwasm_std::Addr;
use cw_storage_plus::Item;

#[cw_serde]
pub struct Config {
    pub owner: Addr,
    pub mint_token_address: Addr,
    pub payment_token_address: Addr,
    pub receive_payment_address: Addr,
}

pub const CONFIG: Item<Config> = Item::new("config");
