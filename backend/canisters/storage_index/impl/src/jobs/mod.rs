use crate::RuntimeState;

pub mod ensure_sufficient_active_buckets;
pub mod sync_buckets;
pub mod upgrade_buckets;

pub(crate) fn start(state: &RuntimeState) {
    ensure_sufficient_active_buckets::start_job_if_required(state);
    sync_buckets::start_job_if_required(&state.data);
    upgrade_buckets::start_job_if_required(state);
}