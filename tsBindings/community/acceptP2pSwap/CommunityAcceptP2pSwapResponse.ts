// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { AcceptSwapSuccess } from "../../shared/AcceptSwapSuccess";
import type { SwapStatusError } from "../../shared/SwapStatusError";
import type { TSBigIntWithDefault } from "../../shared/TSBigIntWithDefault";

export type CommunityAcceptP2pSwapResponse = { "Success": AcceptSwapSuccess } | "InsufficientFunds" | { "StatusError": SwapStatusError } | "SwapNotFound" | "ChannelNotFound" | "UserNotInCommunity" | "UserNotInChannel" | "UserSuspended" | "ChatFrozen" | "PinRequired" | { "PinIncorrect": TSBigIntWithDefault } | { "TooManyFailedPinAttempts": TSBigIntWithDefault } | { "InternalError": string } | "UserLapsed";
