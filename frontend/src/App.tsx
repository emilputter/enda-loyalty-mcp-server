import ChatBox from "./components/ChatBox";

function App() {

    return (
        <div className="min-h-screen bg-gray-100 flex items-center justify-center">

            <div className="w-full max-w-2xl bg-white rounded-xl shadow-lg p-6">

                <h1 className="text-3xl font-bold text-center mb-6">
                    ENDA AI Assistant
                </h1>

                <ChatBox />

            </div>

        </div>
    );
}

export default App;