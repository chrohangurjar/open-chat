// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { EventIndex } from "./EventIndex";
import type { MessageIndex } from "./MessageIndex";

export type ThreadSyncDetails = { root_message_index: MessageIndex, latest_event?: EventIndex, latest_message?: MessageIndex, read_up_to?: MessageIndex, last_updated: bigint, };