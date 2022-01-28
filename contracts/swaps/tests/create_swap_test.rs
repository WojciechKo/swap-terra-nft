use cosmwasm_std::{Addr, StdError};

use swaps::contract::{execute, instantiate, query};
use swaps::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SwapResponse};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    use cosmwasm_std::{coins, from_binary};
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn proper_initialization() -> Result<(), String> {
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(1000, "earth"));

        let res = instantiate(deps.as_mut(), mock_env(), info, msg).unwrap();
        assert_eq!(0, res.messages.len());
        Ok(())
    }

    #[test]
    fn swap_validation() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), creator_info, msg);

        let get_swap_msg = QueryMsg::GetSwap {
            swap_id: String::from("1"),
        };

        // Initiate Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::InitiateSwap {
            collection: String::from("gp_collection"),
            token_id: String::from("123"),
        };
        execute(deps.as_mut(), mock_env(), swapper_info, create_swap_msg).unwrap();

        // Get created Swap
        let get_swap_response = query(deps.as_ref(), mock_env(), get_swap_msg.clone()).unwrap();
        let swap_response: SwapResponse = from_binary(&get_swap_response).unwrap();

        assert_eq!(
            swap_response.lhs.collection,
            Addr::unchecked("gp_collection")
        );
        assert_eq!(swap_response.lhs.owner, Addr::unchecked("swapper"));
        assert_eq!(swap_response.lhs.token_id, String::from("123"));

        Ok(())
    }

    #[test]
    fn create_swap() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), creator_info, msg);

        let get_swap_msg = QueryMsg::GetSwap {
            swap_id: String::from("1"),
        };

        // When Swap does not exists
        // Fetch Swap returns error
        match query(deps.as_ref(), mock_env(), get_swap_msg.clone()) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(
                err,
                StdError::NotFound {
                    kind: String::from("Swap")
                }
            ),
        };

        // Initiate Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::InitiateSwap {
            collection: String::from("gp_collection"),
            token_id: String::from("123"),
        };
        execute(deps.as_mut(), mock_env(), swapper_info, create_swap_msg).unwrap();

        // Get created Swap
        let get_swap_response = query(deps.as_ref(), mock_env(), get_swap_msg.clone()).unwrap();
        let swap_response: SwapResponse = from_binary(&get_swap_response).unwrap();

        assert_eq!(
            swap_response.lhs.collection,
            Addr::unchecked("gp_collection")
        );
        assert_eq!(swap_response.lhs.owner, Addr::unchecked("swapper"));
        assert_eq!(swap_response.lhs.token_id, String::from("123"));

        Ok(())
    }
}
