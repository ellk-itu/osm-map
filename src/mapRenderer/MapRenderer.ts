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
  init: () => void;

  constructor(canvas: HTMLCanvasElement) {
    this.canvas = canvas;
    this.context = canvas.getContext("2d") as CanvasRenderingContext2D;
    this.mouseDown = false;
    this.pos = { x: 0, y: 0 };
    this.scale = 1;

    this.init = async () => {
      window.addEventListener("mousemove", (e) => {
        if (this.mouseDown) {
          this.pos.x += e.movementX;
          this.pos.y += e.movementY;
          this.render();
        }
      });

      canvas.addEventListener("wheel", (e) => {
        if (!e.ctrlKey) {
          this.scale += e.deltaY * 0.001;
          this.render();
          e.preventDefault();
        }
      });

      await invoke("register_canvas", {
        params: {
          width: canvas.width,
          height: canvas.height,
        },
      });
    };

    this.init();
  }

  private async drawWay(
    points: Uint16Array,
    style: { color?: string; stroke?: { width: number; color: string } },
  ) {
    this.context.beginPath();

    for (let i = 0; i < points.length; i += 2) {
      const x = (points[i] + this.pos.x) * this.scale;
      const y = (points[i + 1] + this.pos.y) * this.scale;

      if (i == 0) {
        this.context.moveTo(x, y);
        continue;
      }

      this.context.lineTo(x, y);
    }

    if (
      points[0] === points[points.length - 2] &&
      points[1] === points[points.length - 1]
    ) {
      this.context.closePath();
      this.context.fillStyle = style.color ?? "rgba(0,0,0,0)";
      this.context.fill();
    }

    if (style.stroke) {
      this.context.lineWidth = style.stroke.width;
      this.context.strokeStyle = style.stroke.color;
      this.context.stroke();
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

    for (let way_id of wayOrder.flat()) {
      const way = ways[way_id];
      const coords = new Uint16Array(
        await invoke<ArrayBuffer>("get_viewport_coords", {
          nodeIds: way.node_refs,
        }),
      );

      this.drawWay(coords, {
        color: this.getFillStyle(way.tags),
        stroke: this.getStrokeStyle(way.tags),
      });
    }
  }

  private render() {
    this.context.fillStyle = getColor(COLOR_NAMES["blue"], LIGHT_VALUES[200]);
    this.context.fillRect(0, 0, this.canvas.width, this.canvas.height);
    this.drawWays();
  }

  start() {
    this.render();
  }
}
