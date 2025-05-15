import { io } from "socket.io-client";
import { chatUrl } from "./consts"; 

function deriveSocketPath(url: string) {
  try {
    const pathname = new URL(url).pathname.replace(/\/$/, "");
    return pathname === "" ? "/socket.io" : `${pathname}/socket.io`;
  } catch {
    const p = url.replace(/\/$/, "");
    return p === "" ? "/socket.io" : `${p}/socket.io`;
  }
}

export const socket = io(chatUrl, {
  path: deriveSocketPath(chatUrl),
  transports: ["websocket"],
});
