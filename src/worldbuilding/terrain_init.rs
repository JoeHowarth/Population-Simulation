use crate::{
    terrain::{
        *,
        components::*,
        mesh::{Mesh, MeshJson},
    },
    agriculture::*,
    misc::normalize::*,
};
use fnv::{FnvHashMap, FnvHashSet};
use specs::prelude::*;
use ord_subset::*;
use vec_map::VecMap;
use failure::Error;
use ord_subset::*;
use std::{
    cmp::Ordering,
    iter::FromIterator,
    collections::{
        VecDeque,
        BinaryHeap,
    },
};
use rand::{
    prelude::*,
    FromEntropy,
    seq::{
        SliceRandom,
        IteratorRandom,
    },
};
use arrayvec::ArrayVec;
use std::collections::HashSet;


pub fn register_terrain_ecs(mesh: &Mesh, world: &mut World) {
    world.register::<TileTopography>();
    world.register::<TileID>();
    world.register::<RiverID>(); // check
    world.register::<RegionID>(); // populate
    world.register::<Region>(); // populate
    world.register::<RegionTopography>();
    world.register::<RegionAdjacency>();
    world.register::<Weather>(); // populate
    world.register::<LandMassID>(); // populate
    world.register::<TileAdjacency>(); // populate

    let num_tiles = mesh.centroids.len();
    let rivers = get_rivers(mesh, RIVER_FLUX_THRESH);
//    let farmdata = get_farm_data(mesh);

    let mut tile2entity = Tile2Entity::default();
    for i in 0..num_tiles {
        let entity = world.create_entity()
                          .with(TileID { id: i })
                          .with(TileTopography::new(mesh, i))
                          .build();

        tile2entity.push(entity);
    }

    world.maintain();

    // add river and farm_data components
    {
        let updater: Read<LazyUpdate> = world.system_data();

        for i in 0..num_tiles {
            let e = tile2entity[i];
            updater.insert(e, TileAdjacency::new(i, &mesh.adj[i], &tile2entity));
        }

        for (i, river) in rivers.iter().enumerate() {
            for &id in river {
                let e = tile2entity[id];
                updater.insert(e, RiverID { id: i });
            }
        }
    }
    world.maintain();
    world.add_resource(tile2entity);
}

pub const RIVER_FLUX_THRESH: f32 = 0.006;
const PEOPLE_PER_KM2: f32 = 10.; // quite low + shouldn't use 'blanket' value


/// return list of n rivers with all m tris in river
pub fn get_rivers(mesh: &Mesh, thresh: f32) -> Vec<Vec<usize>> {
    let Mesh { height: h, flux, adj, .. } = mesh;
    let mut rivers = vec![];
    let mut visited = vec![false; mesh.ids.len()];

    for (i, &flux) in flux.iter().enumerate() {
        if visited[i] { continue; } // skip if already seen
        visited[i] = true;

        // if flow great enough for river
        if flux > thresh && h[i] > 0. {
            let mut river = Vec::with_capacity(10);
            let mut stack = mesh.adj[i]
                .iter()
                .filter(|&&i| !visited[i])
                .collect::<Vec<_>>();
            river.push(i);

            while !(stack.len() < 1) {
                let &j = stack.pop().unwrap();
                visited[j] = true;
                if mesh.flux[j] > thresh && h[j] > 0. {
                    river.push(j);
                    mesh.adj[j].iter()
                               .filter(|&&i| !visited[i])
                               .for_each(|i| stack.push(i));
                }
            }
            rivers.push(river);
        }
    }

    rivers
}


type ConsRegionData<'a> = (ReadStorage<'a, TileTopography>,
                           ReadStorage<'a, TileID>,
                           ReadStorage<'a, TileAdjacency>,
                           ReadStorage<'a, BaseFarmData>,
                           WriteStorage<'a, Region>,
                           WriteStorage<'a, RegionID>,
                           Read<'a, Tile2Entity>,
                           Entities<'a>,
                           Write<'a, LazyUpdate>);

/// Merges Tiles into larger Regions based off similarity
pub fn construct_regions(data: ConsRegionData) {
    construct_regions_inner(data).expect("Error in construct_regions_inner, returned None")
}

