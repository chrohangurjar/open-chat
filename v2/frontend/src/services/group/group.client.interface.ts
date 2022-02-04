import type { User } from "../../domain/user/user";
import type {
    AddParticipantsResponse,
    EventsResponse,
    GroupChatEvent,
    SendMessageResponse,
    RemoveParticipantResponse,
    UpdateGroupResponse,
    ToggleReactionResponse,
    IndexRange,
    EventWrapper,
    Message,
    DeleteMessageResponse,
    EditMessageResponse,
    BlockUserResponse,
    MakeAdminResponse,
    DismissAdminResponse,
    GroupChatDetails,
    GroupChatDetailsResponse,
    UnblockUserResponse,
    TransferOwnershipResponse,
    DeleteGroupResponse,
    GroupChatSummary,
} from "../../domain/chat/chat";

export interface IGroupClient {
    chatEventsByIndex(eventIndexes: number[]): Promise<EventsResponse<GroupChatEvent>>;
    chatEventsWindow(
        eventIndexRange: IndexRange,
        messageIndex: number
    ): Promise<EventsResponse<GroupChatEvent>>;
    chatEvents(
        eventIndexRange: IndexRange,
        startIndex: number,
        ascending: boolean,
        previouslyLoadedEvents?: EventWrapper<GroupChatEvent>[],
        iterations?: number
    ): Promise<EventsResponse<GroupChatEvent>>;
    addParticipants(
        userIds: string[],
        myUsername: string,
        allowBlocked: boolean
    ): Promise<AddParticipantsResponse>;
    sendMessage(
        senderName: string,
        mentioned: User[],
        message: Message
    ): Promise<SendMessageResponse>;
    editMessage(message: Message): Promise<EditMessageResponse>;
    makeAdmin(userId: string): Promise<MakeAdminResponse>;
    dismissAsAdmin(userId: string): Promise<DismissAdminResponse>;
    removeParticipant(userId: string): Promise<RemoveParticipantResponse>;
    updateGroup(
        name: string,
        desc: string,
        joinAsViewer: boolean,
        avatar?: Uint8Array
    ): Promise<UpdateGroupResponse>;
    toggleReaction(messageId: bigint, reaction: string): Promise<ToggleReactionResponse>;
    deleteMessage(messageId: bigint): Promise<DeleteMessageResponse>;
    blockUser(userId: string): Promise<BlockUserResponse>;
    unblockUser(userId: string): Promise<UnblockUserResponse>;
    getGroupDetails(): Promise<GroupChatDetailsResponse>;
    getGroupDetailsUpdates(previous: GroupChatDetails): Promise<GroupChatDetails>;
    transferOwnership(userId: string): Promise<TransferOwnershipResponse>;
    deleteGroup(): Promise<DeleteGroupResponse>;
    getPublicSummary(): Promise<GroupChatSummary | undefined>;
}
