use cosmwasm_schema::{cw_serde, QueryResponses};
use cw20::Cw20ReceiveMsg;

use crate::state::Config;

#[cw_serde]
pub struct InstantiateMsg {
    pub owner: String,
    pub mint_token_address: String,
    pub payment_token_address: String,
    pub receive_payment_address: String,
}

#[cw_serde]
pub enum ExecuteMsg {
    Swap(Cw20ReceiveMsg),
    UpdateConfig {
        owner: String,
        mint_token_address: String,
        payment_token_address: String,
        receive_payment_address: String,
    },
}

#[cw_serde]
pub enum ReceiveMsg {
    Swap {},
}

#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // GetConfig returns the current configuration
    #[returns(GetConfigResponse)]
    GetConfig {},
}

// We define a custom struct for each query response
#[cw_serde]
pub struct GetConfigResponse {
    pub config: Config,
}
