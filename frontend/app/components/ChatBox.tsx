"use client";

import { FormEvent, useEffect, useRef, useState } from "react";
import Message from "./Message";
import { MessageType } from "../types/message";
import { askAI } from "../services/aiService";

const welcomeMessage: MessageType = {
  id: 1,
  role: "assistant",
  content:
    "Welcome to **Enda Assist**. I can help you explore loyalty activity, answer questions, and make sense of your account.",
};

const suggestions = [
  "What rewards can I redeem?",
  "Show my latest activity",
  "How do loyalty points work?",
];

export default function ChatBox() {
  const [messages, setMessages] = useState<MessageType[]>([welcomeMessage]);
  const [input, setInput] = useState("");
  const [loading, setLoading] = useState(false);
  const endOfMessages = useRef<HTMLDivElement>(null);

  useEffect(() => {
    endOfMessages.current?.scrollIntoView({ behavior: "smooth", block: "end" });
  }, [messages, loading]);

  async function sendMessage(event?: FormEvent<HTMLFormElement>) {
    event?.preventDefault();
    const text = input.trim();
    if (!text || loading) return;

    const newMessage: MessageType = { id: messages.length + 1, role: "user", content: text };
    setMessages((current) => [...current, newMessage]);
    setInput("");
    setLoading(true);

    try {
      const response = await askAI([...messages, newMessage]);
      setMessages((current) => [
        ...current,
        {
          id: current.length + 1,
          role: "assistant",
          content: response.response,
          toolActivity: response.tool_activity,
        },
      ]);
    } catch {
      setMessages((current) => [
        ...current,
        { id: current.length + 1, role: "assistant", content: "I couldn’t complete that request. Please try again in a moment." },
      ]);
    } finally {
      setLoading(false);
    }
  }

  function startOver() {
    if (!loading) setMessages([welcomeMessage]);
  }

  return (
    <section className="chat-frame" aria-label="Enda loyalty assistant">
      <header className="chat-header">
        <div className="brand-lockup">
          <div className="brand-mark" aria-hidden="true"><span /></div>
          <div>
            <p className="eyebrow">ENDA LOYALTY</p>
            <h1>Enda Assist</h1>
          </div>
        </div>
        <div className="header-actions">
          <span className="status"><i /> Online</span>
          <button className="reset-button" type="button" onClick={startOver} disabled={loading}>
            New chat
          </button>
        </div>
      </header>

      <div className="conversation" aria-live="polite">
        <div className="conversation-intro">
          <p className="eyebrow">YOUR LOYALTY COMPANION</p>
          <h2>How can we help today?</h2>
          <p>Clear answers and account guidance, right when you need them.</p>
        </div>

        <div className="message-list">
          {messages.map((message) => <Message key={message.id} message={message} />)}
          {loading && <div className="thinking" role="status"><span /><span /><span /> <p>Finding the right answer</p></div>}
          <div ref={endOfMessages} />
        </div>
      </div>

      <div className="composer-area">
        {messages.length === 1 && !loading && (
          <div className="suggestions" aria-label="Suggested questions">
            {suggestions.map((suggestion) => (
              <button key={suggestion} type="button" onClick={() => setInput(suggestion)}>{suggestion}</button>
            ))}
          </div>
        )}
        <form className="composer" onSubmit={sendMessage}>
          <label className="sr-only" htmlFor="chat-input">Your message</label>
          <input id="chat-input" placeholder="Ask about your loyalty account..." value={input} onChange={(e) => setInput(e.target.value)} disabled={loading} />
          <button className="send-button" type="submit" disabled={loading || !input.trim()} aria-label="Send message">
            <svg viewBox="0 0 24 24" aria-hidden="true"><path d="m21 3-7.8 18-3.4-7.8L2 9.8 21 3Zm-10.4 8.8 2.4 5.4L17.3 7 6.8 11.3l3.8.5Z" /></svg>
            <span>Send</span>
          </button>
        </form>
        <p className="privacy-note">Enda Assist may use account tools to answer your question.</p>
      </div>
    </section>
  );
}
