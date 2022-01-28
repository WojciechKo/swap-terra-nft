use crate::msg::SwapResponse;
use crate::state::SWAPS;
use cosmwasm_std::{Deps, StdError, StdResult};

pub fn get_swap(deps: Deps, swap_id: String) -> StdResult<SwapResponse> {
    let swap = match SWAPS.load(deps.storage, swap_id) {
        Ok(swap) => swap,
        Err(_) => {
            return Err(StdError::NotFound {
                kind: String::from("Swap"),
            })
        }
    };

    Ok(SwapResponse {
        owner: swap.owner,
        collection: swap.collection,
        token_id: swap.token_id,
    })
}
