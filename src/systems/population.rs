use specs::prelude::*;
use crate::components::{
    tiles::{
        TileTopography,
        Population,
        FarmData,
        CityData,
        TileAdjacency,
        Tile2Entity}
};
use crate::map::mesh::Mesh;
use specs::Join;

pub struct GrowPopulation;

impl<'a> System<'a> for GrowPopulation {
    type SystemData = (WriteStorage<'a, Population>,
                       ReadStorage<'a, FarmData>,
                       ReadStorage<'a, CityData>,
                       ReadStorage<'a, TileTopography>,
                       ReadExpect<'a, Tile2Entity>,
                       ReadExpect<'a, Mesh>);

    fn run(&mut self, (mut pop, farm, city, topo, t2e, mesh): Self::SystemData) {

        for (pop, farm, city, topo) in (&mut pop, &farm, &city, &topo).join() {

        }
    }

    fn setup(&mut self, res: &mut Resources) {
        Self::SystemData::setup(res);


    }
}


