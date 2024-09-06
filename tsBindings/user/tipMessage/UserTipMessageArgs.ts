// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { Chat } from "../../shared/Chat";
import type { Cryptocurrency } from "../../shared/Cryptocurrency";
import type { MessageId } from "../../shared/MessageId";
import type { MessageIndex } from "../../shared/MessageIndex";
import type { PrincipalTS } from "../../shared/PrincipalTS";
import type { UserId } from "../../shared/UserId";

export type UserTipMessageArgs = { chat: Chat, recipient: UserId, thread_root_message_index?: MessageIndex, message_id: MessageId, ledger: PrincipalTS, token: Cryptocurrency, amount: bigint, fee: bigint, decimals: number, pin?: string, };
