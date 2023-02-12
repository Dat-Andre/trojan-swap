use cosmwasm_std::{Addr, StdError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ContractError {
    #[error("{0}")]
    Std(#[from] StdError),

    /*   #[error("{0}")]
    Cw20Error(#[from] cw20_base::ContractError), */
    #[error("Invalid token")]
    InvalidToken { received: Addr, expected: Addr },

    #[error("Unauthorized")]
    Unauthorized {},
}
