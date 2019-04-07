pub mod agr_init;
pub mod terrain_init;
pub mod pop_init;
pub mod map_file_loader;

pub use self::{
    agr_init::*,
    terrain_init::*,
    pop_init::*,
};

use self::map_file_loader::*;

use crate::{
    misc::components::DeltaTime,
    prelude::*
};


pub fn setup_world() -> Result<World, Error> {
    move_map_files()?;
    let (mesh, mesh_json) = load_map_file(None)?;
    debug!("mesh from file, number of tiles: {}", mesh.ids.len());

    let mut world = World::new();

    terrain_init::register_terrain_ecs(&mesh, &mut world);
    world.add_resource(mesh);
    world.add_resource(Some(mesh_json));
    world.add_resource(DeltaTime(0.051));
    time::init_date(&mut world);

    register_agr_ecs(&mut world);
    construct_regions(world.system_data());
    world.maintain();

    register_pop_ecs(&mut world);
    world.maintain();

    init_farm_data(world.system_data());

    world.maintain();
    Ok(world)
}
