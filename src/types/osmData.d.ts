// pub struct Way {
//     pub(crate) id: String,
//     pub(crate) user: String,
//     pub(crate) changeset: String,
//     pub(crate) version: String,
//     pub(crate) timestamp: String,
//     pub(crate) visible: Option<bool>,
//     pub(crate) uid: String,
//     pub(crate) tags: HashMap<String, String>,
//     pub(crate) node_refs: Vec<String>,
// }
export type Way = {
  id: string;
  user: string;
  changeset: string;
  version: string;
  timestamp: string;
  visible?: boolean;
  uid: string;
  tags: Record<string, string>;
  node_refs: string[];
};

// pub struct Node {
//     pub(crate) id: String,
//     pub(crate) lon: f64,
//     pub(crate) lat: f64,
//     pub(crate) user: String,
//     pub(crate) changeset: String,
//     pub(crate) version: String,
//     pub(crate) timestamp: String,
//     pub(crate) visible: Option<bool>,
//     pub(crate) uid: String,
//     pub(crate) tags: HashMap<String, String>,
// }

interface Node {
  id: string;
  lon: number;
  lat: number;
  user: string;
  changeset: string;
  version: string;
  timestamp: string;
  visible?: boolean;
  uid: string;
  tags: Record<string, string>;
}
