import ReactMarkdown from "react-markdown";
import remarkGfm from "remark-gfm";
import { MessageType } from "../types/message";

interface Props {
    message: MessageType;
}

export default function Message({ message }: Props) {

    const isUser = message.role === "user";

    return (
        <div
            className={`flex ${
                isUser ? "justify-end" : "justify-start"
            }`}
        >

            <div className="max-w-3xl">

                <p
                    className={`text-sm mb-1 font-semibold ${
                        isUser
                            ? "text-right text-gray-500"
                            : "text-gray-500"
                    }`}
                >
                    {isUser ? "You" : "Enda Assist"}
                </p>

                <div
                    className={`rounded-2xl px-5 py-4 shadow-sm ${
                        isUser
                            ? "bg-pink-600 text-white"
                            : "bg-white border border-gray-200 text-black"
                    }`}
                >

                    <ReactMarkdown remarkPlugins={[remarkGfm]}>
                        {message.content}
                    </ReactMarkdown>

                    {!isUser &&
    message.toolActivity &&
    message.toolActivity.length > 0 && (

    <details className="mt-4 border rounded-xl bg-gray-50">

        <summary className="cursor-pointer p-3 font-semibold text-sm">

            Used {message.toolActivity.length} MCP Tool{message.toolActivity.length > 1 ? "s" : ""}

        </summary>

        <div className="p-3 space-y-3">

            {message.toolActivity.map((tool, index) => (

                <details
                    key={index}
                    className="border rounded-lg bg-white"
                >

                    <summary className="cursor-pointer p-3 font-medium">

                        {tool.name}

                    </summary>

                    <div className="p-3">

                        <h4 className="font-semibold mb-2">
                            Arguments
                        </h4>

                        <pre className="text-xs overflow-auto bg-gray-100 p-2 rounded">
                            {tool.arguments}
                        </pre>

                        <h4 className="font-semibold mt-4 mb-2">
                            Result
                        </h4>

                        <pre className="text-xs overflow-auto bg-gray-100 p-2 rounded">
                            {tool.result}
                        </pre>

                    </div>

                </details>

            ))}

        </div>

    </details>

)}

                </div>

            </div>

        </div>
    );
}