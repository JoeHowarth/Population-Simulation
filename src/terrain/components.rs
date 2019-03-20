use specs_derive;
use specs::prelude::*;
use fnv::FnvHashMap;
use nalgebra_glm::{vec2, Vec2};
use crate::terrain::mesh::Mesh;
use arrayvec::ArrayVec;
use std::iter::FromIterator;
use vec_map::VecMap;


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

const MAX_TILES_PER_REGION: usize = 31;

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct Region {
    pub id: usize,
    pub tiles: ArrayVec<[TileID; MAX_TILES_PER_REGION ]>,
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

#[derive(Component, Add, Mul, Div, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct TileTopography {
    pub height: f32,
    pub position: Vec2,
    pub flux: f32,
    pub slope: f32,
    pub hillratio: f32,
    pub area: f32,
}

#[derive(Component, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct RegionTopography {
    pub tiles: u8,
    pub height: f32,
    pub position: Vec2,
    pub flux: f32,
    pub slope: f32,
    pub hillratio: f32,
    pub area: f32,
}

#[derive(Component, Debug, PartialOrd, PartialEq)]
pub struct TileAdjacency {
    pub id: usize,
    pub nbs: ArrayVec<[usize; 3]>,
    pub nbs_entity: ArrayVec<[Entity; 3]>,
    pub is_edge: bool,
}

#[derive(Component, Debug, PartialOrd, PartialEq)]
pub struct RegionAdjacency {
    id: usize,
    pub nbs: ArrayVec<[usize; 32]>,
    pub nbs_entity: Option<ArrayVec<[Entity; 32]>>,
}

impl Region {
    pub fn from_tile(tile: TileID) -> Region {
        let mut tiles = ArrayVec::new();
        tiles.push(tile);
        Region {
            id: tile.id,
            tiles,
        }
    }

    pub fn merge(a: usize, b_id: usize, all: &mut VecMap<Region>) {
        let mut r = all.remove(a).unwrap();
        let b = all.remove(b_id).unwrap();


        dbg!(r.tiles.len());
        dbg!(b.tiles.len());
        for t in &b.tiles {
            if !r.tiles.contains(&t) {
                r.tiles.push(*t);
            }
        }

        all.insert(a, r);
    }
}

impl RegionAdjacency {
    pub fn from_tile(tile: &TileAdjacency) -> RegionAdjacency {
        RegionAdjacency {
            id: tile.id,
            nbs: ArrayVec::from_iter(tile.nbs.iter().map(|&x| x)),
            nbs_entity: None,
        }
    }

    /// ASSERT self in b AND b in self
    pub fn merge(a_id: usize, b_id: usize, all: &mut VecMap<RegionAdjacency>) {
        let mut a = all.remove(a_id).unwrap();
        let b = all.remove(b_id).unwrap();

//        dbg!(&all);
        let mut count = 0;
        // update b.id -> self.id in regions adjacent to b
        for &nb_idx in b.nbs.iter().filter(|&&x| x != a_id && x != b_id) {
            if nb_idx == a_id {
                continue
            }
//            dbg!((&b, &a));
//            dbg!(nb_idx);
//            dbg!(count);
            let nb = &mut all[nb_idx];

            // get idx of b in its neighbor
            let idx = nb.nbs.iter()
                        .position(|&i| i == b.id)
                        .expect("b in a.adj, but a not in b.adj");

            if !nb.nbs.contains(&a.id) {
                // if the neighbor doesn't border merged region, add it
                nb.nbs[idx] = a.id;
            } else {
                // if both bordered, then remove b
                nb.nbs.swap_remove(idx);
            }
            count += 1;
        }
        // remove b from a
        if let Some(b_in_a) = a.nbs.iter().position(|&x| b_id == x)
        {
            a.nbs.swap_remove(b_in_a);
        }

        dbg!(a.nbs.len());
        dbg!(b.nbs.len());
        // merge adjacency lists
        for nb in &b.nbs {
            if !a.nbs.contains(&nb) && *nb != a_id {
                a.nbs.push(*nb);
                dbg!(a.nbs.len());
            }
        }
        // above, for (optional) entities
        if let Some(nbs_entity) = &mut a.nbs_entity {
            if let Some(nbs_e) = &b.nbs_entity {
                for nbe in nbs_e {
                    if !nbs_entity.contains(nbe) {
                        nbs_entity.push(*nbe);
                    }
                }
            }
        }

        all.insert(a_id, a);
    }
}

impl TileAdjacency {
    pub fn new(id: usize, nbs: &[usize], t2e: &Tile2Entity) -> Self {
        let nbs_entity = nbs.iter()
                            .map(|&i| t2e[i])
                            .collect();
        TileAdjacency {
            id,
            nbs: ArrayVec::from_iter(nbs.iter().map(|&x| x)),
            nbs_entity,
            is_edge: nbs.len() < 3,
        }
    }
}

impl RegionTopography {
    pub fn from_tile(tile: &TileTopography) -> RegionTopography {
        RegionTopography {
            tiles: 1,
            height: tile.height,
            position: tile.position,
            flux: tile.flux,
            slope: tile.slope,
            hillratio: tile.hillratio,
            area: tile.area,
        }
    }

    pub fn merge(a: usize, b: usize, all: &mut VecMap<RegionTopography>) {
        let mut r = all.remove(a).unwrap();
        let b = all.remove(b).unwrap();

        r.flux += b.flux;
        r.area += b.area;
        let n = r.tiles as f32;
        let m = b.tiles as f32;
        r.height = (n * r.height + n * b.height) / (n + n);
        r.position = (r.position * n + b.position * n) / (n + n);
        r.slope = (r.slope * n + b.slope * n) / (n + n);
        r.hillratio = (r.hillratio * n + b.hillratio * n) / (n + n);
        r.tiles += b.tiles;

        all.insert(a, r);
    }
}

impl TileTopography {
    pub fn new(mesh: &Mesh, i: usize) -> Self {
        let mut hillratio = {
            let mean_slope: f32 = mesh.adj[i].iter()
                                             .map(|&j| mesh.slope[j].abs())
                                             .sum::<f32>() / mesh.adj[i].len() as f32;

            let slope_coef = (0.5 * mean_slope + 0.5 * mesh.slope[i]).sqrt() / 2.5;
            let h = mesh.height[i].abs();
            (0.7 + 0.3 * h) * slope_coef
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



