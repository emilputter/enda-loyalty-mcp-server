"use client";

import { useState, useEffect, useRef } from "react";
import Message from "./Message";
import { MessageType } from "../types/message";
import { askAI } from "../services/aiService";


export default function ChatBox() {
const bottomRef = useRef<HTMLDivElement>(null);
    const [messages, setMessages] = useState<MessageType[]>([
{
    id: 1,
    role: "assistant",
    content: ` Welcome to Enda Assist

I'm connected to the **ENDA Loyalty Platform** and can help you manage the system with several tools.`
}
]);


    const [input, setInput] = useState("");
    const [loading, setLoading] = useState(false);

    async function sendMessage() {

        if (input.trim() === "" || loading) {
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

        setLoading(true);
        const response = await askAI([
    ...messages,
    newMessage
]);


const aiMessage: MessageType = {
    id: messages.length + 2,
    role: "assistant",
    content: response.response,
    toolActivity: response.tool_activity,
};


setMessages(prev => [
    ...prev,
    aiMessage
]);
setLoading(false);

        setInput("");
    }


    return (
    <div className="w-full max-w-5xl h-[85vh] bg-white rounded-2xl shadow-xl border flex flex-col">

        {/* Header */}

        <div className="border-b px-6 py-5 flex justify-between items-center">

            <div>

                <h1 className="text-2xl font-bold text-gray-800">
                    Enda Assist
                </h1>

                <p className="text-sm text-gray-500">
                    AI Administration Assistant
                </p>

            </div>

            <div className="flex items-center gap-2">

                <div className="w-3 h-3 rounded-full bg-green-500"></div>

                <span className="text-sm text-gray-500">
                    Connected
                </span>

            </div>

        </div>

        {/* Messages */}

        <div className="flex-1 overflow-y-auto p-6 space-y-5">

            {messages.map((message) => (

                <Message
                    key={message.id}
                    message={message}
                />

            ))}

            {loading && (
    <div className="flex justify-start">

        <div className="max-w-md">

            <p className="text-sm mb-1 font-semibold text-gray-500">
                Enda Assist
            </p>

            <div className="rounded-2xl px-5 py-4 bg-white border border-gray-200 shadow-sm">

                <div className="flex items-center gap-2">

                    <div className="flex gap-1">

                        <span className="w-2 h-2 rounded-full bg-gray-400 animate-bounce"></span>

                        <span
                            className="w-2 h-2 rounded-full bg-gray-400 animate-bounce"
                            style={{ animationDelay: "150ms" }}
                        ></span>

                        <span
                            className="w-2 h-2 rounded-full bg-gray-400 animate-bounce"
                            style={{ animationDelay: "300ms" }}
                        ></span>

                    </div>

                    <span className="text-gray-500">
                        Thinking...
                    </span>

                </div>

            </div>

        </div>

    </div>
)}
<div ref={bottomRef}></div>

        </div>

        {/* Input */}

        <div className="border-t p-5">

            <div className="flex gap-3">

                <input

                    className="flex-1 border rounded-xl px-4 py-3 text-black focus:outline-none focus:ring-2 focus:ring-pink-500"

                    placeholder="Ask about users, roles, rewards..."

                    value={input}

                    onChange={(e) => setInput(e.target.value)}

                    onKeyDown={(e) => {
                        if (e.key === "Enter") {
                            sendMessage();
                        }
                    }}

                />

                <button

                    className="bg-pink-600 hover:bg-pink-700 text-white px-6 rounded-xl transition"

                    onClick={sendMessage}

                >

                    Send

                </button>

            </div>

        </div>

    </div>
);
}