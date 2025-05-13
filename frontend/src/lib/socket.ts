import { io } from "socket.io-client";
import { chatUrl } from "./consts";

// export const socket = io(chatUrl, {
//   path: `${chatUrl}/socket.io`,
// });

export const socket = io(chatUrl, {
    path: `${chatUrl}/socket.io`,
});