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

                </div>

            </div>

        </div>
    );
}