<script lang="ts">
    import { goto } from "$app/navigation";
    import {
        MessageCircle,
        Search,
        User,
        Calendar,
        ChevronRight,
        Check,
        CheckCheck,
    } from "lucide-svelte";
    import { user } from "$lib/userStore";
    import { getContext, onMount } from "svelte";
    import { socket } from "$lib/socket.js";
    import type { Writable } from "$houdini";

    export let data;

    type StructuredChat = {
        chat: {
            id: number;
            advert_id: number;
            participant_id: number;
            archived: boolean;
            created_at: string;
            updated_at: string;
            last_message?: {
                id: number;
                content: string;
                chat_id: number;
                user_id: number;
                created_at: string;
                read_at: string | null;
            };
        };
        advert: {
            id: number;
            title: string;
            photo_url: string | null;
            owner: {
                id: number;
                name: string;
                surname: string;
                avatar_url: string | null;
            };
        };
        participant: {
            id: number;
            name: string;
            surname: string;
            avatar_url: string | null;
        };
        deal: any;
    };

    let chats: StructuredChat[] = data.chats || [];
    let loading = false;
    let error: string | null = null;
    let searchQuery = "";

    interface UnreadMessages {
        unreadMessages: number;
    }
    const areUnreadMessages: Writable<UnreadMessages> =
        getContext("areUnreadMessages");

    onMount(() => {
        function handleNewMessage(data: any) {
            const chat = chats.find((chat) => chat.chat.id === data.chat_id);

            if (chat) {
                chat.chat.last_message = data;
                chat.chat.updated_at = data.created_at;
            }

            chats = chats.map((chat) => {
                if (chat.chat.id === data.chat_id) {
                    return {
                        ...chat,
                        chat: {
                            ...chat.chat,
                            last_message: data,
                            updated_at: data.created_at,
                        },
                    };
                }
                return chat;
            });

            areUnreadMessages.update((prev) => ({
                unreadMessages: prev.unreadMessages + 1,
            }));
        }

        socket.on("user-" + $user.id, handleNewMessage);

        return () => {
            socket.off("user-" + $user.id, handleNewMessage);
        };
    });

    $: filteredChats = chats.filter((item) => {
        const title = item.advert.title.toLowerCase();
        const partName = item.participant.name.toLowerCase();
        const partSurname = item.participant.surname.toLowerCase();
        const q = searchQuery.toLowerCase();

        areUnreadMessages.set({
            unreadMessages: chats.filter(
                (chat) =>
                    chat.chat.last_message &&
                    chat.chat.last_message.read_at === null &&
                    chat.chat.last_message.user_id !== $user?.id,
            ).length,
        });
        return (
            title.includes(q) || partName.includes(q) || partSurname.includes(q)
        );
    });

    $: groupedChats = Object.values(
        filteredChats.reduce(
            (
                acc: Record<
                    number,
                    {
                        advert: StructuredChat["advert"];
                        chats: StructuredChat[];
                    }
                >,
                item,
            ) => {
                if (!acc[item.advert.id]) {
                    acc[item.advert.id] = {
                        advert: item.advert,
                        chats: [item],
                    };
                } else {
                    acc[item.advert.id].chats.push(item);
                }
                return acc;
            },
            {},
        ),
    );

    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        const now = new Date();

        if (date.toDateString() === now.toDateString()) {
            return date.toLocaleTimeString([], {
                hour: "2-digit",
                minute: "2-digit",
            });
        }

        if (date.getFullYear() === now.getFullYear()) {
            return date.toLocaleDateString([], {
                month: "short",
                day: "numeric",
            });
        }

        return date.toLocaleDateString([], {
            year: "numeric",
            month: "short",
            day: "numeric",
        });
    }

    function navigateToChat(chatId: number) {
        goto(`/chats/${chatId}`);
    }

    function isCurrentUserOwner(item: StructuredChat): boolean {
        return $user?.id === item.advert.owner.id;
    }

    function getOtherUserInfo(item: StructuredChat) {
        if (isCurrentUserOwner(item)) {
            return item.participant;
        } else {
            return {
                id: item.advert.owner.id,
                name: item.advert.owner.name,
                surname: item.advert.owner.surname,
                avatar_url: item.advert.owner.avatar_url,
            };
        }
    }

    function isLastMessageFromCurrentUser(item: StructuredChat): boolean {
        if (!item.chat.last_message) return false;
        return item.chat.last_message.user_id === $user?.id;
    }

    function getLastMessageSenderName(item: StructuredChat): string {
        if (!item.chat.last_message) return "";

        const isFromCurrentUser = isLastMessageFromCurrentUser(item);
        if (isFromCurrentUser) {
            return "You";
        } else {
            const other = getOtherUserInfo(item);
            return other.name;
        }
    }

    function isLastMessageUnread(item: StructuredChat): boolean {
        if (!item.chat.last_message) return false;

        if (isLastMessageFromCurrentUser(item)) return false;

        return item.chat.last_message.read_at === null;
    }
