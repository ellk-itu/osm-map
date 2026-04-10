import { createRef, useEffect } from "react";
import { OsmData } from "./osmData/OsmData";
import { MapRenderer } from "./mapRenderer/MapRenderer";

interface MapProps extends React.ComponentProps<"canvas"> {
  width: number;
  height: number;
  osmData: OsmData;
}

export default function (props: MapProps) {
  const canvasRef = createRef<HTMLCanvasElement>();
  const osm = props.osmData;
  const renderRef = createRef<MapRenderer>();

  useEffect(() => {
    if (!canvasRef.current) {
      return;
    }

    renderRef.current = new MapRenderer(osm, canvasRef.current);

    for (let way of osm.ways) {
      setTimeout(() => {
        renderRef.current?.drawWay(way);
      }, 1);
    }
  }, [canvasRef]);

  return <canvas {...props} ref={canvasRef}></canvas>;
}
