// This file was generated by [ts-rs](https://github.com/Aleph-Alpha/ts-rs). Do not edit this file manually.
import type { CustomPermission } from "./CustomPermission";
import type { GroupPermissionRole } from "./GroupPermissionRole";

export type MessagePermissions = { default: GroupPermissionRole, text?: GroupPermissionRole, image?: GroupPermissionRole, video?: GroupPermissionRole, audio?: GroupPermissionRole, file?: GroupPermissionRole, poll?: GroupPermissionRole, crypto?: GroupPermissionRole, giphy?: GroupPermissionRole, prize?: GroupPermissionRole, p2p_swap?: GroupPermissionRole, video_call?: GroupPermissionRole, custom: Array<CustomPermission>, };