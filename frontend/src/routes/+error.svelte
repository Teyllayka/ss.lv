<script>
    import { page } from "$app/stores";
    import { goto } from "$app/navigation";
    import {
        AlertTriangle,
        FileSearch,
        ServerCrash,
        Wifi,
        Home,
        ArrowLeft,
        RefreshCw,
    } from "lucide-svelte";

    $: status = $page.status;
    $: message = $page.error?.message || "";

    const errorTypes = {
        404: {
            title: "Page Not Found",
            description:
                "The page you're looking for doesn't exist or has been moved.",
            icon: FileSearch,
            color: "text-amber-500",
            bgColor: "bg-amber-100 dark:bg-amber-900/30",
        },
        500: {
            title: "Server Error",
            description:
                "Our server encountered an error. Please try again later.",
            icon: ServerCrash,
            color: "text-red-500",
            bgColor: "bg-red-100 dark:bg-red-900/30",
        },
        503: {
            title: "Service Unavailable",
            description:
                "Our service is currently unavailable. We're working on it!",
            icon: ServerCrash,
            color: "text-red-500",
            bgColor: "bg-red-100 dark:bg-red-900/30",
        },
        offline: {
            title: "You're Offline",
            description: "Check your internet connection and try again.",
            icon: Wifi,
            color: "text-blue-500",
            bgColor: "bg-blue-100 dark:bg-blue-900/30",
        },
        default: {
            title: "Something Went Wrong",
            description: "An unexpected error occurred.",
            icon: AlertTriangle,
            color: "text-orange-500",
            bgColor: "bg-orange-100 dark:bg-orange-900/30",
        },
    };

    $: errorType = errorTypes[status] || errorTypes.default;

    $: if (typeof window !== "undefined" && !window.navigator.onLine) {
        errorType = errorTypes.offline;
    }

    function goBack() {
        history.back();
    }

    function goHome() {
        goto("/");
    }

    function refresh() {
        window.location.reload();
    }
</script>

<div
    class="min-h-screen bg-gray-50 dark:bg-gray-900 flex flex-col items-center justify-center p-4"
>
    <div
        class="w-full max-w-md bg-white dark:bg-gray-800 rounded-lg shadow-lg overflow-hidden"
    >
        <div class="flex justify-center pt-10 pb-6">
            <div class={`p-4 rounded-full ${errorType.bgColor}`}>
                <svelte:component
                    this={errorType.icon}
                    class={`h-16 w-16 ${errorType.color}`}
                />
            </div>
        </div>

        <div class="px-6 pb-6 text-center">
            <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
                {status} | {errorType.title}
            </h1>

            <p class="text-gray-600 dark:text-gray-300 mb-6">
                {errorType.description}
            </p>

            {#if message}
                <div
                    class="mb-6 p-3 bg-gray-100 dark:bg-gray-700 rounded-md text-sm text-gray-700 dark:text-gray-300 overflow-auto max-h-32"
                >
                    {message}
                </div>
            {/if}

            <div class="flex flex-col sm:flex-row gap-3 justify-center">
                <button
                    on:click={goBack}
                    class="flex items-center justify-center gap-2 px-4 py-2 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors"
                >
                    <ArrowLeft class="h-4 w-4" />
                    Go Back
                </button>

                <button
                    on:click={goHome}
                    class="flex items-center justify-center gap-2 px-4 py-2 bg-white dark:bg-gray-700 text-gray-700 dark:text-gray-200 border border-gray-300 dark:border-gray-600 rounded-md hover:bg-gray-50 dark:hover:bg-gray-600 transition-colors"
                >
                    <Home class="h-4 w-4" />
                    Home Page
                </button>

                <button
                    on:click={refresh}
                    class="flex items-center justify-center gap-2 px-4 py-2 bg-blue-600 hover:bg-blue-700 text-white rounded-md transition-colors"
                >
                    <RefreshCw class="h-4 w-4" />
                    Refresh
                </button>
            </div>
        </div>

        <div
            class="px-6 py-4 bg-gray-100 dark:bg-gray-750 border-t border-gray-200 dark:border-gray-700"
        >
            <p class="text-center text-sm text-gray-600 dark:text-gray-400">
                If this problem persists, please contact our support team.
            </p>
        </div>
    </div>

    <div
        class="mt-8 text-center text-sm text-gray-500 dark:text-gray-400 max-w-md"
    >
        <p>
            Need help? Try clearing your browser cache or using a different
            browser.
        </p>
        <p class="mt-2">
            You can also check our <a
                href="/help"
                class="text-blue-600 dark:text-blue-400 hover:underline"
                >help center</a
            > for more information.
        </p>
    </div>
</div>
