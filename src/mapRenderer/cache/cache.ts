import { invoke } from "@tauri-apps/api/core";
import { WayCache } from "./types";
import { TagTagData, Way } from "@/types/osmData";

const wayCache: { current?: WayCache[] } = {};

const cacheWays = async () => {
  const ways = await invoke<[[Way, TagTagData]]>("get_ways");
  const cache: WayCache[] = [];

  for (let [way, tagTagData] of ways) {
    const buffer = await invoke<ArrayBuffer>("get_way_points", way);
    const bytes = new Uint16Array(buffer);

    cache.push(
      Object.assign(
        {} as WayCache,
        way,
        { tagTagData },
        { linePositions: bytes },
      ),
    );

    wayCache.current = cache;
  }
};

export const getWayCache = async () => {
  if (!wayCache.current) {
    await cacheWays();
  }

  return wayCache.current as WayCache[];
};
