import { io } from "socket.io-client";
import { chatUrl, isDev } from "./consts"; 

const host = isDev
  ? "http://localhost:4000"
  : window.location.origin;

const socketPath = isDev
  ? "/socket.io"
  : "/chat/socket.io";

export const socket = io(host, {
  path: socketPath,
  transports: ["polling", "websocket"],
  secure: !isDev,                 
});
