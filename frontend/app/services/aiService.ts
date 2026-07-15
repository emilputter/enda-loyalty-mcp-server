import { MessageType } from "../types/message";


export async function askAI(
    messages: MessageType[]
): Promise<string> {

    const response = await fetch(
        "http://localhost:8080/chat",
        {
            method: "POST",
            headers: {
                "Content-Type": "application/json",
            },
            body: JSON.stringify({
                messages: messages,
            }),
        }
    );


    if(!response.ok){

    console.log("Backend status:", response.status);

    const errorText = await response.text();

    console.log("Backend response:", errorText);

    throw new Error("AI request failed");
}


    const data = await response.json();

    return data.response;
}