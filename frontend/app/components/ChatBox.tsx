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
      content: "Hello, I am the ENDA AI assistant.",
    },
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
      content: input,
    };

    setMessages([...messages, newMessage]);
    setInput("");

    setLoading(true);

    const response = await askAI([...messages, newMessage]);

    const aiMessage: MessageType = {
      id: messages.length + 2,
      role: "assistant",
      content: response,
    };

    setMessages((prev) => [...prev, aiMessage]);
    setLoading(false);
  }

  return (
    <div className="w-full max-w-3xl">
      <div className="border rounded-lg p-4 h-[500px] space-y-3 overflow-y-auto">
        {messages.map((message) => (
          <Message key={message.id} message={message} />
        ))}

        {loading && (
          <div className="bg-gray-100 text-black p-3 rounded-lg italic text-gray-500">
            AI is thinking...
          </div>
        )}
      </div>

      <div className="flex gap-2 mt-4">
        <input
          className="border rounded-lg p-3 flex-1 text-black disabled:opacity-50"
          placeholder="Ask something..."
          value={input}
          onChange={(e) => setInput(e.target.value)}
          onKeyDown={(e) => e.key === "Enter" && sendMessage()}
          disabled={loading}
        />

        <button
          className="bg-blue-500 text-white px-5 rounded-lg disabled:opacity-50"
          onClick={sendMessage}
          disabled={loading}
        >
          {loading ? "..." : "Send"}
        </button>
      </div>
    </div>
  );
}
