import type {
  OsmData as Data,
  Nodes,
  TagTree,
  TagWithChildren,
  Ways,
} from "@/types/osmData";
import { invoke } from "@tauri-apps/api/core";

/**
 * Class containing all the logic for osmData
 */
export class OsmData implements Data {
  tree: TagTree;
  nodes: Nodes;
  ways: Ways;
  bounds: TagWithChildren;

  /**
   * Constructor for OsmData
   * @param data Osm Data
   */
  constructor(data: Data) {
    this.tree = data.tree;
    this.nodes = data.nodes;
    this.ways = data.ways;
    this.bounds = data.bounds;
  }

  /**
   * Finds first node that satisfies id (There should only exist one)
   * @param id id of node
   * @returns Node or null if none was found
   */
  async findNodeById(id: string): Promise<TagWithChildren | null> {
    return (await invoke("find_node_by_id", {
      nodes: this.nodes,
      id: id,
    })) as TagWithChildren | null;
  }

  /**
   * Finds first tag that satisfies name
   * @param tags The tags to search through
   * @param name The name of the tag
   * @returns The tag with said name or null if none are found
   */
  async findTagByName(
    tags: TagTree,
    name: string,
  ): Promise<TagWithChildren | null> {
    return await invoke("find_tag_by_name", { tags, name });
  }

  /**
   * Finds all tags that satisfies name
   * @param tags The tags to search through
   * @param name The name of the tag
   * @returns An array of tags with name or null if none are found
   */
  async findAllTagsByName(
    tags: TagTree,
    name: string,
  ): Promise<TagTree | null> {
    return await invoke("find_all_tags_by_name", { tags, name });
  }

  async getNodesOfWay(way: TagWithChildren): Promise<TagTree | null> {
    return await invoke("get_all_nodes_of_way", { way, nodes: this.nodes });
  }
}
