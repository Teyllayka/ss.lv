import { io } from "socket.io-client";
import { chatUrl } from "./consts";



export const socket = io(chatUrl, {
    path: chatUrl.includes("127.0.0.1") ? "/socket.io" : "/chat/socket.io",
});