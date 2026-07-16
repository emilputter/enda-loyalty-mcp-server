import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { MessageType } from "../types/message";

export default function Message({ message }: { message: MessageType }) {
  const isUser = message.role === "user";
  return (
    <article className={`message-row ${isUser ? "from-user" : "from-assistant"}`}>
      {!isUser && <div className="message-avatar" aria-hidden="true">E</div>}
      <div className="message-stack">
        <span className="message-sender">{isUser ? "You" : "Enda Assist"}</span>
        <div className="message-bubble markdown-content">
          <ReactMarkdown remarkPlugins={[remarkGfm]} components={{
            ul: ({ children }) => <ul>{children}</ul>, ol: ({ children }) => <ol>{children}</ol>,
            code: ({ children }) => <code>{children}</code>, pre: ({ children }) => <pre>{children}</pre>,
          }}>{message.content}</ReactMarkdown>
        </div>
        {!isUser && message.toolActivity && message.toolActivity.length > 0 && (
          <details className="tool-activity">
            <summary>Account tools consulted <span>{message.toolActivity.length}</span></summary>
            <div className="tool-list">{message.toolActivity.map((tool, index) => (
              <details key={`${tool.name}-${index}`}><summary>{tool.name}</summary><pre>{JSON.stringify(tool.arguments, null, 2)}{`\n\n`}{tool.result}</pre></details>
            ))}</div>
          </details>
        )}
      </div>
    </article>
  );
}
