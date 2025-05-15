import { io } from "socket.io-client";
import { chatUrl } from "./consts";




export const socket = chatUrl === "https://ad-ee.tech/chat" ? io(chatUrl, {
  path: `${chatUrl}/socket.io`,  
  transports: ["polling", "websocket"], 
  secure: true,                    
}) : io(chatUrl);



