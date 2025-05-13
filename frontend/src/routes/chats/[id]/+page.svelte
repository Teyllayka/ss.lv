<script lang="ts">
    import { getContext, onMount } from "svelte";
    import { goto } from "$app/navigation";
    import { user } from "$lib/userStore";
    import { fade, slide } from "svelte/transition";
    import {
        Send,
        ArrowLeft,
        User,
        Image as ImageIcon,
        MessageCircle,
        DollarSign,
        X,
        Check,
        ChevronDown,
        ZoomIn,
    } from "lucide-svelte";
    import { enhance } from "$app/forms";
    import { socket } from "$lib/socket";
    import { page } from "$app/stores";
    import InputField from "$lib/components/InputField.svelte";
    import { get, type Writable } from "svelte/store";
    import * as m from "$lib/paraglide/messages.js";

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
    let photos: string[] = [];
    let photoFiles: File[] = [];
    let photoInput: HTMLInputElement;
    let imageViewerOpen = false;

    interface UnreadMessages {
        unreadMessages: number;
    }
    const areUnreadMessages: Writable<UnreadMessages> =
        getContext("areUnreadMessages");

    let currentViewedImage = "";

    if (data.readMessages && data.readMessages.length > 0) {
        areUnreadMessages.update((prev) => ({
            ...prev,
            unreadMessages: prev.unreadMessages - 1,
        }));
    }

    let chat = data.chat || {};
    $: chatClosed =
        (deal && (deal.state === "accepted" || deal.voteCount === 2)) ||
        chat.archived;
    let windowHeight: number;

    function scrollToBottom() {
        setTimeout(() => {
            if (messageContainer) {
                messageContainer.scrollTop = messageContainer.scrollHeight;
            }
        }, 0);
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

    function formatCurrency(amount: number): string {
        return new Intl.NumberFormat("en-US", {
            style: "currency",
            currency: "USD",
            minimumFractionDigits: 0,
            maximumFractionDigits: 0,
        }).format(amount);
    }

    function openImageViewer(url: string) {
        currentViewedImage = url;
        imageViewerOpen = true;
    }

    function closeImageViewer() {
        imageViewerOpen = false;
        currentViewedImage = "";
    }

    async function markMessageAsRead(messageId: number, chatId: number) {
        try {
            await fetch("/api/update-message", {
                method: "POST",
                headers: { "Content-Type": "application/json" },
                body: JSON.stringify({ messageId: messageId, chatId: chatId }),
            });
            messages = messages.map((msg: { id: number }) =>
                msg.id === messageId ? { ...msg, read: true } : msg,
            );
        } catch (err) {
            console.error("could not mark message read", err);
        }
    }

    onMount(() => {
        scrollToBottom();

        windowHeight = window.innerHeight;
        const handleResize = () => {
            windowHeight = window.innerHeight;
        };

        window.addEventListener("resize", handleResize);

        function handleNewMessage(newMessage: any) {
            messages = [...messages, newMessage];
            scrollToBottom();
            if (newMessage.user_id !== get(user).id) {
                markMessageAsRead(newMessage.id, chat.id);
            }
        }

        function handleDealUpdate(data: any) {
            deal = data;
            if (data) {
                showDealModal = false;
            }
        }

        function handleReadReceipt(messageId: any) {
            messages = messages.map((msg: any) =>
                msg.id === messageId ? { ...msg, read: true } : msg,
            );
        }

        const chatId = get(page).params.id;

        if (advert) {
            socket.on(`chat-${chatId}`, handleNewMessage);
            socket.on(`deal-${chatId}`, handleDealUpdate);
            socket.on(`message-read-chat-${chatId}`, handleReadReceipt);
        }

        return () => {
            socket.off(`chat-${chatId}`, handleNewMessage);
            socket.off(`deal-${chatId}`, handleDealUpdate);
            socket.off(`message-read-chat-${chatId}`, handleReadReceipt);
            window.removeEventListener("resize", handleResize);
        };
    });

    function handlePhotosChange(event: Event) {
        const input = event.target as HTMLInputElement;
        const files = input.files;
        if (!files) return;

        const newFiles = Array.from(files);

        photoFiles = [...photoFiles, ...newFiles];
        photos = [...photos, ...newFiles.map((f) => URL.createObjectURL(f))];
    }

    function removePhoto(index: number) {
        photoFiles.splice(index, 1);
        photos.splice(index, 1);
    }
</script>

<svelte:window bind:innerHeight={windowHeight} />

<div
    class="bg-gray-100 dark:bg-gray-900 h-[calc(100vh-74px)] flex flex-col overflow-hidden"
>
    <div
        class="max-w-4xl mx-auto w-full h-full flex flex-col bg-white dark:bg-gray-800 shadow-lg"
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
                    <strong class="font-bold">{m.error()}</strong>
                    <span class="block sm:inline"> {error}</span>
                </div>
            </div>
        {:else if advert}
            {@const otherUser = getOtherUserInfo()}

            <div class="bg-white dark:bg-gray-800 shadow-md p-3 flex flex-col">
                <div class="flex items-center justify-between">
                    <div class="flex items-center">
                        <a
                            href="/chats"
                            class="mr-3 text-gray-600 dark:text-gray-300 hover:text-gray-900 dark:hover:text-white"
                        >
                            <ArrowLeft class="h-5 w-5" />
                        </a>
                        <div class="flex-shrink-0 mr-3">
                            {#if otherUser.avatar}
                                <img
                                    src={otherUser.avatar || "/placeholder.svg"}
                                    alt={`${otherUser.name} ${otherUser.surname}`}
                                    class="h-10 w-10 rounded-full object-cover border-2 border-gray-200 dark:border-gray-700"
                                />
                            {:else}
                                <div
                                    class="h-10 w-10 rounded-full bg-gray-300 dark:bg-gray-600 flex items-center justify-center"
                                >
                                    <User
                                        class="h-5 w-5 text-gray-600 dark:text-gray-300"
                                    />
                                </div>
                            {/if}
                        </div>
                        <div class="flex-1">
                            <h2
                                class="font-medium text-gray-900 dark:text-white"
                            >
                                {otherUser.name}
                                {otherUser.surname}
                            </h2>
                        </div>
                    </div>
                </div>

                <div
                    class="mt-2 flex items-center justify-between border-t border-gray-200 dark:border-gray-700 pt-2"
                >
                    <div class="flex items-center">
                        <img
                            src={advert.photo_url || "/placeholder.svg"}
                            alt={advert.title}
                            class="h-12 w-12 object-cover rounded-md mr-2 border border-gray-200 dark:border-gray-700"
                        />
                        <div>
                            <h3
                                class="font-medium text-sm text-gray-900 dark:text-white truncate max-w-[180px]"
                            >
                                {advert.title}
                            </h3>
                            <div class="flex items-center">
                                <span
                                    class="text-sm font-bold text-green-600 dark:text-green-400"
                                >
                                    {formatCurrency(advert.price)}
                                </span>
                                {#if advert.old_price}
                                    <span
                                        class="ml-2 text-xs line-through text-gray-500 dark:text-gray-400"
                                    >
                                        {formatCurrency(advert.old_price)}
                                    </span>
                                {/if}
                            </div>
                        </div>
                    </div>
                    <div class="flex items-center">
                        <button
                            on:click={toggleAdditionalInfo}
                            class="flex items-center text-xs text-blue-600 dark:text-blue-400 hover:underline"
                        >
                            {m.details()}
                            <ChevronDown
                                class={`h-3 w-3 ml-1 transition-transform ${showAdditionalInfo ? "transform rotate-180" : ""}`}
                            />
                        </button>
                    </div>
                </div>

                {#if showAdditionalInfo}
                    <div
                        transition:slide={{ duration: 200 }}
                        class="mt-2 p-2 bg-gray-100 dark:bg-gray-700 rounded-md text-xs"
                    >
                        <p
                            class="text-gray-800 dark:text-gray-200 mb-1 break-words line-clamp-2"
                        >
                            {advert.description}
                        </p>
                        <div class="flex justify-between text-xs">
                            <span class="text-gray-600 dark:text-gray-400">
                                {m.category()}: {advert.category}
                            </span>
                            <span class="text-gray-600 dark:text-gray-400">
                                {m.listed()}: {new Date(
                                    advert.created_at,
                                ).toLocaleDateString()}
                            </span>
                        </div>
                    </div>
                {/if}

                {#if deal}
                    {#if deal.status === "pending" && deal.requester_id !== get(user).id}
                        <div
                            class="mt-2 bg-yellow-50 border rounded p-2 text-center text-sm"
                        >
                            <p class="font-medium">
                                Deal Offer: {formatCurrency(deal.price)}
                            </p>
                            <div class="mt-2 flex justify-center space-x-3">
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={get(page).params.id}
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
                                        class="flex items-center px-2 py-1 bg-green-600 hover:bg-green-700 text-white rounded-md text-xs"
                                    >
                                        <Check class="h-3 w-3 mr-1" />
                                        {m.accept()}
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
                                        value={get(page).params.id}
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
                                        class="flex items-center px-2 py-1 bg-white dark:bg-gray-800 border border-gray-300 dark:border-gray-600 rounded-md text-xs text-gray-700 dark:text-gray-300 hover:bg-gray-50 dark:hover:bg-gray-700"
                                    >
                                        <X class="h-3 w-3 mr-1" />
                                        {m.decline()}
                                    </button>
                                </form>
                            </div>
                        </div>
                    {:else if deal.status === "pending" && deal.requester_id === get(user).id}
                        <div
                            class="mt-2 bg-gray-50 border rounded p-2 text-center text-xs"
                        >
                            <p class="text-gray-600">Waiting for response...</p>
                        </div>
                    {:else if deal.status === "accepted"}
                        <div
                            class="mt-2 bg-green-100 p-2 rounded flex items-center justify-between text-sm"
                        >
                            <div>
                                <p class="font-medium text-green-800">
                                    {m.deal_accepted()}
                                </p>
                                <p class="text-green-700 text-xs">
                                    {formatCurrency(deal.price)}
                                </p>
                            </div>
                            <div class="text-xs">Votes: {deal.voteCount}</div>
                            <div class="flex space-x-1">
                                <form
                                    method="post"
                                    action="?/sendDeal"
                                    use:enhance
                                >
                                    <input
                                        type="hidden"
                                        name="chat_id"
                                        value={get(page).params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="complete"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-2 py-1 bg-blue-600 hover:bg-blue-700 text-white rounded-md text-xs"
                                    >
                                        {m.complete()}
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
                                        value={get(page).params.id}
                                    />
                                    <input
                                        type="hidden"
                                        name="state"
                                        value="stop"
                                    />
                                    <button
                                        type="submit"
                                        class="flex items-center px-2 py-1 bg-red-600 hover:bg-red-700 text-white rounded-md text-xs"
                                    >
                                        {m.stop()}
                                    </button>
                                </form>
                            </div>
                        </div>
                    {:else if deal.status === "rejected"}
                        <div
                            class="mt-2 bg-red-100 p-2 rounded text-center text-xs"
                        >
                            <p class="text-red-600 font-medium">
                                {m.deal_declined()}
                            </p>
                        </div>
                    {/if}
                {/if}
            </div>

            <div
                class="flex-1 overflow-y-auto p-3 bg-gray-50 dark:bg-gray-900"
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
                    <div class="space-y-3">
                        {#each messages as message, index}
                            {#if shouldShowDate(index)}
                                <div class="flex justify-center my-2">
                                    <div
                                        class="bg-gray-200 dark:bg-gray-700 rounded-full px-3 py-1 text-xs text-gray-600 dark:text-gray-300"
                                    >
                                        {formatDate(message.created_at)}
                                    </div>
                                </div>
                            {/if}
                            <div
                                class={`flex ${message.user_id === get(user).id ? "justify-end" : "justify-start"}`}
                            >
                                <div
                                    class={`max-w-[70%] ${message.user_id === get(user).id ? "bg-blue-500 text-white rounded-tl-lg rounded-tr-lg rounded-bl-lg" : "bg-white dark:bg-gray-800 text-gray-900 dark:text-white rounded-tl-lg rounded-tr-lg rounded-br-lg"} px-3 py-2 shadow`}
                                >
                                    {#if message.content}
                                        <p class="text-sm">{message.content}</p>
                                    {/if}

                                    {#if message.urls && message.urls.length > 0}
                                        <div
                                            class={`mt-2 ${message.urls.length > 1 ? "grid grid-cols-2 gap-1" : ""}`}
                                        >
                                            {#each message.urls as url, i}
                                                {#if url !== null}
                                                    <div class="relative group">
                                                        <img
                                                            src={url ||
                                                                "/placeholder.svg"}
                                                            alt="Message attachment"
                                                            class="rounded-md w-full h-auto object-cover cursor-pointer"
                                                            style="max-height: 150px;"
                                                            on:click={() =>
                                                                openImageViewer(
                                                                    url,
                                                                )}
                                                        />
                                                        <div
                                                            class="absolute inset-0 bg-black bg-opacity-0 group-hover:bg-opacity-20 flex items-center justify-center transition-all duration-200 opacity-0 group-hover:opacity-100"
                                                        >
                                                            <button
                                                                class="bg-black bg-opacity-50 text-white p-1 rounded-full"
                                                                on:click|stopPropagation={() =>
                                                                    openImageViewer(
                                                                        url,
                                                                    )}
                                                            >
                                                                <ZoomIn
                                                                    class="h-4 w-4"
                                                                />
                                                            </button>
                                                        </div>
                                                    </div>
                                                {/if}
                                            {/each}
                                        </div>
                                    {/if}

                                    <div
                                        class={`text-xs mt-1 flex items-center ${message.user_id === get(user).id ? "text-blue-200 justify-end" : "text-gray-500 dark:text-gray-400"}`}
                                    >
                                        {formatTime(message.created_at)}
                                        {#if message.user_id === get(user).id}
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

            {#if !chatClosed}
                <div
                    class="bg-white dark:bg-gray-800 p-3 border-t border-gray-200 dark:border-gray-700"
                >
                    <div class="flex justify-between mb-6">
                        <button
                            on:click={openDealModal}
                            class="flex items-center text-xs font-medium text-green-600 dark:text-green-400 hover:text-green-700"
                        >
                            <DollarSign class="h-4 w-4 mr-1" />
                            {m.make_a_deal()}
                        </button>
                    </div>

                    {#if photos.length}
                        <div class="flex space-x-2 mb-3 overflow-x-auto">
                            {#each photos as src, i}
                                <div class="relative">
                                    <img
                                        {src}
                                        class="h-16 w-16 object-cover rounded-lg border"
                                        alt="preview"
                                    />
                                    <button
                                        type="button"
                                        class="absolute top-1 right-1 bg-red-500 text-white rounded-full p-1 hover:bg-red-600"
                                        on:click={() => removePhoto(i)}
                                    >
                                        <X class="h-3 w-3" />
                                    </button>
                                </div>
                            {/each}
                        </div>
                    {/if}

                    <form
                        method="post"
                        action="?/sendMessage"
                        class="flex items-center space-x-2"
                        enctype="multipart/form-data"
                        use:enhance={() => {
                            return async ({ result, update }) => {
                                if (result) {
                                    messageInput = "";
                                    photos = [];
                                    photoFiles = [];
                                    photoInput.value = "";
                                }
                                update();
                            };
                        }}
                    >
                        <input
                            type="file"
                            accept="image/*"
                            multiple
                            name="photos"
                            bind:this={photoInput}
                            on:change={handlePhotosChange}
                            class="hidden"
                        />

                        <button
                            type="button"
                            class="p-1 rounded-full text-gray-500 dark:text-gray-400 hover:bg-gray-100 dark:hover:bg-gray-700"
                            on:click={() => photoInput.click()}
                        >
                            <ImageIcon class="h-5 w-5" />
                        </button>

                        <div class="flex-1">
                            <InputField
                                type="text"
                                name="content"
                                bind:value={messageInput}
                                placeholder="Type a message..."
                            />
                        </div>

                        <button
                            type="submit"
                            disabled={!messageInput.trim() &&
                                photos.length === 0}
                            class="p-2 rounded-full bg-blue-500 hover:bg-blue-600 text-white focus:outline-none focus:ring-2 focus:ring-blue-500 disabled:opacity-50 disabled:cursor-not-allowed"
                        >
                            <Send class="h-5 w-5" />
                        </button>
                    </form>
                </div>
            {:else}
                <div
                    class="bg-white dark:bg-gray-800 p-3 border-t border-gray-200 dark:border-gray-700 text-center text-gray-500 text-sm"
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
            class="bg-white dark:bg-gray-800 rounded-lg shadow-xl p-5 w-full max-w-md mx-4"
            transition:slide={{ duration: 200 }}
        >
            <div class="flex justify-between items-center mb-4">
                <h3 class="text-lg font-medium text-gray-900 dark:text-white">
                    {m.make_a_deal()}
                </h3>
                <button
                    on:click={closeDealModal}
                    class="text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-300"
                >
                    <X class="h-5 w-5" />
                </button>
            </div>
            <form method="post" action="?/sendDeal" use:enhance>
                <input
                    type="hidden"
                    name="chat_id"
                    value={get(page).params.id}
                />
                <div class="mb-4">
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
                        >{m.cancel()}</button
                    >
                    <button
                        type="submit"
                        disabled={!dealPrice}
                        class="px-4 py-2 bg-green-600 hover:bg-green-700 text-white rounded-md disabled:opacity-50 disabled:cursor-not-allowed"
                        >{m.send_offer()}</button
                    >
                </div>
            </form>
        </div>
    </div>
{/if}

{#if imageViewerOpen}
    <div
        class="fixed inset-0 bg-black bg-opacity-90 flex items-center justify-center z-50"
        transition:fade={{ duration: 200 }}
        on:click={closeImageViewer}
    >
        <div class="relative max-w-4xl max-h-[90vh]">
            <img
                src={currentViewedImage}
                alt="Full size image"
                class="max-h-[90vh] max-w-full object-contain"
            />
            <button
                class="absolute top-4 right-4 bg-black bg-opacity-50 text-white p-2 rounded-full hover:bg-opacity-70"
                on:click={closeImageViewer}
            >
                <X class="h-6 w-6" />
            </button>
        </div>
    </div>
{/if}
