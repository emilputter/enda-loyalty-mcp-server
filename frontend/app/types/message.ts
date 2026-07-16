export interface ToolActivity {
  name: string;
  arguments: unknown;
  result: string;
}

export interface MessageType {
    id: number;
    role: "user" | "assistant";
    content: string;
    toolActivity?: ToolActivity[];
}
