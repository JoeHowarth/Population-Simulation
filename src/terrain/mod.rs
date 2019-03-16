pub mod mesh;
pub mod components;
pub mod map_file_loader;
pub mod init_tile_stats;

use self::{
    components::*,
    init_tile_stats::{get_rivers},
    mesh::{Mesh, MeshJson},
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


pub fn register_map_ecs(mesh: &Mesh, world: &mut World) {
    world.register::<TileTopography>();
    world.register::<TileID>();
    world.register::<RiverID>();

    let rivers = get_rivers(mesh, RIVER_FLUX_THRESH);
//    let farmdata = get_farm_data(mesh);

    let mut tile2entity = Tile2Entity::default();
    for i in 0..(mesh.centroids.len()) {
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


        for (i, river) in rivers.iter().enumerate() {
            for &id in river {
                let &e = tile2entity.get(id).expect("entity from tileID not found...");
                updater.insert(e, RiverID { id: i });
            }
        }
    }
    world.maintain();
    world.add_resource(tile2entity);
}


