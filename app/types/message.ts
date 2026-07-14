export interface MessageType {
    id: number;
    role: "user" | "assistant";
    content: string;
}