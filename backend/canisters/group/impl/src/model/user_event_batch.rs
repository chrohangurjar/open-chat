use timer_job_queues::{TimerJobItem, TimerJobItemGroup};
use types::UserId;
use user_canister::GroupCanisterEvent;

pub struct UserEventBatch {
    user_id: UserId,
    events: Vec<GroupCanisterEvent>,
}

impl TimerJobItemGroup for UserEventBatch {
    type Key = UserId;
    type Item = GroupCanisterEvent;

    fn new(user_id: UserId) -> Self {
        UserEventBatch {
            user_id,
            events: Vec::new(),
        }
    }

    fn key(&self) -> UserId {
        self.user_id
    }

    fn add(&mut self, event: GroupCanisterEvent) {
        self.events.push(event)
    }

    fn into_items(self) -> Vec<GroupCanisterEvent> {
        self.events
    }

    fn is_full(&self) -> bool {
        self.events.len() >= 1000
    }
}

impl TimerJobItem for UserEventBatch {
    async fn process(&self) -> Result<(), bool> {
        let response = user_canister_c2c_client::c2c_notify_group_canister_events(
            self.user_id.into(),
            &user_canister::c2c_notify_group_canister_events::Args {
                events: self.events.clone(),
            },
        )
        .await;

        match response {
            Ok(user_canister::c2c_notify_group_canister_events::Response::Success) => Ok(()),
            Err(_) => Err(true),
        }
    }
}