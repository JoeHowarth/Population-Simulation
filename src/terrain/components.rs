use specs_derive;
use specs::prelude::*;
use fnv::FnvHashMap;
use nalgebra_glm::{vec2, Vec2};
use crate::terrain::mesh::Mesh;


// resource for going the other way
pub type Tile2Entity = Vec<Entity>;
pub type Region2Entity = Vec<Entity>;

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
pub struct TileID {
    pub id: usize
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Ord, Eq)]
/// Which region a tile belongs to
pub struct RegionID {
    pub id: usize
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Region {
    pub id: usize,
    pub tiles: Vec<TileID>,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Weather {
    /// amount of rainfall above or below 'base'
    pub rainfall: i8,
    /// average temperature in fahrenheit  (?)
    pub temperature: i8,
    /// how severe
    pub severe_weather: u8,
    // flood, hurricane, tornado, winter storm, monsoon etc.
    // pub severe_weather_type: 'enum'
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct RiverID {
    pub id: usize
    // pub name: String
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct LandMassID {
    pub id: usize
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileTopography {
    pub height: f32,
    pub position: Vec2,
    pub flux: f32,
    pub slope: f32,
    pub hillratio: f32,
    pub area: f32,
}


#[derive(Component, Debug, PartialOrd, PartialEq)]
pub struct TileAdjacency {
    pub nbs: Vec<usize>,
    pub nbs_entity: Vec<Entity>,
    pub is_edge: bool,
}


impl TileTopography {
    pub fn new(mesh: &Mesh, i: usize) -> Self {

        let mut hillratio = {
            let mean_slope: f32 = mesh.adj[i].iter()
                .map(|&j| mesh.slope[j].abs())
                .sum::<f32>() / mesh.adj[i].len() as f32;

            let slope_coef = (0.5 * mean_slope + 0.5 * mesh.slope[i] ).sqrt() / 2.5;
            let h = mesh.height[i].abs();
            (0.7 + 0.3 *h) * slope_coef
        };

        if mesh.height[i] < 0. {
            hillratio = -1.
        }

        TileTopography {
                height: mesh.height[i],
                position: mesh.centroids[i],
                flux: mesh.flux[i],
                slope: mesh.slope[i],
                area: mesh.area[i],
                hillratio,
        }
    }
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



