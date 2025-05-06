<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import {
        MessageCircle,
        Search,
        User,
        Calendar,
        ChevronRight,
    } from "lucide-svelte";
    import { user } from "$lib/userStore";

    export let data;
    console.log("data", data);

    type Chat = {
        chat_id: number;
        advert_id: number;
        participant_id: number;
        deal_state: string;
        archived: boolean;
        chat_created_at: string;
        chat_updated_at: string;
        advert_owner_id: number;
        advert_created_at: string;
        advert_updated_at: string;
        available: boolean;
        price: number;
        photo_url: string; // Advert photo URL
        lat: number;
        lon: number;
        additional_photos: string[] | null;
        title: string; // Advert title/name
        category: string;
        description: string;
        sold_to: number | null;
        old_price: number | null;
        owner_id: number | null;
        owner_created_at: string;
        owner_updated_at: string;
        owner_avatar_url: string | null;
        owner_name: string;
        owner_surname: string;
        owner_company_name: string | null;
        owner_email: string;
        owner_phone: string | null;
        owner_telegram_id: string | null;
        owner_telegram_username: string | null;
        owner_balance: number;
        owner_email_verified: boolean;
        owner_role: string;
        participant_created_at: string;
        participant_updated_at: string;
        participant_avatar_url: string | null;
        participant_name: string;
        participant_surname: string;
        participant_company_name: string | null;
        participant_email: string;
        participant_phone: string | null;
        participant_telegram_id: string | null;
        participant_telegram_username: string | null;
        participant_balance: number;
        participant_email_verified: boolean;
        participant_role: string;
        deal_id: number | null;
        deal_chat_id: number | null;
        deal_price: number | null;
        deal_created_at: string | null;
        last_message?: string;
        last_message_time?: string;
        unread_count?: number;
    };

    let chats: Chat[] = data.chats || [];
    let loading = false;
    let error: string | null = null;
    let searchQuery = "";

    $: filteredChats = chats.filter(
        (chat) =>
            chat.title.toLowerCase().includes(searchQuery.toLowerCase()) ||
            chat.participant_name
                .toLowerCase()
                .includes(searchQuery.toLowerCase()) ||
            chat.participant_surname
                .toLowerCase()
                .includes(searchQuery.toLowerCase()),
    );

    $: groupedChats = Object.values(
        filteredChats.reduce(
            (acc: Record<number, { advert: Chat; chats: Chat[] }>, chat) => {
                if (!acc[chat.advert_id]) {
                    acc[chat.advert_id] = { advert: chat, chats: [chat] };
                } else {
                    acc[chat.advert_id].chats.push(chat);
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

    function isCurrentUserOwner(chat: Chat): boolean {
        return $user && $user.id === chat.advert_owner_id;
    }

    function getOtherUserInfo(chat: Chat) {
        if (isCurrentUserOwner(chat)) {
            return {
                name: chat.participant_name,
                surname: chat.participant_surname,
                avatar: chat.participant_avatar_url,
            };
        } else {
            return {
                name: chat.owner_name,
                surname: chat.owner_surname,
                avatar: chat.owner_avatar_url,
            };
        }
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
                    {#each groupedChats as group (group.advert.advert_id)}
                        {@const groupClosed = group.chats.some(
                            (chat) =>
                                chat.deal_state === "accepted" || chat.archived,
                        )}
                        <div
                            class="mb-8 block border border-gray-200 dark:border-gray-700 rounded-lg overflow-hidden shadow-sm"
                        >
                            <div
                                class="flex items-center p-4 bg-gray-50 dark:bg-gray-800"
                            >
                                {#if group.advert.photo_url}
                                    <a
                                        href={`/advert/${group.advert.advert_id}`}
                                    >
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
                                            >
                                                Closed
                                            </span>
                                        {/if}
                                    </h2>
                                </div>
                            </div>
                            <!-- Chats list for this advert -->
                            <ul
                                class="divide-y divide-gray-200 dark:divide-gray-700"
                            >
                                {#each group.chats as chat (chat.chat_id)}
                                    {@const otherUser = getOtherUserInfo(chat)}
                                    <li
                                        class="flex items-center justify-between p-4 hover:bg-gray-50 dark:hover:bg-gray-750 cursor-pointer transition duration-150 ease-in-out"
                                        on:click={() =>
                                            navigateToChat(chat.chat_id)}
                                    >
                                        <div class="flex items-center">
                                            {#if otherUser.avatar}
                                                <img
                                                    src={otherUser.avatar ||
                                                        "/placeholder.svg"}
                                                    alt={`${otherUser.name} ${otherUser.surname}`}
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
                                                    {otherUser.name}
                                                    {otherUser.surname}
                                                </p>
                                                {#if chat.last_message}
                                                    <p
                                                        class="text-xs text-gray-500 dark:text-gray-400"
                                                    >
                                                        {chat.last_message}
                                                    </p>
                                                {/if}
                                            </div>
                                        </div>
                                        <div class="flex flex-col items-end">
                                            <p
                                                class="text-xs text-gray-500 dark:text-gray-400 flex items-center"
                                            >
                                                <Calendar
                                                    class="h-3 w-3 mr-1"
                                                />
                                                {formatDate(
                                                    chat.chat_updated_at,
                                                )}
                                            </p>
                                            {#if chat.unread_count && chat.unread_count > 0}
                                                <span
                                                    class="mt-1 inline-flex items-center justify-center px-2 py-1 text-xs font-bold leading-none text-white bg-blue-500 rounded-full"
                                                >
                                                    {chat.unread_count}
                                                </span>
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
