import { ChangeEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import "./Colors.css";
import readFileAsText from "@common/readFileAsText";
import Map from "./Map";

function App() {
  const [osmLoaded, setOsmLoaded] = useState<boolean>(false);

  const handleInput = async (
    e: ChangeEvent<HTMLInputElement, HTMLInputElement>,
  ) => {
    const file = (e.target.files as FileList)[0];
    const fileString = await readFileAsText(file);
    await invoke("parse_osm", { data: fileString });
    console.log(await invoke("get_osm_test"));
    setOsmLoaded(true);
  };

  return (
    <main className="container">
      {osmLoaded ? <Map width={1000} height={1000} /> : undefined}
      <input
        type="file"
        name="click"
        className="file-input"
        onChange={handleInput}
      />
    </main>
  );
}

export default App;
