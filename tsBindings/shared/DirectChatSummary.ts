// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ChatMetrics } from "./ChatMetrics";
import type { EventIndex } from "./EventIndex";
import type { EventWrapperMessage } from "./EventWrapperMessage";
import type { MessageIndex } from "./MessageIndex";
import type { UserId } from "./UserId";
import type { VideoCall } from "./VideoCall";

export type DirectChatSummary = { them: UserId, last_updated: bigint, latest_message: EventWrapperMessage, latest_event_index: EventIndex, latest_message_index: MessageIndex, date_created: bigint, read_by_me_up_to?: MessageIndex, read_by_them_up_to?: MessageIndex, notifications_muted: boolean, metrics: ChatMetrics, my_metrics: ChatMetrics, archived: boolean, events_ttl?: bigint, events_ttl_last_updated: bigint, video_call_in_progress?: VideoCall, };