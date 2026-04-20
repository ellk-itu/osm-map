import { invoke } from "@tauri-apps/api/core";
import { Node, Way } from "@/types/osmData";

class Cache<T> {
  cache: { current: T | null };

  constructor() {
    this.cache = { current: null };
  }

  public async setCache(t: T) {}

  public async getCache(...args: any): Promise<T | null> {
    return null;
  }
}

class WayCache extends Cache<Record<string, Way>> {
  constructor() {
    super();
  }

  private async requestWays(way_ids?: string[]) {
    const res = await invoke<Record<string, Way>>("get_ways", { way_ids });
    this.setCache(res);
    return res;
  }

  public async setCache(t: Record<string, Way>): Promise<void> {
    if (!this.cache.current) {
      this.cache.current = t;
      return;
    }

    Object.assign(this.cache.current, t);
  }

  public async getCache(way_ids?: string[]): Promise<Record<string, Way>> {
    if (!way_ids) {
      return await this.requestWays();
    }

    if (!this.cache.current) {
      return await this.requestWays(way_ids);
    }

    const ways: Record<string, Way> = {};
    const unknowns = [];
    for (let id of way_ids) {
      const res = this.cache.current[id];

      if (res) {
        ways[res.id] = res;
      }

      unknowns.push(id);
    }

    return Object.assign(this.cache.current, this.requestWays(unknowns));
  }
}

export const wayCache = new WayCache();

class NodeCache extends Cache<Record<string, Node>> {
  constructor() {
    super();
  }

  private async requestNodes(node_ids?: string[]) {
    const res = await invoke<Record<string, Node>>("get_nodes", {
      node_ids,
    });
    this.setCache(res);
    return res;
  }

  public async setCache(t: Record<string, Node>): Promise<void> {
    if (!this.cache.current) {
      this.cache.current = t;
      return;
    }

    Object.assign(this.cache.current, t);
  }

  public async getCache(node_ids?: string[]): Promise<Record<string, Node>> {
    if (!node_ids) {
      return await this.requestNodes();
    }

    if (!this.cache.current) {
      return await this.requestNodes(node_ids);
    }

    const ways: Record<string, Node> = {};
    const unknowns = [];
    for (let id of node_ids) {
      const res = this.cache.current[id];

      if (res) {
        ways[res.id] = res;
      }

      unknowns.push(id);
    }

    return Object.assign(this.cache.current, this.requestNodes(unknowns));
  }
}

export const nodeCache = new NodeCache();
