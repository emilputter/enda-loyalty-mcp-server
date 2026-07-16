import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { MessageType } from "../types/message";

interface Props {
  message: MessageType;
}

export default function Message({ message }: Props) {
  return (
    <div
      className={
        message.role === "user"
          ? "bg-blue-100 text-black p-3 rounded-lg text-right"
          : "bg-gray-100 text-black p-3 rounded-lg"
      }
    >
      <strong className="block mb-1">
        {message.role === "user" ? "You" : "AI"}:
      </strong>

      <div className="markdown-content">
        <ReactMarkdown
          remarkPlugins={[remarkGfm]}
          components={{
            strong: ({ children }) => (
              <strong className="font-semibold">{children}</strong>
            ),
            ul: ({ children }) => (
              <ul className="list-disc pl-5 my-1">{children}</ul>
            ),
            ol: ({ children }) => (
              <ol className="list-decimal pl-5 my-1">{children}</ol>
            ),
            li: ({ children }) => <li className="my-0.5">{children}</li>,
            p: ({ children }) => <p className="my-1">{children}</p>,
            code: ({ children }) => (
              <code className="bg-gray-200 px-1 rounded text-sm">
                {children}
              </code>
            ),
            pre: ({ children }) => (
              <pre className="bg-gray-200 p-2 rounded my-1 overflow-x-auto text-sm">
                {children}
              </pre>
            ),
            h1: ({ children }) => (
              <h1 className="text-lg font-bold my-2">{children}</h1>
            ),
            h2: ({ children }) => (
              <h2 className="text-base font-bold my-1">{children}</h2>
            ),
            h3: ({ children }) => (
              <h3 className="text-sm font-bold my-1">{children}</h3>
            ),
          }}
        >
          {message.content}
        </ReactMarkdown>
      </div>
    </div>
  );
}
