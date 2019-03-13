pub mod mesh;
pub mod map_file_loader;

use crate::components::{tiles::*};
use crate::map::mesh::{Mesh, MeshJson};
use fnv::FnvHashMap;
use specs::prelude::*;
use fnv::FnvHashSet;
use ord_subset::*;
use vec_map::VecMap;
use std::collections::BinaryHeap;
use failure::Error;
use std::{
    iter::FromIterator,
    collections::VecDeque
};
use crate::map::mesh::MeshWrapper;

pub const RIVER_FLUX_THRESH: f32 = 0.006;


pub fn register_map_ecs(mesh: &Mesh, world: &mut World) {
    world.register::<TileTopography>();
    world.register::<Population>();
    world.register::<FarmData>();
    world.register::<CityData>();
    world.register::<TileID>();
    world.register::<River>();

    let rivers = get_rivers(mesh, RIVER_FLUX_THRESH);
    let farmdata = get_farm_data(mesh);

    let mut tile2entity = Tile2Entity::default();
    for i in 0..(mesh.centroids.len()) {
        let builder = world.create_entity()
            .with(TileID { id: i })
            .with(TileTopography {
                height: mesh.height[i],
                position: mesh.centroids[i],
                flux: mesh.flux[i],
                slope: mesh.slope[i],
            });

        let entity = if let Some(fd) = farmdata[i].clone() {
            builder.with(fd)
        } else {
            builder
        }.build();

        tile2entity.push(entity);
    }

    world.maintain();

    // add river components
    {
        let updater: Read<LazyUpdate> = world.system_data();
        for (i, river) in rivers.iter().enumerate() {
            for &id in river {
                let &e = tile2entity.get(id).expect("entity from tileID not found...");
                updater.insert(e, River { id: i });
            }
        }
    }
    world.maintain();
    world.add_resource(tile2entity);
}

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

const PEOPLE_PER_KM2: f32 = 10.;

pub fn get_farm_data(mesh: &Mesh) -> Vec<Option<FarmData>> {
    let Mesh { height: h, flux, adj, slope, .. } = mesh;
    // lower threshold than 'real' rivers
    let rivers = get_rivers(mesh, RIVER_FLUX_THRESH * 0.5);

    use std::cmp::Ordering;
    #[derive(PartialOrd, PartialEq, Copy, Clone, Debug)]
    struct Node(f32, f32, usize, usize); // flux, strength in [1,4], tileID {id <--- } curr, last
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

    let mut heap = BinaryHeap::from_iter(rivers.iter().flatten()
        .map(|&k| {
            let strength = (flux[k] / (RIVER_FLUX_THRESH * 3.0)).min(1.0).sqrt();
            let shifted = strength * 0.2 + 0.80;
            Node(flux[k].min(RIVER_FLUX_THRESH), shifted, k, k)
        }));

    const BASE_CARRIED: f32 = 1.0;
    let mut fert_flux = vec![0.; h.len()];

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
        if fert_flux[i] < new_flux {
            fert_flux[i] = new_flux;
            for &j in &adj[i] {
                if h[j] > 0. {
                    heap.push(Node(fert_flux[i], strength, j, i));
                }
            }
        }
    }


    let mut fertility: Vec<_> = (0..h.len()).map(|i| {
        // smoothed slope
        let slope_coef = {
            let mean_slope: f32 = [i].iter().map(|&j| slope[j]).sum::<f32>() / adj[i].len() as f32;

            let relaxed_slope: f32 = 0.7 * mean_slope + 0.3 * slope[i];
            if relaxed_slope > 1.0 {
                1. / (relaxed_slope + 0.2).sqrt() + 0.18
            } else {
                let x = relaxed_slope;
                -(x * x) / 5. + 1.2 // -x^2/b + 1 + 1/b
            }
        };

//        let fertility = (slope_coef * fert_flux[i]).sqrt();
        let fertility = fert_flux[i];
        fertility
    }).collect();

    fertility = normalize_vec(fertility);

    fertility.iter().enumerate().map(|(i, &fertility)| {
        if h[i] < 0. {
            return None;
        }

        let arable_cap = PEOPLE_PER_KM2 * mesh.area[i];
//        debug!("slope coef {}, max_flux {:?}, fert {}", slope_coef, max_fluxs, fertility);

        Some(FarmData { fertility, arable_cap })
    }).collect()

    // do 'flood fill' out from river tiles
}


pub fn normalize<'a, I: 'static>(arr: I) -> Box<dyn Iterator<Item=f32>>
    where I: Iterator<Item=&'a f32> + Clone,
{
    use std::f32;
    let (min, max): (f32, f32) = arr.clone()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

    debug!("normalize, max {}, min {}", &max, &min);

    Box::new(arr.map(move |&x| (x.clone() - min) / (max - min)))
}

pub fn normalize_vec<'a>(mut arr: Vec<f32>) -> Vec<f32>
{
    use std::f32;
    let (min, max) = arr.iter()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

    debug!("normalize, max {}, min {}", &max, &min);

    arr.iter_mut().for_each(|x| *x = (*x - min) / (max - min));
    arr
}

pub fn normalize_mut<'a, I>(arr: I)
    where I: Iterator<Item=&'a mut f32> + Clone,
{
    use std::f32;
    let (min, max) = arr.clone()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &mut v| {
            (min.min(v), max.max(v))
        });

    debug!("normalize, max {}, min {}", &max, &min);

    arr.for_each(|x| *x = (*x - min) / (max - min));
}

pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn float_max<'a, I>(arr: I) -> f32
    where I: Iterator<Item=&'a f32>,
{
    use std::f32;
    arr.fold(-f32::INFINITY, |max, &v| {
        max.max(v)
    })
}

