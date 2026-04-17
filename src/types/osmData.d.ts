// pub struct Way {
//     pub(crate) id: String,
//     pub(crate) user: String,
//     pub(crate) changeset: String,
//     pub(crate) version: String,
//     pub(crate) timestamp: String,
//     pub(crate) visible: bool,
//     pub(crate) uid: String,
// }
export type Way = {
  id: string;
  user: string;
  changeset: string;
  version: string;
  timestamp: string;
  visible: boolean;
  uid: string;
};

// pub type TagTagData = HashMap<String, String>;
export type TagTagData = Record<string, string>;
