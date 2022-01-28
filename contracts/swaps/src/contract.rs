use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult,
};

use crate::error::ContractError;
use crate::executions::{cancel_swap, finalize_swap, initialize, initiate_swap, swap_reply};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
use crate::queries::get_swap;

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    initialize(deps, env, info, msg)
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::InitiateSwap {
            collection,
            token_id,
        } => initiate_swap(deps, env, info, collection, token_id),
        ExecuteMsg::SwapReply {
            swap_id,
            collection,
            token_id,
        } => swap_reply(deps, env, info, swap_id, collection, token_id),
        ExecuteMsg::FinalizeSwap { swap_id } => finalize_swap(deps, env, info, swap_id),
        ExecuteMsg::CancelSwap { swap_id } => cancel_swap(deps, env, info, swap_id),
    }
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetSwap { swap_id } => to_binary(&get_swap(deps, swap_id)?),
    }
}
