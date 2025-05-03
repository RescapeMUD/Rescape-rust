import { useEffect, useRef, useState } from 'react';
import './App.css';

function App() {
    const [log, setLog] = useState<string[]>([]);
    const [input, setInput] = useState('');
    const [ws, setWs] = useState<WebSocket | null>(null);
    const logRef = useRef<HTMLDivElement>(null);

    useEffect(() => {
        if (logRef.current) {
            logRef.current.scrollTop = logRef.current.scrollHeight;
        }
    }, [log]);

    const connectToGame = () => {
      const socket = new WebSocket('ws://localhost:4000/ws');
  
      socket.onopen = () => {
          console.log("✅ WebSocket connected");
          setLog(['Connected to Rescape MUD.', 'Enter your name:']);
      };
  
      socket.onerror = (err) => {
          console.error("❌ WebSocket error:", err);
      };
  
      socket.onclose = () => {
          console.log("🔌 WebSocket disconnected");
          setLog((prev) => [...prev, 'Connection closed.']);
      };
  
      socket.onmessage = (event) => {
          setLog((prev) => [...prev, event.data]);
      };
  
      setWs(socket);
  };
  

    const sendCommand = () => {
        if (!ws || ws.readyState !== WebSocket.OPEN) return;

        const command = input.trim();
        if (command !== '') {
            ws.send(command);
            setLog((prev) => [...prev, `> ${command}`]);
            setInput('');

            if (command.toLowerCase() === 'exit') {
                ws.close();
            }
        }
    };

    const handleKeyPress = (e: React.KeyboardEvent<HTMLInputElement>) => {
        if (e.key === 'Enter') {
            sendCommand();
        }
    };

    return (
        <div>
            <h1>Rescape MUD</h1>
            <button onClick={connectToGame}>Connect</button>
            <div id="log" ref={logRef}>
                {log.map((line, index) => (
                    <p key={index} dangerouslySetInnerHTML={{ __html: line.replace(/\n/g, '<br>') }} />
                ))}
            </div>
            <input
                type="text"
                value={input}
                onChange={(e) => setInput(e.target.value)}
                onKeyDown={handleKeyPress}
                placeholder="Type a command..."
                disabled={!ws || ws.readyState !== WebSocket.OPEN}
            />
            <button onClick={sendCommand}>Send</button>
        </div>
    );
}

export default App;