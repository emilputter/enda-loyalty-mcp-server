export interface ToolActivity {
    name: string;
    arguments: string;
    result: string;
}

export interface MessageType {
    id: number;
    role: "user" | "assistant";
    content: string;

    toolActivity?: ToolActivity[];
}