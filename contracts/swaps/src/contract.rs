use cosmwasm_std::StdError;
#[cfg(not(feature = "library"))]
use cosmwasm_std::{
    entry_point, to_binary, Addr, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, SwapResponse};
use crate::state::{Config, Swap, CONFIG, SWAPS};

// version info for migration info
const CONTRACT_NAME: &str = "crates.io:counter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;

    let config = Config { next_swap_id: 1 };
    CONFIG.save(deps.storage, &config)?;

    Ok(Response::new()
        .add_attribute("method", "instantiate")
        .add_attribute("owner", info.sender))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    _env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::CreateSwap {
            collection,
            token_id,
        } => try_create_swap(deps, info, collection, token_id),
        ExecuteMsg::CancelSwap { swap_id } => try_cancel_swap(deps, info, swap_id),
    }
}

pub fn try_create_swap(
    deps: DepsMut,
    info: MessageInfo,
    collection_address: Addr,
    token_id: String,
) -> Result<Response, ContractError> {
    let data = Swap {
        owner: info.sender.clone(),
        collection: collection_address,
        token_id: token_id,
    };

    let swap_id = CONFIG.load(deps.storage).unwrap().next_swap_id;
    SWAPS.save(deps.storage, swap_id.to_string(), &data)?;

    CONFIG.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.next_swap_id += 1;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("method", "create_swap")
        .add_attribute("swap_id", swap_id.to_string()))
}

pub fn try_cancel_swap(
    deps: DepsMut,
    _info: MessageInfo,
    swap_id: String,
) -> Result<Response, ContractError> {
    match SWAPS.load(deps.storage, swap_id.clone()) {
        Ok(_) => (),
        Err(_) => {
            return Err(ContractError::SwapNotFound {});
        }
    };

    SWAPS.remove(deps.storage, swap_id.clone());

    Ok(Response::new().add_attribute("method", "cancel_swap"))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSwap { swap_id } => to_binary(&get_swap(deps, swap_id)?),
    }
}

fn get_swap(deps: Deps, swap_id: String) -> StdResult<SwapResponse> {
    let swap = match SWAPS.load(deps.storage, swap_id) {
        Ok(swap) => swap,
        Err(_) => {
            return Err(StdError::NotFound {
                kind: String::from("Swap"),
            })
        }
    };

    let response = SwapResponse {
        owner: swap.owner,
        collection: swap.collection,
        token_id: swap.token_id,
    };

    Ok(response)
}

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
    fn create_swap() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&coins(2, "token"));
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

        // Create Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::CreateSwap {
            collection: Addr::unchecked("gp_collection"),
            token_id: String::from("123"),
        };
        execute(deps.as_mut(), mock_env(), swapper_info, create_swap_msg).unwrap();

        // Get created Swap
        let get_swap_response = query(deps.as_ref(), mock_env(), get_swap_msg.clone()).unwrap();
        let swap_response: SwapResponse = from_binary(&get_swap_response).unwrap();

        assert_eq!(swap_response.collection, Addr::unchecked("gp_collection"));
        assert_eq!(swap_response.owner, Addr::unchecked("swapper"));
        assert_eq!(swap_response.token_id, String::from("123"));

        Ok(())
    }

    #[test]
    fn cancel_swap() -> Result<(), String> {
        // Initialization
        let mut deps = mock_dependencies(&coins(2, "token"));
        let msg = InstantiateMsg {};
        let info = mock_info("creator", &coins(2, "token"));
        instantiate(deps.as_mut(), mock_env(), info.clone(), msg).unwrap();

        // When Swap does not exists
        // Cancel Swap returns an error
        let cancel_swap_msg = ExecuteMsg::CancelSwap {
            swap_id: String::from("1"),
        };
        match execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            cancel_swap_msg.clone(),
        ) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(err, ContractError::SwapNotFound {}),
        };

        // Create Swap
        let swapper_info = mock_info("swapper", &coins(2, "token"));
        let create_swap_msg = ExecuteMsg::CreateSwap {
            collection: Addr::unchecked("gp_collection"),
            token_id: String::from("123"),
        };
        let swap_created =
            execute(deps.as_mut(), mock_env(), swapper_info, create_swap_msg).unwrap();
        let created_swap_id = swap_created
            .attributes
            .iter()
            .find(move |x| x.key == String::from("swap_id"))
            .unwrap()
            .value
            .clone();

        let get_swap_msg = QueryMsg::GetSwap {
            swap_id: created_swap_id.clone(),
        };

        let get_swap_response = query(deps.as_ref(), mock_env(), get_swap_msg.clone()).unwrap();
        let _: SwapResponse = from_binary(&get_swap_response).unwrap();

        // Cancel Swap
        match execute(
            deps.as_mut(),
            mock_env(),
            info.clone(),
            cancel_swap_msg.clone(),
        ) {
            Ok(_) => (),
            Err(e) => panic!("Unexpected error: {:#?}", e),
        };

        // When Swap is canceled
        // Fetch Swap returns an error
        match query(deps.as_ref(), mock_env(), get_swap_msg.clone()) {
            Ok(_) => panic!("Error expected"),
            Err(err) => assert_eq!(
                err,
                StdError::NotFound {
                    kind: String::from("Swap")
                }
            ),
        };

        Ok(())
    }
}
