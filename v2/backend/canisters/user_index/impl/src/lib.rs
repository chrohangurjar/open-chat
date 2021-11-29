use crate::model::failed_messages_pending_retry::FailedMessagesPendingRetry;
use crate::model::open_storage_user_sync_queue::OpenStorageUserSyncQueue;
use crate::model::user_map::UserMap;
use candid::{CandidType, Principal};
use canister_logger::LogMessagesWrapper;
use serde::{Deserialize, Serialize};
use std::cell::RefCell;
use std::collections::{HashSet, VecDeque};
use types::{CanisterId, CanisterWasm, ChatId, ConfirmationCodeSms, Cycles, TimestampMillis, Timestamped, UserId, Version};
use utils::canister::CanistersRequiringUpgrade;
use utils::env::Environment;
use utils::event_stream::EventStream;
use utils::{canister, memory};

mod guards;
mod lifecycle;
mod model;
mod queries;
mod updates;

const MIN_CYCLES_BALANCE: Cycles = 5_000_000_000_000; // 5T
const USER_CANISTER_INITIAL_CYCLES_BALANCE: Cycles = 500_000_000_000; // 0.5T cycles
const USER_CANISTER_TOP_UP_AMOUNT: Cycles = 100_000_000_000; // 0.1T cycles
const DEFAULT_OPEN_STORAGE_USER_BYTE_LIMIT: u64 = 100 * 1024 * 1024; // 100 MB
const CONFIRMATION_CODE_EXPIRY_MILLIS: u64 = 60 * 60 * 1000; // 1 hour
const STATE_VERSION: StateVersion = StateVersion::V1;

#[derive(CandidType, Serialize, Deserialize)]
enum StateVersion {
    V1,
}

thread_local! {
    static RUNTIME_STATE: RefCell<Option<RuntimeState>> = RefCell::default();
    static LOG_MESSAGES: RefCell<LogMessagesWrapper> = RefCell::default();
    static WASM_VERSION: RefCell<Timestamped<Version>> = RefCell::default();
}

struct RuntimeState {
    pub env: Box<dyn Environment>,
    pub data: Data,
}

impl RuntimeState {
    pub fn new(env: Box<dyn Environment>, data: Data) -> RuntimeState {
        RuntimeState { env, data }
    }

    /// Traps if the caller is not an OpenChat user or an OpenChat user's canister
    pub fn trap_if_caller_not_open_chat_user(&self) {
        let caller = self.env.caller();

        if !self.data.users.is_valid_caller(caller) {
            #[cfg(not(test))]
            ic_cdk::trap("Not authorized");
        }
    }

    pub fn is_caller_service_principal(&self) -> bool {
        let caller = self.env.caller();
        self.data.service_principals.contains(&caller)
    }

    pub fn is_caller_sms_service(&self) -> bool {
        let caller = self.env.caller();
        self.data.sms_service_principals.contains(&caller)
    }

    pub fn is_caller_notifications_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.notifications_canister_ids.contains(&caller)
    }

    pub fn is_caller_online_users_aggregator_canister(&self) -> bool {
        let caller = self.env.caller();
        self.data.online_users_aggregator_canister_ids.contains(&caller)
    }

    pub fn metrics(&self) -> Metrics {
        let user_metrics = self.data.users.metrics();
        Metrics {
            memory_used: memory::used(),
            now: self.env.now(),
            cycles_balance: self.env.cycles_balance(),
            total_cycles_spent_on_canisters: self.data.total_cycles_spent_on_canisters,
            canisters_in_pool: self.data.canister_pool.len() as u16,
            user_wasm_version: self.data.user_canister_wasm.version,
            users_unconfirmed: user_metrics.users_unconfirmed,
            users_confirmed: user_metrics.users_confirmed,
            users_created: user_metrics.users_created,
            users_online_5_minutes: user_metrics.users_online_5_minutes,
            users_online_1_hour: user_metrics.users_online_1_hour,
            users_online_1_week: user_metrics.users_online_1_week,
            users_online_1_month: user_metrics.users_online_1_month,
            canister_upgrades_in_progress: user_metrics.canister_upgrades_in_progress,
            sms_messages_in_queue: self.data.sms_messages.len() as u32,
            super_admins: self.data.super_admins.len() as u8,
            super_admins_to_dismiss: self.data.super_admins_to_dismiss.len() as u32,
        }
    }
}

