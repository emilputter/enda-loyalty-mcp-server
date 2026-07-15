import { useState } from "react";
import Message from "./Message";
import type { Message as MessageType } from "../types/message";
import { askAI } from "../services/aiService";


function ChatBox() {

    const [messages, setMessages] = useState<MessageType[]>([
        {
            id: 1,
            role: "assistant",
            content: "Hello, I am the ENDA AI assistant."
        }
    ]);


    const [input, setInput] = useState("");


    async function sendMessage() {

        if(input.trim() === "")
            return;


        const newMessage: MessageType = {
            id: messages.length + 1,
            role: "user",
            content: input
        };


        setMessages([
            ...messages,
            newMessage
        ]);

        const response = await askAI(input);

const aiMessage: MessageType = {
    id: messages.length + 2,
    role: "assistant",
    content: response
};

setMessages(prev => [
    ...prev,
    aiMessage
]);


        setInput("");
    }


    return (
    <div className="space-y-4">

        <div className="h-96 overflow-y-auto border rounded-lg p-4 space-y-3">

            {
                messages.map(message => (
                    <Message
                        key={message.id}
                        message={message}
                    />
                ))
            }

        </div>


        <div className="flex gap-2">

            <input
                className="flex-1 border rounded-lg p-2"
                value={input}
                onChange={(e)=>setInput(e.target.value)}
                placeholder="Ask something..."
            />


            <button
                className="bg-blue-500 text-white px-4 rounded-lg hover:bg-blue-600"
                onClick={sendMessage}
            >
                Send
            </button>

        </div>

    </div>
);
}


export default ChatBox;