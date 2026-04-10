import { OsmData } from "@/osmData/OsmData";
import { TagWithChildren } from "@/types/osmData";
import { invoke } from "@tauri-apps/api/core";

type Point = { x: number; y: number };

export class MapRenderer {
  osmData: OsmData;
  canvas: HTMLCanvasElement;
  context: CanvasRenderingContext2D;
  viewportScale: Point;
  coordSize: Point;

  constructor(osmData: OsmData, canvas: HTMLCanvasElement) {
    this.osmData = osmData;
    this.canvas = canvas;
    this.context = canvas.getContext("2d") as CanvasRenderingContext2D;
    this.coordSize = this.#getCoordtSize();
    this.viewportScale = this.#getViewportScale();
  }

  #getCoordtSize(): Point {
    const minlat = parseFloat(this.osmData.bounds.parameters["minlat"]);
    const maxlat = parseFloat(this.osmData.bounds.parameters["maxlat"]);
    const minlon = parseFloat(this.osmData.bounds.parameters["minlon"]);
    const maxlon = parseFloat(this.osmData.bounds.parameters["maxlon"]);
    return {
      x: maxlon - minlon,
      y: maxlat - minlat,
    };
  }

  #getViewportScale(): Point {
    return {
      x: this.canvas.width / this.coordSize.x,
      y: this.canvas.height / this.coordSize.y,
    };
  }

  #coordsToCanvas(tag: TagWithChildren): Point | null {
    const minlat = parseFloat(this.osmData.bounds.parameters["minlat"]);
    const minlon = parseFloat(this.osmData.bounds.parameters["minlon"]);

    const lat = parseFloat(tag.parameters["lat"]);
    const lon = parseFloat(tag.parameters["lon"]);

    if (!lat && !lon) {
      return null;
    }

    return {
      x: (lon - minlon) * this.viewportScale.x,
      y: (lat - minlat) * this.viewportScale.y,
    };
  }

  drawNode(node: TagWithChildren) {
    if (node.tag_type !== "node") {
      throw new Error("node is not node");
    }

    const coord = this.#coordsToCanvas(node);

    if (!coord) {
      throw new Error("node does not have coordinates");
    }

    this.context.fillStyle = "rgb(200 0 0)";
    this.context.fillRect(coord?.x, coord?.y, 10, 10);
  }

  async drawWay(way: TagWithChildren) {
    if (way.tag_type !== "way") {
      throw new Error("node is not node");
    }

    const nodes = await this.osmData.getNodesOfWay(way);

    if (!nodes) {
      throw new Error("Some shit went wrong");
    }

    this.context.lineWidth = 2;
    this.context.fillStyle = "rgba(200,0,0,0.1)";
    this.context.beginPath();

    const first = nodes.splice(0, 1)[0];
    const firstCoords = this.#coordsToCanvas(first);

    this.context.moveTo(firstCoords?.x ?? 0, firstCoords?.y ?? 0);

    for (let node of nodes) {
      const nodeCoordinate = this.#coordsToCanvas(node);

      if (!nodeCoordinate) continue;

      this.context.lineTo(nodeCoordinate?.x, nodeCoordinate?.y);
    }

    if (first.parameters["id"] == nodes[nodes.length - 1].parameters["id"]) {
      this.context.fill();
    }

    this.context.stroke();
  }
}