pub const MAX_TILES_PER_REGION: u8 = 12;
pub const REGION_BASE_FOOD_MAX: f32 = 30.;// too high, bring down eventually

// TODO refactor and simplify
fn construct_regions_inner((tile_topo, tile_id, tile_adj, farm, region, region_id, t2e, entities, updater): ConsRegionData) -> Option<()> {
    // N: num tiles
    // R: num regions
    // A: tiles per region; N / R
    const MAX_TILES: usize = 100_000;
    let mut rng = SmallRng::from_entropy();


    // Step 0 --
    //   Turn all tiles into 1-tile Regions, RegionAjacency and RegionTopography
    let mut region_map = VecMap::<Region>::with_capacity(10_000);
    let mut reg_topo = VecMap::<RegionTopography>::with_capacity(10_000);
    let mut reg_adj = VecMap::<RegionAdjacency>::with_capacity(10_000);
    let mut reg_agr = VecMap::<RegBaseFarmData>::with_capacity(10_000);

    for (&t_id, t_topo, t_adj, e) in (&tile_id, &tile_topo, &tile_adj, &entities).join() {
        let TileID { id } = t_id;
        region_map.insert(id, Region::from_tile(t_id));
        reg_topo.insert(id, RegionTopography::from_tile(t_topo));
        reg_adj.insert(id, RegionAdjacency::from_tile(t_adj));

        if let Some(farm) = farm.get(e) {
            reg_agr.insert(id, RegBaseFarmData::from_tile(farm));
        }
    }
    let num_tiles = region_map.len();
    let min_regions = ((num_tiles / MAX_TILES_PER_REGION as usize) as f32 * 0.8) as usize;
    debug!("num_tiles: {}, min_regions: {}", num_tiles, min_regions);


    // Step 1 --
    // randomly pick region, pair with most similar neighbor until
    // no regions unpaired (ie make all regions have 2 tiles)

    // ids not paired
    let mut region_pool = region_map.keys().collect::<Vec<_>>();
    let mut done = FnvHashSet::<usize>::default();
    let mut max_not_in_region_set = 0;
    while region_pool.len() > 0 {
        let id = region_pool.swap_remove(rng.gen_range(0, region_pool.len()));

        let mut nbs = reg_adj[id].nbs.iter().filter(|&i| {
            &reg_topo[i].tiles + &reg_topo[id].tiles < 5
                && !done.contains(i)
                && reg_topo[i].height * reg_topo[id].height > 0.
        });

        if let Some(&first) = nbs.next() {
            let mut max_sim = sim_region(id, first, &reg_topo, &reg_agr);
            let mut max = first;

            while let Some(&i) = nbs.next() {
                let sim = sim_region(id, i, &reg_topo, &reg_agr);
                if sim > max_sim {
                    max = i;
                    max_sim = sim;
                }
            }

            if reg_agr.contains_key(id) && reg_agr.contains_key(max) {
                RegBaseFarmData::merge(id, max, &mut reg_agr, &reg_topo);
            }

            Region::merge(id, max, &mut region_map);
            RegionTopography::merge(id, max, &mut reg_topo);
            RegionAdjacency::merge(id, max, &mut reg_adj);

            // region_pool.swap_remove(max);
            if let Some(i) = region_pool.iter().position(|&x| x == max) {
                region_pool.swap_remove(i);
                done.remove(&max);
            } else {
                max_not_in_region_set += 1;
            }
        }
    }
    info!("Num times max not in region set: {}", max_not_in_region_set);

    debug!("num_regions after step 1: {}", region_map.len());

    // Step 2 --
    // randomly pick region and pair with most similar neighbor
    // if new region > food prod thresh ---> take out of merge pool
    // else continue merging until ~ R regions
    let mut region_pool = region_map.keys().collect::<Vec<_>>();
    let mut done = HashSet::new();
    let mut loops = 0;
    while region_map.len() > min_regions && region_pool.len() > 0 {
        let id = region_pool[(rng.gen_range(0, region_pool.len()))];
        loops += 1;

        let mut nbs = reg_adj[id].nbs.iter().filter(|&i| {
            &reg_topo[i].tiles + &reg_topo[id].tiles < MAX_TILES_PER_REGION
                && !done.contains(i)
                && reg_topo[i].height * reg_topo[id].height > 0.
        });

        let mut region_done = false;
        if let Some(&first) = nbs.next() {
            let mut max_sim = sim_region(id, first, &reg_topo, &reg_agr);
            let mut max = first;

            while let Some(&i) = nbs.next() {
                let sim = sim_region(id, i, &reg_topo, &reg_agr);
                if sim > max_sim {
                    max = i;
                    max_sim = sim;
                }
            }

            // only merge if result would be below region cap
            if reg_agr.contains_key(id) & &reg_agr.contains_key(max) {
                RegBaseFarmData::merge(id, max, &mut reg_agr, &reg_topo);
            }

            Region::merge(id, max, &mut region_map);
            RegionTopography::merge(id, max, &mut reg_topo);
            RegionAdjacency::merge(id, max, &mut reg_adj);

            // region_pool.swap_remove(max);
            if let Some(i) = region_pool.iter().position(|&x| x == max) {
                region_pool.swap_remove(i);
            } else {
                error!("Should have removed max");
                dbg!( &region_pool);
                dbg!(max);
            }

            // if base food prod too high, stop merging region
            if let Some(farm) = reg_agr.get(id) {
                if farm.fertility * farm.arable > REGION_BASE_FOOD_MAX {
                    // region_pool.swap_remove(max);
                    region_done = true;
                }
            }

            region_done |= reg_topo[id].tiles > MAX_TILES_PER_REGION;
        } else {
            region_done = true;
        }

        if region_done {
            done.insert(id);
            if let Some(i) = region_pool.iter().position(|&x| x == id) {
                region_pool.swap_remove(i);
            }
        }
    }

    debug!("num loops: {}", loops);
    debug!("num_regions: {}", region_map.len());

    for (i, r) in region_map.drain() {
        let b = updater.create_entity(&entities)
                       .with(r)
                       .with(reg_topo.remove(i).unwrap())
                       .with(reg_adj.remove(i).unwrap());
        if let Some(farm) = reg_agr.remove(i) {
            b.with(farm).build();
        } else {
            b.build();
        }
    }

    Some(())
}