#[derive(Serialize, Deserialize)]
struct Data {
    pub users: UserMap,
    pub service_principals: HashSet<Principal>,
    pub user_canister_wasm: CanisterWasm,
    pub sms_service_principals: HashSet<Principal>,
    pub sms_messages: EventStream<ConfirmationCodeSms>,
    pub group_index_canister_id: CanisterId,
    pub notifications_canister_ids: Vec<CanisterId>,
    pub canisters_requiring_upgrade: CanistersRequiringUpgrade,
    pub canister_pool: canister::Pool,
    pub total_cycles_spent_on_canisters: Cycles,
    pub online_users_aggregator_canister_ids: HashSet<CanisterId>,
    #[serde(default = "open_storage_index_canister_id")]
    pub open_storage_index_canister_id: CanisterId,
    #[serde(default)]
    pub open_storage_user_sync_queue: OpenStorageUserSyncQueue,
    pub failed_messages_pending_retry: FailedMessagesPendingRetry,
    pub super_admins: HashSet<UserId>,
    pub super_admins_to_dismiss: VecDeque<(UserId, ChatId)>,
    pub test_mode: bool,
}

fn open_storage_index_canister_id() -> CanisterId {
    Principal::from_text("rturd-qaaaa-aaaaf-aabaq-cai").unwrap()
}

impl Data {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        service_principals: Vec<Principal>,
        sms_service_principals: Vec<Principal>,
        user_canister_wasm: CanisterWasm,
        group_index_canister_id: CanisterId,
        notifications_canister_ids: Vec<CanisterId>,
        online_users_aggregator_canister_id: CanisterId,
        open_storage_index_canister_id: CanisterId,
        canister_pool_target_size: u16,
        test_mode: bool,
    ) -> Self {
        Data {
            users: UserMap::default(),
            service_principals: service_principals.into_iter().collect(),
            user_canister_wasm,
            sms_service_principals: sms_service_principals.into_iter().collect(),
            sms_messages: EventStream::default(),
            group_index_canister_id,
            notifications_canister_ids,
            online_users_aggregator_canister_ids: HashSet::from([online_users_aggregator_canister_id]),
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            canister_pool: canister::Pool::new(canister_pool_target_size),
            total_cycles_spent_on_canisters: 0,
            open_storage_index_canister_id,
            open_storage_user_sync_queue: OpenStorageUserSyncQueue::default(),
            failed_messages_pending_retry: FailedMessagesPendingRetry::default(),
            super_admins: HashSet::new(),
            super_admins_to_dismiss: VecDeque::new(),
            test_mode,
        }
    }
}

#[cfg(test)]
impl Default for Data {
    fn default() -> Data {
        Data {
            users: UserMap::default(),
            service_principals: HashSet::new(),
            user_canister_wasm: CanisterWasm::default(),
            sms_service_principals: HashSet::new(),
            sms_messages: EventStream::default(),
            group_index_canister_id: Principal::anonymous(),
            notifications_canister_ids: vec![Principal::anonymous()],
            canisters_requiring_upgrade: CanistersRequiringUpgrade::default(),
            online_users_aggregator_canister_ids: HashSet::new(),
            canister_pool: canister::Pool::new(5),
            total_cycles_spent_on_canisters: 0,
            open_storage_index_canister_id: Principal::anonymous(),
            open_storage_user_sync_queue: OpenStorageUserSyncQueue::default(),
            failed_messages_pending_retry: FailedMessagesPendingRetry::default(),
            super_admins: HashSet::new(),
            super_admins_to_dismiss: VecDeque::new(),
            test_mode: true,
        }
    }
}

#[derive(CandidType, Serialize, Debug)]
pub struct Metrics {
    pub memory_used: u64,
    pub now: TimestampMillis,
    pub cycles_balance: Cycles,
    pub total_cycles_spent_on_canisters: Cycles,
    pub users_unconfirmed: u32,
    pub users_confirmed: u32,
    pub users_created: u64,
    pub users_online_5_minutes: u32,
    pub users_online_1_hour: u32,
    pub users_online_1_week: u32,
    pub users_online_1_month: u32,
    pub canisters_in_pool: u16,
    pub canister_upgrades_in_progress: u32,
    pub user_wasm_version: Version,
    pub sms_messages_in_queue: u32,
    pub super_admins: u8,
    pub super_admins_to_dismiss: u32,
}
