<script lang="ts">
    import { onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { user } from "$lib/userStore";
    import { get } from "svelte/store";
    import { fade, slide } from "svelte/transition";
    import {
        Send,
        ArrowLeft,
        User,
        Image,
        MessageCircle,
        DollarSign,
        X,
        Check,
        ChevronDown,
        MoreVertical,
    } from "lucide-svelte";
    import { enhance } from "$app/forms";
    import { socket } from "$lib/socket";
    import { page } from "$app/stores";

    export let data;

    let advert = data.advert;
    let partner = data.partner;
    let deal = data.deal;
    let messages = data.messages || [];
    let loading = false;
    let error: string | null = null;
    let messageInput = "";
    let messageContainer: HTMLElement;
    let showDealModal = false;
    let dealPrice: number | null = null;
    let showAdditionalInfo = false;
    let showActionsMenu = false;

    let chat = data.chat || {};
    const chatClosed = (deal && deal.state === "accepted") || chat.archived;

    function scrollToBottom() {
        setTimeout(() => {
            if (messageContainer) {
                messageContainer.scrollTop = messageContainer.scrollHeight;
            }
        }, 0);
    }
    async function markMessagesAsRead() {
        if (!advert || !messages.length) return;
        const unreadMessages = messages.filter(
            (msg: { read: any; sender_id: number }) =>
                !msg.read && msg.sender_id !== $user.id,
        );
        if (unreadMessages.length === 0) return;
        messages = messages.map((msg: { sender_id: number }) => {
            if (msg.sender_id !== $user.id) {
                return { ...msg, read: true };
            }
            return msg;
        });
    }
    function openDealModal() {
        dealPrice = advert ? advert.price : null;
        showDealModal = true;
    }
    function closeDealModal() {
        showDealModal = false;
        dealPrice = null;
    }

    function formatTime(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleTimeString([], {
            hour: "2-digit",
            minute: "2-digit",
        });
    }
    function formatDate(dateString: string): string {
        const date = new Date(dateString);
        return date.toLocaleDateString([], {
            weekday: "long",
            month: "long",
            day: "numeric",
        });
    }
    function shouldShowDate(index: number): boolean {
        if (index === 0) return true;
        const currentDate = new Date(messages[index].created_at).toDateString();
        const prevDate = new Date(
            messages[index - 1].created_at,
        ).toDateString();
        return currentDate !== prevDate;
    }
    function goBack() {
        goto("/chats");
    }
    function getOtherUserInfo() {
        return {
            name: partner.name,
            surname: partner.surname,
            avatar: partner.avatar_url,
        };
    }
    function toggleAdditionalInfo() {
        showAdditionalInfo = !showAdditionalInfo;
    }
    function toggleActionsMenu() {
        showActionsMenu = !showActionsMenu;
    }
    function formatCurrency(amount: number): string {
        return new Intl.NumberFormat("en-US", {
            style: "currency",
            currency: "USD",
            minimumFractionDigits: 0,
            maximumFractionDigits: 0,
        }).format(amount);
    }
    onMount(() => {
        markMessagesAsRead();
        scrollToBottom();
        if (advert) {
            socket.on("chat-" + $page.params.id, (newMessage) => {
                messages = [...messages, newMessage];
                console.log(
                    "New message received:",
                    newMessage,
                    $user.id,
                    messages,
                );
                scrollToBottom();
            });
            socket.on("deal-" + $page.params.id, (data) => {
                console.log("Deal data received:", data);
                deal = data;
                if (data) {
                    showDealModal = false;
                }
            });
        }
    });
</script>

<div class="min-h-screen bg-gray-100 dark:bg-gray-900 flex flex-col">
    <div
        class="max-w-4xl mx-auto w-full h-screen flex flex-col bg-white dark:bg-gray-800 shadow-lg"
    >
        {#if loading}
            <div class="flex-1 flex justify-center items-center">
                <div
                    class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"
                ></div>
            </div>
        {:else if error}
            <div class="flex-1 flex justify-center items-center">
                <div
                    class="bg-red-100 dark:bg-red-900 border border-red-400 dark:border-red-700 text-red-700 dark:text-red-300 px-4 py-3 rounded relative"
                    role="alert"
                >
                    <strong class="font-bold">Error!</strong>
                    <span class="block sm:inline"> {error}</span>
                </div>
            </div>
        {:else if advert}
            {@const otherUser = getOtherUserInfo()}
            <div class="bg-white dark:bg-gray-800 shadow-md p-4 flex flex-col">
                <div class="flex items-center justify-between">
                    <div class="flex items-center">
                        <button
                            on:click={goBack}
                            class="mr-4 text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white"
                        >
                            <ArrowLeft class="h-6 w-6" />
                        </button>
                        <div class="flex-shrink-0 mr-4">
                            {#if otherUser.avatar}
                                <img
                                    src={otherUser.avatar || "/placeholder.svg"}
                                    alt={`${otherUser.name} ${otherUser.surname}`}
                                    class="h-12 w-12 rounded-full object-cover border-2 border-gray-200 dark:border-gray-700"
                                />
                            {:else}
                                <div
                                    class="h-12 w-12 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center"
                                >
                                    <User
                                        class="h-6 w-6 text-gray-600 dark:text-gray-300"
                                    />
                                </div>
                            {/if}
                        </div>
                        <div class="flex-1">
                            <h2
                                class="font-medium text-gray-900 dark:text-white text-lg"
                            >
                                {otherUser.name}
                                {otherUser.surname}
                            </h2>
                            <p class="text-sm text-gray-500 dark:text-gray-400">
                                Last active: {formatTime(partner.updated_at)}
                            </p>
                        </div>
                    </div>
                    <div class="relative">
                        <button
                            on:click={toggleActionsMenu}
                            class="p-2 rounded-full hover:bg-gray-100 dark:hover:bg-gray-700"
                        >
                            <MoreVertical
                                class="h-5 w-5 text-gray-600 dark:text-gray-300"
                            />
                        </button>
                        {#if showActionsMenu}
                            <div
                                transition:fade={{ duration: 150 }}
                                class="absolute right-0 mt-2 w-48 bg-white dark:bg-gray-800 rounded-md shadow-lg z-10 border border-gray-200 dark:border-gray-700"
                            >
                                <div class="py-1">
                                    <button
                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                                        on:click={toggleActionsMenu}
                                        >Report user</button
                                    >
                                    <button
                                        class="w-full text-left px-4 py-2 text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-100 dark:hover:bg-gray-700"
                                        on:click={toggleActionsMenu}
                                        >Block user</button
                                    >
                                </div>
                            </div>
                        {/if}
                    </div>
                </div>
                <div
                    class="mt-4 flex items-center justify-between border-t border-gray-200 dark:border-gray-700 pt-4"
                >
                    <div class="flex items-center">
                        <img
                            src={advert.photo_url || "/placeholder.svg"}
                            alt={advert.title}
                            class="h-16 w-16 object-cover rounded-md mr-3 border border-gray-200 dark:border-gray-700"
                        />
                        <div>
                            <h3
                                class="font-medium text-gray-900 dark:text-white"
                            >
                                {advert.title}
                            </h3>
                            <div class="flex items-center mt-1">
                                <span
                                    class="text-lg font-bold text-green-600 dark:text-green-400"
                                    >{formatCurrency(advert.price)}</span
                                >
                                {#if advert.old_price}
                                    <span
                                        class="ml-2 text-sm line-through text-gray-500 dark:text-gray-400"
                                        >{formatCurrency(
                                            advert.old_price,
                                        )}</span
                                    >
                                {/if}
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center">
                        <button
                            on:click={toggleAdditionalInfo}
                            class="flex items-center text-sm text-blue-600 dark:text-blue-400 hover:underline"
                        >
                            Details
                            <ChevronDown
                                class={`h-4 w-4 ml-1 transition-transform ${showAdditionalInfo ? "transform rotate-180" : ""}`}
                            />
                        </button>
                    </div>
                </div>
                {#if showAdditionalInfo}
                    <div
                        transition:slide={{ duration: 200 }}
                        class="mt-3 p-3 bg-gray-100 dark:bg-gray-700 rounded-md text-sm"
                    >
                        <p class="text-gray-800 dark:text-gray-200 mb-2">
                            {advert.description}
                        </p>
                        <div class="flex justify-between">
                            <span class="text-gray-600 dark:text-gray-400">
                                Category: {advert.category}
                            </span>
                            <span class="text-gray-600 dark:text-gray-400">
                                Listed: {new Date(
                                    advert.created_at,
                                ).toLocaleDateString()}
                            </span>
                        </div>
                    </div>
                {/if}
                {#if deal}
                    {#if deal.status === "pending" && deal.requester_id !== $user.id}
                        <div
                            class="mt-3 bg-yellow-50 border rounded p-4 text-center"
                        >
                            <p class="font-medium">
                                Deal Offer: {formatCurrency(deal.price)}
                            </p>
                            <div class="mt-3 flex justify-center space-x-3">
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={$page.params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="price"
                                        value={deal.price}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="accept"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-3 py-1 bg-green-600 hover:bg-green-700 text-white rounded-md text-sm"
                                    >
                                        <Check class="h-4 w-4 mr-1" /> Accept
                                    </button>
                                </form>
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={$page.params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="price"
                                        value={deal.price}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="decline"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-3 py-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md text-sm text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                                    >
                                        <X class="h-4 w-4 mr-1" /> Decline
                                    </button>
                                </form>
                            </div>
                        </div>
                    {:else if deal.status === "pending" && deal.requester_id === $user.id}
                        <div
                            class="mt-3 bg-gray-50 border rounded p-4 text-center"
                        >
                            <p class="text-sm text-gray-600">
                                Waiting for response...
                            </p>
                        </div>
                    {:else if deal.status === "accepted"}
                        <div
                            class="mt-3 bg-green-100 p-3 rounded flex items-center justify-between"
                        >
                            <div>
                                <p class="font-medium text-green-800">
                                    Deal accepted!
                                </p>
                                <p class="text-green-700">
                                    {formatCurrency(deal.price)}
                                </p>
                            </div>
                            Votes: {deal.voteCount}
                            <div class="flex space-x-2">
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={$page.params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="complete"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-3 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-md text-sm"
                                    >
                                        Complete Deal
                                    </button>
                                </form>
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={$page.params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="stop"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-3 py-1 bg-red-600 hover:bg-red-700 text-white rounded-md text-sm"
                                    >
                                        Stop Deal
                                    </button>
                                </form>
                            </div>
                        </div>
                    {:else if deal.status === "rejected"}
                        <div class="mt-3 bg-red-100 p-3 rounded text-center">
                            <p class="text-red-600 font-medium">
                                Deal declined
                            </p>
                        </div>
                    {/if}
                {/if}
            </div>

            <!-- Chat messages area -->
            <div
                class="flex-1 overflow-y-auto p-4 bg-gray-50 dark:bg-gray-900"
                bind:this={messageContainer}
            >
                {#if messages.length === 0}
                    <div
                        class="flex flex-col items-center justify-center h-full"
                    >
                        <div
                            class="bg-blue-100 dark:bg-blue-900 rounded-full p-4 mb-4"
                        >
                            <MessageCircle
                                class="h-8 w-8 text-blue-500 dark:text-blue-300"
                            />
                        </div>
                        <p class="text-gray-600 dark:text-gray-400">
                            No messages yet. Start the conversation!
                        </p>
                    </div>
                {:else}
                    <div class="space-y-4">
                        {#each messages as message, index}
                            {#if shouldShowDate(index)}
                                <div class="flex justify-center my-4">
                                    <div
                                        class="bg-gray-200 dark:bg-gray-700 rounded-full px-4 py-1 text-xs text-gray-600 dark:text-gray-300"
                                    >
                                        {formatDate(message.created_at)}
                                    </div>
                                </div>
                            {/if}
                            <div
                                class={`flex ${message.user_id === $user.id ? "justify-end" : "justify-start"}`}
                            >
                                <div
                                    class={`max-w-[70%] ${message.user_id === $user.id ? "bg-blue-500 text-white rounded-tl-lg rounded-tr-lg rounded-bl-lg" : "bg-white dark:bg-gray-800 text-gray-900 dark:text-white rounded-tl-lg rounded-tr-lg rounded-br-lg"} px-4 py-3 shadow`}
                                >
                                    <p>{message.content}</p>
                                    <div
                                        class={`text-xs mt-1 flex items-center ${message.user_id === $user.id ? "text-blue-200 justify-end" : "text-gray-500 dark:text-gray-400"}`}
                                    >
                                        {formatTime(message.created_at)}
                                        {#if message.user_id === $user.id}
                                            <span class="ml-1"
                                                >{message.read
                                                    ? "✓✓"
                                                    : "✓"}</span
                                            >
                                        {/if}
                                    </div>
                                </div>
                            </div>
                        {/each}
                    </div>
                {/if}
            </div>

            <!-- Message sending area -->
            {#if !chatClosed}
                <div
                    class="bg-white dark:bg-gray-800 p-4 border-t border-gray-200 dark:border-gray-700"
                >
                    <div class="flex justify-between mb-2">
                        <button
                            on:click={openDealModal}
                            class="flex items-center text-sm font-medium text-green-600 dark:text-green-400 hover:text-green-700"
                        >
                            <DollarSign class="h-4 w-4 mr-1" /> Make a deal
                        </button>
                    </div>
                    <form
                        method="post"
                        action="?/sendMessage"
                        class="flex items-center space-x-2"
                        use:enhance
                    >
                        <button
                            type="button"
                            class="p-2 rounded-full text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                        >
                            <Image class="h-5 w-5" />
                        </button>

                        <!-- Updated input: flex‑1 for full width, dark:bg‑gray for dark mode -->
                        <input
                            type="text"
                            name="content"
                            bind:value={messageInput}
                            placeholder="Type a message..."
                            class="flex-1 bg-gray-100 dark:bg-gray-700 dark:text-white text-gray-900 rounded-md px-4 py-2 focus:outline-none focus:ring-2 focus:ring-blue-500"
                        />

                        <button
                            type="submit"
                            disabled={!messageInput.trim()}
                            class="ml-2 p-2 rounded-full bg-blue-500 hover:bg-blue-600 text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <Send class="h-5 w-5" />
                        </button>
                    </form>
                </div>
            {:else}
                <div
                    class="bg-white dark:bg-gray-800 p-4 border-t border-gray-200 dark:border-gray-700 text-center text-gray-500"
                >
                    This chat is closed. You cannot send new messages.
                </div>
            {/if}
        {/if}
    </div>
</div>

{#if showDealModal}
    <div
        class="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50"
        transition:fade={{ duration: 200 }}
    >
        <div
            class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-6 w-full max-w-md mx-4"
            transition:slide={{ duration: 200 }}
        >
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-medium text-gray-900 dark:text-white">
                    Make a Deal
                </h3>
                <button
                    on:click={closeDealModal}
                    class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                >
                    <X class="h-5 w-5" />
                </button>
            </div>
            <form method="post" action="?/sendDeal" use:enhance>
                <input type="hidden" name="chat_id" value={$page.params.id} />
                <div class="mb-6">
                    <label
                        class="block text-gray-700 dark:text-gray-300 mb-2"
                        for="price">Enter your offer price</label
                    >
                    <div class="relative">
                        <div
                            class="absolute inset-y-0 left-0 pl-3 flex items-center pointer-events-none"
                        >
                            <DollarSign class="h-5 w-5 text-gray-400" />
                        </div>
                        <input
                            id="price"
                            type="number"
                            name="price"
                            min="1"
                            step="1"
                            bind:value={dealPrice}
                            class="pl-10 block w-full border border-gray-300 dark:border-gray-600 rounded-md py-2 px-4 bg-white dark:bg-gray-700 text-gray-900 dark:text-white focus:outline-none focus:ring-2 focus:ring-blue-500"
                            placeholder="Enter price"
                        />
                    </div>
                </div>
                <div class="flex justify-end space-x-3">
                    <button
                        type="button"
                        on:click={closeDealModal}
                        class="px-4 py-2 border border-gray-300 dark:border-gray-600 rounded-md text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                        >Cancel</button
                    >
                    <button
                        type="submit"
                        disabled={!dealPrice}
                        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        >Send Offer</button
                    >
                </div>
            </form>
        </div>
    </div>
{/if}
