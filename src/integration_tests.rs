#[cfg(test)]
mod tests {
    use std::borrow::BorrowMut;

    use cosmwasm_std::{
        testing::{mock_dependencies, mock_env, mock_info},
        to_binary, Addr, Empty, Uint128,
    };
    use cw20::{Cw20Coin, MinterResponse};
    use cw_multi_test::{next_block, App, Contract, ContractWrapper, Executor};

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

    fn instantiate_trojan_swap(
        app: &mut App,
        owner: String,
        mint_token_address: String,
        payment_token_address: String,
        receive_payment_address: String,
    ) -> Addr {
        let staking_code_id = app.store_code(contract_trojan_swap());
        let msg = crate::msg::InstantiateMsg {
            owner,
            mint_token_address,
            payment_token_address,
            receive_payment_address,
        };
        app.instantiate_contract(
            staking_code_id,
            Addr::unchecked(ADDR4.to_string()),
            &msg,
            &[],
            "staking",
            Some("admin".to_string()),
        )
        .unwrap()
    }

    fn setup_test_case(app: &mut App) -> (Addr, Addr, Addr) {
        // instantiate mint cw20
        let cw20_mint_addr = instantiate_cw20_minter(app, vec![]);
        // instantiate payment cw20
        let initial_balances = vec![Cw20Coin {
            address: PAYMENT_ADDR.to_string(),
            amount: Uint128::from(1000u128),
        }];
        let cw20_payment_addr = instantiate_cw20_payment(app, initial_balances);
        // instantiate trojan swap
        let trojan_swap_addr = instantiate_trojan_swap(
            app,
            DAO_ADDR.to_string(),
            cw20_mint_addr.to_string(),
            cw20_payment_addr.to_string(),
            DAO_ADDR.to_string(),
        );

        // update minter on cw20 mint contract
        let msg = cw20_base::msg::ExecuteMsg::UpdateMinter {
            new_minter: Some(trojan_swap_addr.to_string()),
        };
        app.borrow_mut().execute_contract(
            Addr::unchecked(DAO_ADDR.to_string()),
            cw20_mint_addr.clone(),
            &msg,
            &[],
        );

        (cw20_mint_addr, cw20_payment_addr, trojan_swap_addr)
        /* // Instantiate cw20 contract
        let cw20_addr = instantiate_cw20(app, initial_balances);
        app.update_block(next_block);
        // Instantiate staking contract
        let staking_addr = instantiate_staking(app, cw20_addr.clone(), unstaking_duration);
        app.update_block(next_block);
        (staking_addr, cw20_addr) */
    }

    #[test]
    fn test_swap() {
        let _deps = mock_dependencies();

        let mut app = mock_app();
        let amount1 = Uint128::from(100u128);
        let _token_address = Addr::unchecked("token_address");
        /* let initial_balances = vec![Cw20Coin {
            address: ADDR1.to_string(),
            amount: amount1,
        }]; */
        let (cw20_mint_addr, cw20_payment_addr, trojan_swap_addr) = setup_test_case(&mut app);

        let info = mock_info(PAYMENT_ADDR, &[]);
        let _env = mock_env();

        // Test swap
        let msg = cw20::Cw20ExecuteMsg::Send {
            contract: trojan_swap_addr.to_string(),
            amount: Uint128::new(100),
            msg: to_binary(&crate::msg::ReceiveMsg::Swap {}).unwrap(),
        };
        app.execute_contract(
            Addr::unchecked(PAYMENT_ADDR.to_string()),
            cw20_payment_addr.clone(),
            &msg,
            &[],
        )
        .unwrap();

        /*  let msg = cw20::Cw20ExecuteMsg::Transfer {
            recipient: DAO_ADDR.to_string(),
            amount: Uint128::new(100),
        };
        app.execute_contract(
            Addr::unchecked(PAYMENT_ADDR.to_string()),
            cw20_payment_addr.clone(),
            &msg,
            &[],
        ); */

        app.update_block(next_block);
        //Query payment_addr balance of minter and payment cw20
        let msg = cw20::Cw20QueryMsg::Balance {
            address: PAYMENT_ADDR.to_string(),
        };
        let result: cw20::BalanceResponse = app
            .wrap()
            .query_wasm_smart(cw20_payment_addr.clone(), &msg)
            .unwrap();

        assert_eq!(result.balance, Uint128::new(900));

        let msg = cw20::Cw20QueryMsg::Balance {
            address: PAYMENT_ADDR.to_string(),
        };
        let result: cw20::BalanceResponse = app
            .wrap()
            .query_wasm_smart(cw20_mint_addr.clone(), &msg)
            .unwrap();
        assert_eq!(result.balance, Uint128::from(100u128));

        /* let msg = QueryMsg::StakedBalanceAtHeight {
                address: address.into(),
                height: None,
            };
        let result: StakedBalanceAtHeightResponse =
                app.wrap().query_wasm_smart(contract_addr, &msg).unwrap();
            result.balance     */

        /*  let config = query_config(&app, &staking_addr);
        assert_eq!(config.owner, Some(Addr::unchecked("owner2")));
        assert_eq!(config.unstaking_duration, Some(Duration::Height(100))); */
    }
}
