import type { Message as MessageType } from "../types/message";

interface Props {
    message: MessageType;
}

function Message({ message }: Props) {

    return (
    <div
        className={
            message.role === "user"
                ? "bg-blue-100 p-3 rounded-lg text-right"
                : "bg-gray-100 p-3 rounded-lg"
        }
    >

        <strong>
            {message.role === "user" ? "You" : "AI"}:
        </strong>

        <span>
            {" "}
            {message.content}
        </span>

    </div>
);
}

export default Message;