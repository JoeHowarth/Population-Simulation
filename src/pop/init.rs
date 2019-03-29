use super::*;
use specs::prelude::*;
use specs_derive;
use crate::{
    terrain::components::Region,
};


pub fn register_pop_ecs(world: &mut World) {
    {
        world.register::<RegionPop>();
        let (regions, entities): (ReadStorage<Region>, Entities) = world.system_data();
        let updater: Read<LazyUpdate> = world.system_data();


        // temporary
        let mut dist = [1. / 16.; 17]; // uniform
        dist[16] = 0.;
        for (region, e) in (&regions, &entities).join() {
            let rp = RegionPop::new(&dist, 10_000);
            dbg!(&rp);
            updater.insert(e, rp);
        }
    }
    world.maintain();
}
