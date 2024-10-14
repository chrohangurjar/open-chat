// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CryptoTransaction } from "./CryptoTransaction";
import type { TSBigIntWithDefault } from "./TSBigIntWithDefault";
import type { TSBoolWithDefault } from "./TSBoolWithDefault";

export type PrizeContentInitial = { 
/**
 * @default []
 */
prizes_v2: Array<bigint>, transfer: CryptoTransaction, end_date: TSBigIntWithDefault, caption?: string | undefined, diamond_only: TSBoolWithDefault, };
