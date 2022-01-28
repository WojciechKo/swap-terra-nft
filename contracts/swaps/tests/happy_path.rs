use swaps::contract::{execute, instantiate};
use swaps::msg::{ExecuteMsg, InstantiateMsg};

#[cfg(test)]
mod tests {
    use super::*;
    use cosmwasm_std::coins;
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info};
    #[allow(unused_imports)]
    use pretty_assertions::{assert_eq, assert_ne};

    #[test]
    fn swap_happy_path() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&[]);
        let msg = InstantiateMsg {};
        let owner = mock_info("creator", &coins(2, "token"));
        let _res = instantiate(deps.as_mut(), mock_env(), owner, msg);

        // Create Swap
        let creator_info = mock_info("creator", &coins(2, "token"));
        let initiate_swap_msg = ExecuteMsg::InitiateSwap {
            collection: String::from("gp_collection"),
            token_id: String::from("123"),
        };
        let swap_created = execute(
            deps.as_mut(),
            mock_env(),
            creator_info.clone(),
            initiate_swap_msg,
        )
        .unwrap();

        let swap_id = swap_created
            .attributes
            .iter()
            .find(move |x| x.key == String::from("swap_id"))
            .unwrap()
            .value
            .clone();

        // Reply to Swap
        let responder_info = mock_info("responder", &coins(2, "token"));
        let swap_reply_msg = ExecuteMsg::SwapReply {
            swap_id: swap_id.clone(),
            collection: String::from("goochi-goochi"),
            token_id: String::from("abc"),
        };
        execute(deps.as_mut(), mock_env(), responder_info, swap_reply_msg).unwrap();

        // Finalize Swap
        let finalize_swap_msg = ExecuteMsg::FinalizeSwap { swap_id: swap_id };
        execute(
            deps.as_mut(),
            mock_env(),
            creator_info.clone(),
            finalize_swap_msg,
        )
        .unwrap();

        Ok(())
    }
}
