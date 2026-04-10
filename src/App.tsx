import { ChangeEvent, useState } from "react";
import { invoke } from "@tauri-apps/api/core";
import "./App.css";
import readFileAsText from "@common/readFileAsText";
import { OsmData } from "./osmData/OsmData";
import Map from "./Map";

function App() {
  const [osmData, setOsmData] = useState<OsmData>();

  const handleInput = async (
    e: ChangeEvent<HTMLInputElement, HTMLInputElement>,
  ) => {
    const file = (e.target.files as FileList)[0];
    const fileString = await readFileAsText(file);
    const data = (await invoke("parse_osm", { data: fileString })) as OsmData;
    setOsmData(new OsmData(data));
  };

  return (
    <main className="container">
      {osmData ? (
        <Map width={1000} height={1000} osmData={osmData} />
      ) : undefined}
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
