use crate::guards::caller_is_user_controller;
use crate::model::bucket_sync_state::EventToSync;
use crate::{mutate_state, RuntimeState};
use canister_tracing_macros::trace;
use ic_cdk::update;
use storage_index_canister::remove_user::*;

#[update(guard = "caller_is_user_controller")]
#[trace]
fn remove_user(args: Args) -> Response {
    mutate_state(|state| remove_user_impl(args, state))
}

fn remove_user_impl(args: Args, state: &mut RuntimeState) -> Response {
    state.data.users.remove(&args.user_id);
    state.push_event_to_buckets(EventToSync::UserRemoved(args.user_id));
    Response::Success
}
