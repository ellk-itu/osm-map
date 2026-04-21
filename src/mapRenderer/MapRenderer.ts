import { invoke } from "@tauri-apps/api/core";
import {
  COLOR_NAMES,
  colorConditions,
  getColor,
  LIGHT_VALUES,
  strokeConditions,
} from "./Style";
import { wayCache } from "./cache/cache";
import { Way } from "@/types/osmData";

type Point = { x: number; y: number };

export class MapRenderer {
  public mouseDown: boolean;

  canvas: HTMLCanvasElement;
  context: CanvasRenderingContext2D;
  pos: Point;
  scale: number;
  layers: HTMLCanvasElement[];
  init: () => void;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.context = canvas.getContext("2d") as CanvasRenderingContext2D;
    this.mouseDown = false;
    this.pos = { x: 0, y: 0 };
    this.scale = 1;
    this.layers = new Array(16);

    this.init = async () => {
      this.addListeners();

      await this.registerCanvas();

      this.drawWays();
    };

    this.init();
  }

  /**
   * Adds the functionality to register panning and zooming
   */
  private addListeners() {
    window.addEventListener("mousemove", (e) => {
      if (this.mouseDown) {
        this.pos.x += e.movementX / this.scale;
        this.pos.y += e.movementY / this.scale;
        this.render();
      }
    });

    this.canvas.addEventListener("wheel", (e) => {
      if (!e.ctrlKey) {
        this.scale += e.deltaY * 0.001;
        this.render();
        e.preventDefault();
      }
    });
  }

  /**
   * Registers the canvas parameters to the backend
   */
  private async registerCanvas() {
    await invoke("register_canvas", {
      params: {
        width: this.canvas.width,
        height: this.canvas.height,
      },
    });
  }

  private async drawWay(
    points: Uint16Array,
    style: { color?: string; stroke?: { width: number; color: string } },
    canvas?: HTMLCanvasElement,
  ) {
    const context = canvas?.getContext("2d") ?? this.context;

    context.beginPath();

    for (let i = 0; i < points.length; i += 2) {
      const x = (points[i] + this.pos.x) * this.scale;
      const y = (points[i + 1] + this.pos.y) * this.scale;

      if (i == 0) {
        context.moveTo(x, y);
        continue;
      }

      context.lineTo(x, y);
    }

    if (
      points[0] === points[points.length - 2] &&
      points[1] === points[points.length - 1]
    ) {
      context.closePath();
      context.fillStyle = style.color ?? "rgba(0,0,0,0)";
      context.fill();
    }

    if (style.stroke) {
      context.lineWidth = style.stroke.width;
      context.strokeStyle = style.stroke.color;
      context.stroke();
    }
  }

  private getFillStyle(tags: Way["tags"]) {
    for (let [condition, color] of colorConditions) {
      if (condition(tags)) {
        return color;
      }
    }
  }

  private getStrokeStyle(tags: Way["tags"]) {
    for (let [condition, style] of strokeConditions) {
      if (condition(tags)) {
        return style;
      }
    }
  }

  private async drawWays() {
    const ways = await wayCache.getCache();
    const wayOrder = await invoke<string[][]>("get_sorted_ways");

    for (let i = 0; i < wayOrder.length; i++) {
      const layer = wayOrder[i];

      if (layer.length === 0) {
        continue;
      }

      const canvas = document.createElement("canvas");
      canvas.width = this.canvas.width;
      canvas.height = this.canvas.height;

      for (let way_id of layer) {
        const way = ways[way_id];
        const coords = new Uint16Array(
          await invoke<ArrayBuffer>("get_viewport_coords", {
            nodeIds: way.node_refs,
          }),
        );

        this.drawWay(
          coords,
          {
            color: this.getFillStyle(way.tags),
            stroke: this.getStrokeStyle(way.tags),
          },
          canvas,
        );
      }
    }
  }

  private render() {
    this.context.fillStyle = getColor(COLOR_NAMES["blue"], LIGHT_VALUES[200]);
    this.context.fillRect(0, 0, this.canvas.width, this.canvas.height);
    for (let canvas of this.layers) {
      if (!canvas) {
        continue;
      }
      this.context.drawImage(canvas, 1000, 1000);
    }
  }

  public start() {
    this.render();
  }
}
