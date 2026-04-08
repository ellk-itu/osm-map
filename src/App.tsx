import { ChangeEvent, useState } from "react";
import reactLogo from "./assets/react.svg";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import readFileAsText from "@common/readFileAsText";

function App() {
  const [greetMsg, setGreetMsg] = useState("");
  const [name, setName] = useState("");

  const handleInput = async (
    e: ChangeEvent<HTMLInputElement, HTMLInputElement>,
  ) => {
    const file = (e.target.files as FileList)[0];
    const fileString = await readFileAsText(file);
    console.log(await invoke("parse_osm", { data: fileString }));
  };

  return (
    <main className="container">
      <input
        type="file"
        name="click"
        id=""
        className="file-input"
        onChange={handleInput}
      />
    </main>
  );
}

export default App;
