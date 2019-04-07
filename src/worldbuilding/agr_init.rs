use super::*;
use crate::{
    terrain::{
        components::*,
        mesh::{Mesh, MeshJson},
    },
    misc::normalize::*,
    pop::components::RegionPop,
    worldbuilding::terrain_init::{get_rivers, RIVER_FLUX_THRESH},
    prelude::*,
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
use std::intrinsics::unaligned_volatile_load;


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

// TODO use weighted Node
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

    let it = fertility.iter().filter(|&&f| f > 0.);
    let mean = it.clone().sum::<f32>() / it.count() as f32;
    fertility.iter_mut().for_each(|f| *f /= mean); // change
    dbg!(&fertility);

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
pub fn init_farm_data((base, reg, topo, pop, mut farm, entities): (ReadStorage<RegBaseFarmData>, ReadStorage<Region>, ReadStorage<RegionTopography>, ReadStorage<RegionPop>, WriteStorage<FarmData>, Entities)) {
    let mut count = 0;
    let mut rng = SmallRng::from_entropy();
    for (base, pop, reg, topo, e) in (&base, &pop, &reg, &topo, &entities).join() {
        count += 1;
        let &RegBaseFarmData { fertility, arable } = base;

        let high_yield = base_yield(2, arable, fertility);
        let needed_grain = pop.cohorts.iter().fold(0., |acc, c| acc + grain_for_cohort(c));
        if high_yield < needed_grain {
            warn!("Too many people for yield to supply, Needed: {}, Max Yield: {}", needed_grain, high_yield);
        }

        let needed_area = inverse_yield(2, needed_grain, fertility);
        let cleared = (needed_area * rng.gen_range(1.01, 1.5)).min(arable);
        let auc = needed_area.min(arable);

        dbg!(needed_grain / high_yield);
        farm.insert(e, FarmData {auc, cleared}).unwrap();
    }

    dbg!(count);
}


fn inverse_yield(seed_ratio: u8, bushels: f32, fertility: f32) -> f32 {
    let bph = BUSHELS_PER_HECTARE_2_TO_1 * (seed_ratio - 1) as f32 * SEED_RATIO * fertility;

    // km = bushels / bushels_per_km
    bushels / (bph * 1000.)
}


