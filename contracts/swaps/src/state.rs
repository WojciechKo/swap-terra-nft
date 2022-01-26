use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

use cosmwasm_std::Addr;
use cw_storage_plus::{Item, Map};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Swap {
    pub owner: Addr,
    pub collection: Addr,
    pub token_id: String,
}

pub const SWAPS: Map<String, Swap> = Map::new("swaps");

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct Config {
    pub next_swap_id: u32,
}
pub const CONFIG: Item<Config> = Item::new("config");
