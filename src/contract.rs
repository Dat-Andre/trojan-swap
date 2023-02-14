#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, GetConfigResponse, InstantiateMsg, QueryMsg};
use crate::state::{Config, CONFIG};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:trojan-swap";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    let state = Config {
        owner: deps.api.addr_validate(&msg.owner)?,
        mint_token_address: deps.api.addr_validate(&msg.mint_token_address)?,
        payment_token_address: deps.api.addr_validate(&msg.payment_token_address)?,
        receive_payment_address: deps.api.addr_validate(&msg.receive_payment_address)?,
    };
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    CONFIG.save(deps.storage, &state)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", msg.owner)
        .add_attribute("mint_token_address", msg.mint_token_address)
        .add_attribute("payment_token_address", msg.payment_token_address)
        .add_attribute("receive_payment_address", msg.receive_payment_address))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::Receive(msg) => execute::execute_swap(deps, info, msg),
        ExecuteMsg::UpdateConfig {
            owner,
            mint_token_address,
            payment_token_address,
            receive_payment_address,
        } => execute::execute_update_config(
            deps,
            info,
            owner,
            mint_token_address,
            payment_token_address,
            receive_payment_address,
        ),
    }
}

pub mod execute {
    use cosmwasm_std::{from_binary, Addr, Uint128};
    use cw20::Cw20ReceiveMsg;

    use crate::msg::ReceiveMsg;

    use super::*;

    pub fn execute_update_config(
        deps: DepsMut,
        info: MessageInfo,
        owner: String,
        mint_token_address: String,
        payment_token_address: String,
        receive_payment_address: String,
    ) -> Result<Response, ContractError> {
        let mut config = CONFIG.load(deps.storage)?;

        // only the owner can update the config
        if info.sender != config.owner {
            return Err(ContractError::Unauthorized {});
        }

        config.owner = deps.api.addr_validate(&owner)?;
        config.mint_token_address = deps.api.addr_validate(&mint_token_address)?;
        config.payment_token_address = deps.api.addr_validate(&payment_token_address)?;
        config.receive_payment_address = deps.api.addr_validate(&receive_payment_address)?;

        CONFIG.save(deps.storage, &config)?;

        Ok(Response::new()
            .add_attribute("method", "execute_update_config")
            .add_attribute("owner", owner)
            .add_attribute("mint_token_address", mint_token_address)
            .add_attribute("payment_token_address", payment_token_address)
            .add_attribute("receive_payment_address", receive_payment_address))
    }

    pub fn execute_swap(
        deps: DepsMut,
        info: MessageInfo,
        wrapper: Cw20ReceiveMsg,
    ) -> Result<Response, ContractError> {
        let config = CONFIG.load(deps.storage)?;

        // validate that the tranfered token is the payment token
        if info.sender != config.payment_token_address {
            return Err(ContractError::InvalidToken {
                received: info.sender,
                expected: config.payment_token_address,
            });
        }

        let msg: ReceiveMsg = from_binary(&wrapper.msg)?;
        let sender = deps.api.addr_validate(&wrapper.sender)?;
        match msg {
            ReceiveMsg::Swap {} => process_trojan(deps, sender, wrapper.amount),
        }
    }

    pub fn process_trojan(
        deps: DepsMut,
        sender: Addr,
        amount: Uint128,
    ) -> Result<Response, ContractError> {
        let config = CONFIG.load(deps.storage)?;

        // create the cw20 mint message with recipient set to sender field of the Cw20ReceiveMsg
        let cw_mint_msg = cw20::Cw20ExecuteMsg::Mint {
            recipient: sender.to_string(),
            amount,
        };
        let wasm_msg_mint = cosmwasm_std::WasmMsg::Execute {
            contract_addr: config.mint_token_address.to_string(),
            msg: to_binary(&cw_mint_msg)?,
            funds: vec![],
        };

        // create the cw20 transfer message with recipient set to the receive_payment_address
        let cw_transfer_msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: config.receive_payment_address.to_string(),
            amount,
        };

        let wasm_msg_transfer = cosmwasm_std::WasmMsg::Execute {
            contract_addr: config.payment_token_address.to_string(),
            msg: to_binary(&cw_transfer_msg)?,
            funds: vec![],
        };

        Ok(Response::new()
            .add_message(wasm_msg_mint)
            .add_message(wasm_msg_transfer)
            .add_attribute("action", "swap")
            .add_attribute("asked by", sender.to_string())
            .add_attribute("amount", amount))
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetConfig {} => to_binary(&query::get_config(deps)?),
    }
}

pub mod query {
    use super::*;

    pub fn get_config(deps: Deps) -> StdResult<GetConfigResponse> {
        let config = CONFIG.load(deps.storage)?;
        Ok(GetConfigResponse { config })
    }
}
