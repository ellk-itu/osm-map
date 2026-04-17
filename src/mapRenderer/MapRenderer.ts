import { TagTagData } from "@/types/osmData";
import { invoke } from "@tauri-apps/api/core";
import {
  COLOR_NAMES,
  colorConditions,
  getColor,
  LIGHT_VALUES,
  strokeConditions,
} from "./Style";
import { getWayCache } from "./cache/cache";

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

      await invoke("parse_ways");
    };

    this.init();
  }

  private async drawWay(
    points: Uint32Array,
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

  private getFillStyle(tagTagData: TagTagData) {
    for (let [condition, color] of colorConditions) {
      if (condition(tagTagData)) {
        return color;
      }
    }
  }

  private getStrokeStyle(tagTagData: TagTagData) {
    for (let [condition, style] of strokeConditions) {
      if (condition(tagTagData)) {
        return style;
      }
    }
  }

  private async drawWays() {
    const wayCacheArr = await getWayCache();

    console.log("drawing");

    for (let wayCache of wayCacheArr) {
      const { linePositions, tagTagData } = wayCache;

      this.drawWay(linePositions, {
        color: this.getFillStyle(tagTagData),
        stroke: this.getStrokeStyle(tagTagData),
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
