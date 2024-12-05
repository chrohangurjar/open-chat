// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { ChannelId } from "../ChannelId";
import type { ChatMetrics } from "./ChatMetrics";
import type { EventIndex } from "./EventIndex";
import type { EventWrapperMessage } from "./EventWrapperMessage";
import type { GroupMembershipUpdates } from "./GroupMembershipUpdates";
import type { GroupPermissions } from "./GroupPermissions";
import type { MessageIndex } from "./MessageIndex";
import type { OptionUpdateAccessGate } from "./OptionUpdateAccessGate";
import type { OptionUpdateAccessGateConfig } from "./OptionUpdateAccessGateConfig";
import type { OptionUpdateGroupSubtype } from "./OptionUpdateGroupSubtype";
import type { OptionUpdateString } from "./OptionUpdateString";
import type { OptionUpdateU128 } from "./OptionUpdateU128";
import type { OptionUpdateU64 } from "./OptionUpdateU64";
import type { OptionUpdateVideoCall } from "./OptionUpdateVideoCall";

export type CommunityCanisterChannelSummaryUpdates = { channel_id: ChannelId, last_updated: bigint, name?: string | undefined, description?: string | undefined, subtype: OptionUpdateGroupSubtype, avatar_id: OptionUpdateU128, is_public?: boolean | undefined, messages_visible_to_non_members?: boolean | undefined, latest_message?: EventWrapperMessage | undefined, latest_message_sender_display_name?: string | undefined, latest_event_index?: EventIndex | undefined, latest_message_index?: MessageIndex | undefined, member_count?: number | undefined, permissions_v2?: GroupPermissions | undefined, updated_events: Array<[MessageIndex | null, EventIndex, bigint]>, metrics?: ChatMetrics | undefined, date_last_pinned?: bigint | undefined, events_ttl: OptionUpdateU64, events_ttl_last_updated?: bigint | undefined, gate: OptionUpdateAccessGate, gate_config: OptionUpdateAccessGateConfig, membership?: GroupMembershipUpdates | undefined, video_call_in_progress: OptionUpdateVideoCall, external_url: OptionUpdateString, any_updates_missed: boolean, };