</script>

<div
    class="min-h-screen bg-gray-100 dark:bg-gray-900 py-12 px-4 sm:px-6 lg:px-8"
>
    <div class="max-w-4xl mx-auto">
        <div
            class="bg-white dark:bg-gray-800 shadow-lg rounded-lg overflow-hidden"
        >
            <div class="p-6">
                <div class="flex justify-between items-center mb-6">
                    <h1
                        class="text-2xl font-bold text-gray-900 dark:text-white"
                    >
                        My Chats
                    </h1>
                    <div class="relative">
                        <div
                            class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
                        >
                            <Search class="h-5 w-5 text-gray-400" />
                        </div>
                        <input
                            type="text"
                            bind:value={searchQuery}
                            placeholder="Search adverts or chats..."
                            class="pl-10 pr-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />
                    </div>
                </div>

                {#if loading}
                    <div class="flex justify-center items-center py-12">
                        <div
                            class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"
                        ></div>
                    </div>
                {:else if error}
                    <div
                        class="bg-red-100 dark:bg-red-900 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded relative"
                        role="alert"
                    >
                        <strong class="font-bold">Error!</strong>
                        <span class="block sm:inline"> {error}</span>
                    </div>
                {:else if groupedChats.length === 0}
                    <div class="text-center py-12">
                        <MessageCircle
                            class="h-16 w-16 text-gray-400 mx-auto mb-4"
                        />
                        <h3
                            class="text-lg font-medium text-gray-900 dark:text-white"
                        >
                            No chats found
                        </h3>
                        <p class="mt-1 text-gray-500 dark:text-gray-400">
                            {searchQuery
                                ? "No chats match your search criteria."
                                : "You don't have any active chats yet."}
                        </p>
                    </div>
                {:else}
                    {#each groupedChats as group (group.advert.id)}
                        {@const groupClosed = group.chats.some(
                            (item) =>
                                item.deal?.status === "accepted" ||
                                item.chat.archived,
                        )}
                        <div
                            class="mb-8 block border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden shadow-sm"
                        >
                            <div
                                class="flex items-center p-4 bg-gray-50 dark:bg-gray-800"
                            >
                                {#if group.advert.photo_url}
                                    <a href={`/advert/${group.advert.id}`}>
                                        <img
                                            src={group.advert.photo_url}
                                            alt={group.advert.title}
                                            class="h-16 w-16 rounded object-cover mr-4"
                                        />
                                    </a>
                                {:else}
                                    <div
                                        class="h-16 w-16 rounded bg-gray-300 dark:bg-gray-600 flex items-center justify-center mr-4"
                                    >
                                        <User
                                            class="h-8 w-8 text-gray-600 dark:text-gray-300"
                                        />
                                    </div>
                                {/if}
                                <div class="flex-1">
                                    <h2
                                        class="text-xl font-semibold text-gray-900 dark:text-white"
                                    >
                                        {group.advert.title}
                                        {#if groupClosed}
                                            <span
                                                class="ml-2 px-2 py-1 text-xs font-bold text-white bg-red-500 rounded"
                                                >Closed</span
                                            >
                                        {/if}
                                    </h2>
                                </div>
                            </div>
                            <ul
                                class="divide-y divide-gray-200 dark:divide-gray-700"
                            >
                                {#each group.chats as item (item.chat.id)}
                                    {@const other = getOtherUserInfo(item)}
                                    {@const isUnread =
                                        isLastMessageUnread(item)}
                                    <li
                                        class="flex items-center justify-between p-4 hover:bg-gray-50 dark:hover:bg-gray-750 cursor-pointer transition duration-150 ease-in-out"
                                        class:bg-blue-50={isUnread}
                                        class:dark:bg-blue-900={isUnread}
                                        class:dark:bg-opacity-20={isUnread}
                                        on:click={() =>
                                            navigateToChat(item.chat.id)}
                                    >
                                        <div class="flex items-center">
                                            {#if other.avatar_url}
                                                <img
                                                    src={other.avatar_url}
                                                    alt={`${other.name} ${other.surname}`}
                                                    class="h-12 w-12 rounded-full object-cover mr-4"
                                                />
                                            {:else}
                                                <div
                                                    class="h-12 w-12 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center mr-4"
                                                >
                                                    <User
                                                        class="h-6 w-6 text-gray-600 dark:text-gray-300"
                                                    />
                                                </div>
                                            {/if}
                                            <div>
                                                <p
                                                    class="text-sm font-medium text-gray-900 dark:text-white"
                                                >
                                                    {other.name}
                                                    {other.surname}
                                                </p>
                                                {#if item.chat.last_message}
                                                    <p
                                                        class="text-xs text-gray-500 dark:text-gray-400 flex items-center"
                                                    >
                                                        <span
                                                            class="font-medium mr-1"
                                                        >
                                                            {getLastMessageSenderName(
                                                                item,
                                                            )}:
                                                        </span>
                                                        <span
                                                            class:font-semibold={isUnread}
                                                            class:text-gray-800={isUnread}
                                                            class:dark:text-gray-200={isUnread}
                                                        >
                                                            {item.chat
                                                                .last_message
                                                                .content
                                                                .length > 30
                                                                ? item.chat.last_message.content.substring(
                                                                      0,
                                                                      30,
                                                                  ) + "..."
                                                                : item.chat
                                                                      .last_message
                                                                      .content}
                                                        </span>
                                                    </p>
                                                {/if}
                                            </div>
                                        </div>
                                        <div class="flex flex-col items-end">
                                            {#if item.chat.last_message}
                                                <p
                                                    class="text-xs text-gray-500 dark:text-gray-400 flex items-center"
                                                >
                                                    <Calendar
                                                        class="h-3 w-3 mr-1"
                                                    />
                                                    {formatDate(
                                                        item.chat.last_message
                                                            .created_at,
                                                    )}
                                                </p>
                                                {#if isLastMessageFromCurrentUser(item)}
                                                    <div
                                                        class="text-xs text-gray-500 dark:text-gray-400 mt-1 flex items-center"
                                                    >
                                                        {#if item.chat.last_message.read_at}
                                                            <CheckCheck
                                                                class="h-4 w-4 text-blue-500"
                                                            />
                                                            <span
                                                                class="ml-1 text-xs"
                                                                >Read</span
                                                            >
                                                        {:else}
                                                            <Check
                                                                class="h-4 w-4"
                                                            />
                                                            <span
                                                                class="ml-1 text-xs"
                                                                >Sent</span
                                                            >
                                                        {/if}
                                                    </div>
                                                {:else if isUnread}
                                                    <div
                                                        class="mt-1 px-2 py-1 bg-blue-500 text-white text-xs rounded-full"
                                                    >
                                                        New
                                                    </div>
                                                {/if}
                                            {:else}
                                                <p
                                                    class="text-xs text-gray-500 dark:text-gray-400 flex items-center"
                                                >
                                                    <Calendar
                                                        class="h-3 w-3 mr-1"
                                                    />
                                                    {formatDate(
                                                        item.chat.updated_at,
                                                    )}
                                                </p>
                                            {/if}
                                            <ChevronRight
                                                class="h-5 w-5 text-gray-400 mt-1"
                                            />
                                        </div>
                                    </li>
                                {/each}
                            </ul>
                        </div>
                    {/each}
                {/if}
            </div>
        </div>
    </div>
</div>
