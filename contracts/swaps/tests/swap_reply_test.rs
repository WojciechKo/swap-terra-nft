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
    fn cant_respond_to_not_existing_swap() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let creator_info = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), creator_info, msg);

        // Reply to not existing Swap
        let responder_info = mock_info("responder", &coins(2, "token"));
        let swap_reply_msg = ExecuteMsg::SwapReply {
            swap_id: "123".to_string(),
            collection: String::from("goochi-goochi"),
            token_id: String::from("abc"),
        };

        match execute(deps.as_mut(), mock_env(), responder_info, swap_reply_msg) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(err, ContractError::SwapNotFound {}),
        };

        Ok(())
    }
}
