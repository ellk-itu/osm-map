import { createRef, useEffect } from "react";
import { MapRenderer } from "./mapRenderer/MapRenderer";

interface MapProps extends React.ComponentProps<"canvas"> {
  width: number;
  height: number;
}

export default function (props: MapProps) {
  const canvasRef = createRef<HTMLCanvasElement>();
  const renderRef = createRef<MapRenderer>();
  useEffect(() => {
    if (!canvasRef.current) {
      return;
    }

    renderRef.current = new MapRenderer(canvasRef.current);
    renderRef.current.start();

    // for (let way of osm.ways) {
    //   setTimeout(() => {
    //     renderRef.current?.drawWay(way);
    //   }, 1);
    // }
  }, [canvasRef]);

  return (
    <canvas
      {...props}
      ref={canvasRef}
      onMouseDown={() =>
        renderRef.current ? (renderRef.current.mouseDown = true) : null
      }
      onMouseUp={() => {
        renderRef.current ? (renderRef.current.mouseDown = false) : null;
      }}
    ></canvas>
  );
}
