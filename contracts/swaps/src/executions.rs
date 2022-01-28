use cosmwasm_std::{to_binary, CosmosMsg, DepsMut, Env, MessageInfo, Response, WasmMsg};
use cw2::set_contract_version;
use cw721::Cw721ExecuteMsg::TransferNft;

use crate::error::ContractError;
use crate::msg::InstantiateMsg;
use crate::state::Config;
use crate::state::{Swap, CONFIG, SWAPS};

const CONTRACT_NAME: &str = "crates.io:swaps";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");

pub fn initialize(
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

pub fn create_swap(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    collection: String,
    token_id: String,
) -> Result<Response, ContractError> {
    let collection = match deps.api.addr_validate(&collection) {
        Ok(collection) => collection,
        Err(_) => {
            return Err(ContractError::InvalidAddress {
                address: collection,
            })
        }
    };

    let swap = Swap {
        owner: info.sender.clone(),
        collection: collection.clone(),
        token_id: token_id.clone(),
    };

    let swap_id = CONFIG.load(deps.storage).unwrap().next_swap_id;
    SWAPS.save(deps.storage, swap_id.to_string(), &swap)?;

    CONFIG.update(deps.storage, |mut state| -> Result<_, ContractError> {
        state.next_swap_id += 1;
        Ok(state)
    })?;

    Ok(Response::new()
        .add_attribute("method", "create_swap")
        .add_attribute("swap_id", swap_id.to_string())
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: swap.collection.to_string(),
            funds: vec![],
            msg: to_binary(&TransferNft {
                recipient: env.contract.address.to_string(),
                token_id: token_id.clone(),
            })?,
        })]))
}

pub fn cancel_swap(
    deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    swap_id: String,
) -> Result<Response, ContractError> {
    let swap = match SWAPS.load(deps.storage, swap_id.clone()) {
        Ok(swap) => swap,
        Err(_) => {
            return Err(ContractError::SwapNotFound {});
        }
    };

    SWAPS.remove(deps.storage, swap_id.clone());

    Ok(Response::new()
        .add_attribute("method", "cancel_swap")
        .add_messages(vec![CosmosMsg::Wasm(WasmMsg::Execute {
            contract_addr: swap.collection.to_string(),
            funds: vec![],
            msg: to_binary(&TransferNft {
                recipient: swap.owner.to_string(),
                token_id: swap.token_id.clone(),
            })?,
        })]))
}
