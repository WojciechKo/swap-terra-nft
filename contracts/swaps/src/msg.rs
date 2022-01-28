use crate::state::SwapSide;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct InstantiateMsg {}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExecuteMsg {
    InitiateSwap {
        collection: String,
        token_id: String,
    },
    SwapReply {
        swap_id: String,
        collection: String,
        token_id: String,
    },
    FinalizeSwap {
        swap_id: String,
    },
    CancelSwap {
        swap_id: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    GetSwap { swap_id: String },
}

// We define a custom struct for each query response
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
pub struct SwapResponse {
    pub lhs: SwapSide,
    pub rhs: Option<SwapSide>,
}