// How similar 2 regions are for paring purposes
//     based off height, fertility, hilliness, flux
pub fn sim_region(a: usize, b: usize, topo: &VecMap<RegionTopography>,
                  agr: &VecMap<RegBaseFarmData>) -> i32
{
    let RegionTopography { height: a_h, hillratio: a_hil, flux: a_flux, position: a_pos, .. } = topo[a];
    let RegionTopography { height: b_h, hillratio: b_hil, flux: b_flux, position: b_pos, .. } = topo[b];
    const RIVER_MULT: f32 = 1. / RIVER_FLUX_THRESH;

    // don't mix sea and land in same region
    if a_h * b_h <= 0. {
        return -1;
    }


    let d_h = (a_h - b_h).abs();
    let d_hil = (a_hil - b_hil).abs();
    let d_flux = ((a_flux - b_flux).abs() * RIVER_MULT)
        .sqrt()
        .min(1.); // b/c not normalized

    // ratio  dist / sqrt(area)
    // idea: long regions avoided
    // TODO figure out coefficients
//     let d_pos = a_pos.distance(b_pos) / (topo[a].area + topo[b].area).sqrt();

    match (agr.get(a), agr.get(b)) {
        (Some(a_agr), Some(b_agr)) => {
            let d_fert = (a_agr.fertility - b_agr.fertility).abs();
            //let sim = 0.25 * d_h + 0.25 * d_hil + 0.05 * d_flux + 0.3 * d_fert + 0.15 * d_pos; // 0 - 1
            let sim = 0.3 * d_h + 0.3 * d_hil + 0.1 * d_flux + 0.3 * d_fert; // 0 - 1
            (sim * 10000.) as i32
        }
        _ => {
            let sim = 0.4 * d_h + 0.4 * d_hil + 0.3 * d_flux; // 0 - 1
            (sim * 10000.) as i32
        }
    }
}

