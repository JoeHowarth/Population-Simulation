pub mod mesh;
pub mod components;
pub mod map_file_loader;
pub mod init;
pub mod sub_req;

pub use self::{
    components::*,
    init::{register_terrain_ecs, get_rivers},
    mesh::{Mesh, MeshJson},
    sub_req::*,
};
use fnv::{FnvHashMap, FnvHashSet};
use specs::prelude::*;
use ord_subset::*;
use vec_map::VecMap;
use failure::Error;
use std::{
    collections::BinaryHeap,
    iter::FromIterator,
    collections::VecDeque,
};

pub const RIVER_FLUX_THRESH: f32 = 0.006;




