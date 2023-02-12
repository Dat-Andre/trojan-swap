#[cfg(test)]
mod tests {
    /* use cosmwasm_std::{Addr, Empty};
    use cw20::{Cw20Coin, MinterResponse};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    const DAO_ADDR: &str = "addr0001";
    const PAYMENT_ADDR: &str = "addr0002";
    const ADDR3: &str = "addr0003";
    const ADDR4: &str = "addr0004";

    pub fn contract_trojan_swap() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            crate::contract::execute,
            crate::contract::instantiate,
            crate::contract::query,
        );
        Box::new(contract)
    }

    pub fn contract_cw20_minter() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        );
        Box::new(contract)
    }

    pub fn contract_cw20_payment() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(
            cw20_base::contract::execute,
            cw20_base::contract::instantiate,
            cw20_base::contract::query,
        );
        Box::new(contract)
    }

    fn mock_app() -> App {
        App::default()
    }

    fn instantiate_cw20_minter(app: &mut App, initial_balances: Vec<Cw20Coin>) -> Addr {
        let cw20_id = app.store_code(contract_cw20_minter());
        let msg = cw20_base::msg::InstantiateMsg {
            name: String::from("Bond"),
            symbol: String::from("BOND"),
            decimals: 6,
            initial_balances,
            mint: Some(MinterResponse {
                minter: Addr::unchecked(DAO_ADDR).to_string(),
                cap: None,
            }),
            marketing: None,
        };

        app.instantiate_contract(cw20_id, Addr::unchecked(DAO_ADDR), &msg, &[], "cw20", None)
            .unwrap()
    }

    fn instantiate_cw20_payment(app: &mut App, initial_balances: Vec<Cw20Coin>) -> Addr {
        let cw20_id = app.store_code(contract_cw20_payment());
        let msg = cw20_base::msg::InstantiateMsg {
            name: String::from("Red"),
            symbol: String::from("RED"),
            decimals: 6,
            initial_balances,
            mint: None,
            marketing: None,
        };

        app.instantiate_contract(
            cw20_id,
            Addr::unchecked(PAYMENT_ADDR),
            &msg,
            &[],
            "cw20",
            None,
        )
        .unwrap()
    }

    fn instantiate_trojan_swap(app: &mut App, cw20: Addr) -> Addr {
        let staking_code_id = app.store_code(contract_trojan_swap());
        let msg = crate::msg::InstantiateMsg {
            owner: DAO_ADDR.to_string(),
            mint_token_address: todo!(),
            payment_token_address: todo!(),
            receive_payment_address: todo!(),
        };
        app.instantiate_contract(
            staking_code_id,
            Addr::unchecked(ADDR1),
            &msg,
            &[],
            "staking",
            Some("admin".to_string()),
        )
        .unwrap()
    } */
}
