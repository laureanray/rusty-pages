import { invoke } from "@tauri-apps/api/tauri";
import "./App.css";

function App() {
  async function read() {
    await invoke("read", { name });
  }

  return (
    <div className="container">
      <button onClick={read}> Read </button>
    </div>
  );
}

export default App;
