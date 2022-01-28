use swaps::contract::{execute, instantiate};
use swaps::error::ContractError;
use swaps::msg::{ExecuteMsg, InstantiateMsg};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn cant_finalize_not_existing_sawp() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), creator_info.clone(), msg);

        // Try to finalize not existing Swap
        let finalize_swap_msg = ExecuteMsg::FinalizeSwap {
            swap_id: "123".to_string(),
        };
        match execute(
            deps.as_mut(),
            mock_env(),
            creator_info.clone(),
            finalize_swap_msg,
        ) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(err, ContractError::SwapNotFound {}),
        };

        Ok(())
    }

    #[test]
    fn cant_finalize_unresponded_sawp() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), creator_info, msg);

        // Initiate Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::InitiateSwap {
            collection: String::from("gp_collection"),
            token_id: String::from("123"),
        };
        let swap_created = execute(
            deps.as_mut(),
            mock_env(),
            swapper_info.clone(),
            create_swap_msg,
        )
        .unwrap();
        let swap_id = swap_created
            .attributes
            .iter()
            .find(move |x| x.key == String::from("swap_id"))
            .unwrap()
            .value
            .clone();

        // Try to finalize not responded Swap
        let finalize_swap_msg = ExecuteMsg::FinalizeSwap { swap_id: swap_id };
        match execute(
            deps.as_mut(),
            mock_env(),
            swapper_info.clone(),
            finalize_swap_msg,
        ) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(err, ContractError::SwapNotResponded {}),
        };

        Ok(())
    }
}
