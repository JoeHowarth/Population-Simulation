use super::*;
use crate::{
    terrain::{
        components::*,
        init::{get_rivers, RIVER_FLUX_THRESH},
        mesh::{Mesh, MeshJson},
    },
    misc::normalize::*,
    pop::components::RegionPop,
};
use fnv::{FnvHashMap, FnvHashSet};
use specs::prelude::*;
use ord_subset::*;
use vec_map::VecMap;
use failure::Error;
use ord_subset::*;
use std::{
    collections::BinaryHeap,
    cmp::{Ordering, PartialOrd},
    iter::FromIterator,
    collections::VecDeque,
};


pub fn register_agr_ecs(world: &mut World) {
    {
        world.register::<FarmData>();
        world.register::<BaseFarmData>();
        world.register::<RegBaseFarmData>();
        world.register::<FoodStock>();


        let mesh: ReadExpect<Mesh> = world.system_data();
        let updater: Read<LazyUpdate> = world.system_data();
        let tile2entity: Read<Tile2Entity> = world.system_data();

        let fd = get_base_farm_data(&mesh, world.system_data());
        for (i, farmdata) in fd.into_iter().enumerate() {
            match farmdata {
                Some(data) => {
                    let &e = tile2entity.get(i).expect("entity from TileID not found");
                    updater.insert(e, data);
                }
                None => {}
            }
        }
    }
    world.maintain();
}

#[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
struct Node(f32, f32, usize, usize);

// flux, strength in [1,4], tileID {id <--- } curr, last
impl Eq for Node {}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        match self.0.partial_cmp(&other.0) {
            Some(Ordering::Equal) => self.1.partial_cmp(&other.1).expect("No ordering comparing Node, probably NaN"),
            Some(x) => x,
            None => Ordering::Less
        }
    }
}

pub fn get_base_farm_data(mesh: &Mesh, (t2e, topo): (Read<Tile2Entity>, ReadStorage<TileTopography>)) -> Vec<Option<BaseFarmData>> {
    let Mesh { height: h, flux, adj, slope, .. } = mesh;
    // lower threshold than 'real' rivers
    let rivers = get_rivers(mesh, RIVER_FLUX_THRESH * 0.5);


    let mut heap = BinaryHeap::from_iter(rivers.iter().flatten()
                                               .map(|&k| {
                                                   let strength = (flux[k] / (RIVER_FLUX_THRESH * 3.0)).min(1.0).sqrt();
                                                   let shifted = strength * 0.2 + 0.80;
                                                   Node(flux[k].min(RIVER_FLUX_THRESH), shifted, k, k)
                                               }));

    const BASE_CARRIED: f32 = 1.0;
    let mut fertility = vec![0.; h.len()];

    // breadth first to carry flux out
    while let Some(Node(f, strength, i, last)) = heap.pop() {
        let downhill_coef = {
            let delta_h = h[i] - h[last]; // between [0,1)
            if delta_h > 0. {
                1. - delta_h * 1.5 // if deltaH large, make coef small (ie reduces spread)
            } else {
                1. - delta_h * 0.2 // aid going downhill less than hurt uphill
            }
        };


        // cap flux power
        let carried = (BASE_CARRIED * strength * downhill_coef).min(0.99);
        let new_flux = f * carried;
        if fertility[i] < new_flux {
            fertility[i] = new_flux;
            for &j in &adj[i] {
                if h[j] > 0. {
                    heap.push(Node(fertility[i], strength, j, i));
                }
            }
        }
    }

    fertility = normalize_vec(fertility);


    fertility.iter().enumerate().map(|(i, &fertility)| {
        if h[i] < 0. {
            return None;
        }

        let e = t2e[i];
        let TileTopography { hillratio, area, .. } = topo.get(e).unwrap();
        let arable = area * hillratio;

        Some(BaseFarmData { fertility, arable })
    }).collect()
}


// 'FarmData' is regional for now
pub fn init_farm_data((base, pop, reg, topo, mut farm, entities): (ReadStorage<RegBaseFarmData>, ReadStorage<RegionPop>, ReadStorage<Region>, ReadStorage<RegionTopography>, WriteStorage<FarmData>, Entities)) {
    for (base, pop, reg, topo, mut farm, e) in (&base, &pop, &reg, &topo, &mut farm, &entities).join() {
        let area = topo.area * 25.; // map area is 1/5 'rea' area TODO correct area
        let RegBaseFarmData { fertilty, arable } = base;
        let cleared = 
    }
}
