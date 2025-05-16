import { env } from "$env/dynamic/public";



export const activeTabClass = "border-b-2 border-blue-500 text-blue-500";
export const inactiveTabClass =
  "text-gray-500 hover:text-gray-700 dark:text-gray-400 dark:hover:text-gray-200";



export const socketChatUrl = env.PUBLIC_CHAT_API || "http://127.0.0.1:4000";
export const chatUrl = env.PUBLIC_CHAT_API?.includes('127.0.0.1') ? env.PUBLIC_CHAT_API + "/chat" : "http://127.0.0.1:4000";