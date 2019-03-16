use super::*;
use specs::prelude::*;
use specs_derive;
use crate::{
    terrain::components::RegionID,
};


pub fn register_pop_ecs(world: &mut World) {
    world.register::<RegionPop>();
    let (regions, entities): (ReadStorage<RegionID>, Entities) = world.system_data();
    let updater: Read<LazyUpdate> = world.system_data();

    // temporary
    let dist = [1./14.;14]; // uniform
    for (region, e) in (&regions, &entities).join() {
        let rp = RegionPop::new(&dist, 1000);
        updater.insert(e, rp);
    }

}
