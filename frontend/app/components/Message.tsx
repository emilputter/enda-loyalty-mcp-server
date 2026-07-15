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

            <strong>
                {message.role === "user" ? "You" : "AI"}:
            </strong>

            {" "}
            {message.content}

        </div>
    );
}