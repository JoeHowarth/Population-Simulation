pub mod mesh;

use crate::components::{tiles::*};
use self::mesh::Mesh;
use fnv::FnvHashMap;
use specs::prelude::*;
use fnv::FnvHashSet;
use std::collections::vec_deque::VecDeque;
use std::iter::FromIterator;

pub fn register_map_ecs(mesh: &Mesh, world: &mut World) {
    world.register::<TileTopography>();
    world.register::<Population>();
    world.register::<FarmData>();
    world.register::<CityData>();
    world.register::<TileID>();

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
    world.add_resource(tile2entity);
}

/// return list of n rivers with all m tris in river
pub fn get_rivers(mesh: &Mesh, thresh: f32) -> Vec<Vec<usize>> {
    let mut rivers = vec![];
    let mut visited = vec![false; mesh.ids.len()];

    for (i, &flux) in mesh.flux.iter().enumerate() {
        if visited[i] { continue; } // skip if already seen
        visited[i] = true;

        // if flow great enough for river
        if flux > thresh {
            let mut river = Vec::with_capacity(10);
            let mut stack = mesh.adj[i]
                .iter()
                .filter(|&&i| !visited[i])
                .collect::<Vec<_>>();
            river.push(i);

            while !(stack.len() < 1) {
                let &j = stack.pop().unwrap();
                visited[j] = true;
                if mesh.flux[j] > thresh {
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

const PEOPLE_PER_KM2: f32 = 50.;

pub fn get_farm_data(mesh: &Mesh) -> Vec<Option<FarmData>> {
    let Mesh { height: h, flux, adj, .. } = mesh;

    // loop over all tiles
    let mut flux_coefs = (0..mesh.centroids.len()).map(|i| {
        // only farmable if land
        if h[i] <= 0. {
            return None;
        }

        // find highest flux within 2 tiles
        let mut max_fluxs = vec![0.; 6];
        let mut slope_acc = 0.;

        // bfs over flux
        let mut queue = VecDeque::from_iter(adj[i]
            .iter()
            .map(|&j| (j, 0)));
        let mut visited = FnvHashSet::from_iter(adj[i].iter());

        let mut curr_depth = 0;
        while let Some((j, depth)) = queue.pop_front() {
            if depth > curr_depth {
                curr_depth = depth;
                max_fluxs[depth] = (max_fluxs[depth - 1] as f32).max(max_fluxs[depth] as f32);
            }
            if flux[j] > max_fluxs[depth] && h[j] >= 0. {
                max_fluxs[depth] = flux[j];
            }
            if depth > 2 { continue; }
            for k in adj[j].iter() {
                if !visited.contains(k) {
                    queue.push_back((*k, depth + 1));
                    visited.insert(k);
                }
            }
        }

        let flux_coef = 0.3 * max_fluxs[0]
            + 0.3 * max_fluxs[1]
            + 0.4 * max_fluxs[2];
//            + 0.3 * max_fluxs[3];
//            + 0.1 * max_fluxs[4];


        Some(flux_coef);
    }).collect();

    for (i, flux_coef) in flux_coefs.iter().enumerate() {
        // relax flux_coef
        let mean_flux = adj[i].iter().map(|&j| flux_coefs[j]).sum() / adj[i].len() as f32;
        let max_flux = adj[i].iter().float_max();

        let flux_coef = flux_coef * 0.2 + mean_flux * 0.4 + max_flux * 0.4;

        let mut slope_acc = 0.;
        for &j in &adj[i] {
            slope_acc += mesh.slope[j]
        };

        // smoothed slope
        let relaxed_slope = 0.7 * slope_acc / adj[i].len() as f32 + 0.3 * mesh.slope[i];
        let slope_coef = if relaxed_slope > 1.0 {
            1. / (relaxed_slope + 0.2).sqrt() + 0.18
        } else {
            let x = relaxed_slope;
            -(x * x) / 5. + 1.2 // -x^2/b + 1 + 1/b
        };

        let fertility = (slope_coef * flux_coef).sqrt(); // TODO: ACTUALLY WORK
        let arable_cap = PEOPLE_PER_KM2 * mesh.area[i];
//        println!("slope coef {}, max_flux {:?}, fert {}", slope_coef, max_fluxs, fertility);

        Some(FarmData { fertility, arable_cap })
    }.collect()
}


pub fn normalize<'a, I>(arr: I) -> I
    where I: Iterator<Item=&'a f32> + Clone,
{
    use std::f32;
    let (min, max) = arr.clone()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

    println!("normalize, max {}, min {}", &max, &min);

    arr.map(|x| (x - min) / (max - min))
}

pub fn normalize_slice<'a>(arr: &[f32]) -> Vec<f32>
{
    use std::f32;
    let (min, max) = arr.iter()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &v| {
            (min.min(v), max.max(v))
        });

    println!("normalize, max {}, min {}", &max, &min);

    arr.iter().map(|x| (x - min) / (max - min))
        .collect()
}

pub fn normalize_mut<'a, I>(arr: I)
    where I: Iterator<Item=&'a mut f32> + Clone,
{
    use std::f32;
    let (min, max) = arr.clone()
        .fold((f32::INFINITY, -f32::INFINITY), |(min, max), &mut v| {
            (min.min(v), max.max(v))
        });

    println!("normalize, max {}, min {}", &max, &min);

    arr.for_each(|x| *x = (*x - min) / (max - min));
}

pub fn average(numbers: &[i32]) -> f32 {
    numbers.iter().sum::<i32>() as f32 / numbers.len() as f32
}

pub fn float_max<'a, I>(arr: I) -> f32
    where I: Iterator<Item=&'a f32>,
{
    use std::f32;
    arr.fold(-f32::INFINITY, |(max), &v| {
        max.max(v)
    })
}
