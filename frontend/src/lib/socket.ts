import { io } from "socket.io-client";
import { socketChatUrl } from "./consts";



export const socket = io(socketChatUrl, {
    path: socketChatUrl.includes("127.0.0.1") ? "/socket.io" : "/chat/socket.io",
    transports: ['websocket', 'polling'],

});