export async function askAI(message: string): Promise<string> {

    // Temporary fake AI response
    return new Promise((resolve) => {

        setTimeout(() => {

            resolve(
                `AI response to: ${message}`
            );

        }, 1000);

    });
}