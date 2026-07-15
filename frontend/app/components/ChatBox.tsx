"use client";

import { useState } from "react";
import Message from "./Message";
import { MessageType } from "../types/message";
import { askAI } from "../services/aiService";


export default function ChatBox() {

    const [messages, setMessages] = useState<MessageType[]>([
        {
            id: 1,
            role: "assistant",
            content: "Hello, I am the ENDA AI assistant."
        }
    ]);


    const [input, setInput] = useState("");


    async function sendMessage() {

        if(input.trim() === "") {
            return;
        }


        const newMessage: MessageType = {
            id: messages.length + 1,
            role: "user",
            content: input
        };


        setMessages([
            ...messages,
            newMessage
        ]);

        const response = await askAI([
    ...messages,
    newMessage
]);


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
        <div className="w-full max-w-3xl">

            <div className="border rounded-lg p-4 h-[500px] space-y-3 overflow-y-auto">

                {
                    messages.map((message)=>(
                        <Message 
                            key={message.id}
                            message={message}
                        />
                    ))
                }

            </div>


            <div className="flex gap-2 mt-4">

                <input
                    className="border rounded-lg p-3 flex-1 text-black"
                    placeholder="Ask something..."
                    value={input}
                    onChange={(e)=>setInput(e.target.value)}
                />


                <button
                    className="bg-blue-500 text-white px-5 rounded-lg"
                    onClick={sendMessage}
                >
                    Send
                </button>

            </div>


        </div>
    );
}