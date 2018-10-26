use specs_derive;
use specs::prelude::*;
use fnv::FnvHashMap;


#[derive(Component, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileID(pub usize);

// resource for going the other way
pub type Tile2Entity = FnvHashMap<usize, Entity>;


#[derive(Component, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileTopography {
    pub height: f32,
    pub position: [f32; 2],
}

#[derive(Component, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileAdjacency {
    pub neighbors: Vec<TileID>,
    pub is_edge: bool,
}



