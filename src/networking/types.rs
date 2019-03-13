use crate::map::mesh::{Mesh, MeshJson};
use super::{SubMsg};

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceiveTypeWrapper {
    MapComponentTag(MapCompTag),
    SubMsg(SubMsg),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MapCompTag {
    name: String,
    data: Vec<usize>
}
