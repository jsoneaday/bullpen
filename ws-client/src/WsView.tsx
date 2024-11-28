import { useState, useEffect, useCallback } from "react";

const poolAddress = "8sLbNZoA1cfnvMJLPfp98ZLAnFSYCFApfJKMbiXNLwxj";

const WsView = () => {
  const [messages, setMessages] = useState([]);
  const [socket, setSocket] = useState<WebSocket | null>(null);
  const [isConnected, setIsConnected] = useState(false);

  // Initialize WebSocket connection
  useEffect(() => {
    const ws = new WebSocket("ws://0.0.0.0:4000/ray");

    ws.onopen = () => {
      console.log("Connected to WebSocket");
      setIsConnected(true);
      setSocket(ws);

      ws.send(poolAddress);
    };

    ws.onmessage = (event) => {
      console.log("ws message:", event);
      // setMessages((prev) => [...prev, event.data]);
    };

    ws.onclose = () => {
      console.log("Disconnected from WebSocket");
      setIsConnected(false);
    };

    ws.onerror = (error) => {
      console.error("WebSocket error:", error);
    };

    // Cleanup on component unmount
    return () => {
      ws.close();
    };
  }, []); // Empty dependency array means this effect runs once on mount

  // Function to close the connection
  const handleDisconnect = useCallback(() => {
    if (socket) {
      socket.close();
      setSocket(null);
    }
  }, [socket]);

  return (
    <div className="p-4">
      <div className="mb-4">
        <div className="flex items-center gap-2">
          <div
            className={`w-3 h-3 rounded-full ${
              isConnected ? "bg-green-500" : "bg-red-500"
            }`}
          />
          <span>{isConnected ? "Connected" : "Disconnected"}</span>
        </div>
        {isConnected && (
          <button
            onClick={handleDisconnect}
            className="mt-2 px-4 py-2 bg-red-500 text-white rounded hover:bg-red-600"
          >
            Disconnect
          </button>
        )}
      </div>

      <div className="border rounded-lg p-4 max-h-96 overflow-y-auto">
        <h2 className="text-lg font-semibold mb-2">Messages:</h2>
        {messages.map((message, index) => (
          <div key={index} className="p-2 border-b last:border-b-0">
            {message}
          </div>
        ))}
        {messages.length === 0 && (
          <p className="text-gray-500">No messages received yet</p>
        )}
      </div>
    </div>
  );
};

export default WsView;
