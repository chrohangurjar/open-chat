use candid::CandidType;
use serde::{Deserialize, Serialize};
use types::{BuildVersion, CanisterId};

#[derive(CandidType, Serialize, Deserialize, Debug)]
pub struct Args {
    pub user_index_canister_id: CanisterId,
    pub local_user_index_canister_id: CanisterId,
    pub local_group_index_canister_id: CanisterId,
    pub cycles_dispenser_canister_id: CanisterId,
    pub event_relay_canister_id: CanisterId,
    pub wasm_version: BuildVersion,
    pub public_key: String,
    pub test_mode: bool,
}
