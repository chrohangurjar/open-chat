// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CommunityExploreChannelsSuccessResult } from "./CommunityExploreChannelsSuccessResult";

export type CommunityExploreChannelsResponse = { "Success": CommunityExploreChannelsSuccessResult } | { "TermTooShort": number } | { "TermTooLong": number } | "InvalidTerm" | "PrivateCommunity";