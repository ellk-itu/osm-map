// pub struct TagWithChildren {
//     pub(crate) tag_type: String,
//     pub(crate) parameters: HashMap<String, String>,
//     pub(crate) children: Option<Box<Vec<TagWithChildren>>>,
// }
export interface TagWithChildren {
  tag_type: string;
  parameters: Record<string, string>;
  children?: TagWithChildren[];
}

// pub type TagTree = Vec<Box<TagWithChildren>>;
export type TagTree = TagWithChildren[];

// pub type Ways = TagTree;
export type Ways = TagTree;

// pub type Nodes = TagTree;
export type Nodes = TagTree;

// #[derive(Debug, Serialize, Deserialize)]
// pub struct OsmData {
//     pub(crate) ways: Ways,
//     pub(crate) nodes: Nodes,
//     pub(crate) tree: TagTree,
// }
export interface OsmData {
  ways: Ways;
  nodes: Nodes;
  tree: TagTree;
  bounds: TagWithChildren;
}
