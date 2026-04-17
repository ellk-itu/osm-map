import { TagTagData, Way } from "@/types/osmData";

export interface WayCache extends Way {
  linePositions: Uint32Array;
  tagTagData: TagTagData;
}
