export type MessageType = {
    id: number;
    role: "user" | "assistant";
    content: string;
};