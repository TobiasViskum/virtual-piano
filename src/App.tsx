import { useEffect, useState } from "react";
import "./App.css";
import { PianoView } from "./PianoView";
import { Event, emit, listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

function App() {
  const [isRecording, setIsRecording] = useState(false);
  const [isListening, setIsListening] = useState(false);

  useEffect(() => {
    async function updateIsListening() {
      setIsListening(await invoke<boolean>("is_listening"));
    }
    updateIsListening();

    listen<{ event_type: string; intensity: number; key_string: string; key_id: number }>(
      "pianoevent",
      (ev) => {
        const event = new CustomEvent("pianoevent", { detail: ev.payload });
        document.dispatchEvent(event);
      }
    );
  }, []);

  async function startPianoListen() {
    setIsListening((prev) => {
      if (prev) {
        invoke<boolean>("kill_piano_listener");
      } else {
        invoke<boolean>("spawn_piano_listener");
      }

      return !prev;
    });
  }

  async function startPianoRecorder() {
    setIsRecording((prev) => {
      if (prev) {
        invoke<boolean>("end_piano_recording", {name: "First recording"});

        setTimeout(() => {
          invoke<boolean>("play_recording")
        }, 500);
      } else {
        invoke<boolean>("spawn_piano_recorder");
      }

      return !prev;
    });
  }

  const stopColor = "bg-red-500 h-8 w-8 disabled:opacity-80 disabled:cursor-not-allowed";
  const startColor = "bg-green-500 h-8 w-8 disabled:opacity-80 disabled:cursor-not-allowed";

  return (
    <div className="max-h-[calc(100svh-0px)] h-[100svh] flex flex-col">
      <div className="flex gap-x-8">
        <div>
          <p>Listen:</p>
          <button
            disabled={isRecording}
            onMouseDown={startPianoListen}
            className={isListening ? startColor : stopColor}
          ></button>
        </div>
        <div>
          <p>Record:</p>
          <button
            disabled={isListening}
            onMouseDown={startPianoRecorder}
            className={isRecording ? startColor : stopColor}
          ></button>
        </div>
      </div>
      <div className="bg-red-500 w-screen mt-auto">
        <PianoView />
      </div>
    </div>
  );
}

export default App;
