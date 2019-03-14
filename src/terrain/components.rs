use specs_derive;
use specs::prelude::*;
use fnv::FnvHashMap;
use nalgebra_glm::{vec2, Vec2};
use crate::terrain::mesh::Mesh;


// resource for going the other way
pub type Tile2Entity = Vec<Entity>;

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileID {
    pub id: usize
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct River {
    pub id: usize
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct LandMass {
    pub id: usize
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileTopography {
    pub height: f32,
    pub position: Vec2,
    pub flux: f32,
    pub slope: f32,
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct FarmData {
    pub fertility: f32,
    pub arable_cap: f32,
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Population {
    pub pop: u32,
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct CityData {
    pub gravity: u32,
    pub wealth: f64,
}


#[derive(Component, Debug, PartialOrd, PartialEq)]
pub struct TileAdjacency {
    pub nbs: Vec<usize>,
    pub nbs_entity: Vec<Entity>,
    pub is_edge: bool,
}

//pub fn to_map_displayable<'a, J, T: Component>((data, ids, mesh): J,
//                                 f: fn(T) -> f32)
//                                 -> Vec<T>
//where J: (ReadStorage<'a, T>, ReadStorage<'a, TileID>, Read<'a, Mesh>) + Join
//{
//    let default_value = -10.;
//    let mut out = vec![default_value; mesh.ids.len()];
//    for (d, TileID { id }) in (data, ids).join() {
//        out[id] = f(d);
//    }
//    out
//}



