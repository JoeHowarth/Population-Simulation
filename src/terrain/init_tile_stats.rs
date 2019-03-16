use crate::{
    terrain::{
        components::*,
        mesh::{Mesh, MeshJson}
    }
};
use fnv::{FnvHashMap, FnvHashSet};
use specs::prelude::*;
use ord_subset::*;
use vec_map::VecMap;
use failure::Error;
use ord_subset::*;
use std::{
    collections::BinaryHeap,
    cmp::Ordering,
    iter::FromIterator,
    collections::VecDeque
};

use crate::normalize::*;

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

