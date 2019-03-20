use super::*;
use crate::networking::*;
use std::collections::HashSet;

pub struct Terr;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum TerrData {
    Region,
    Weather,
    RiverID,
    LandMassID,
    TileTopography,
}

pub struct TerrSender {
    pub out: ClientSender
}

impl<'a> System<'a> for TerrSender {
    type SystemData = (Read<'a, HashSet<TerrData>>,
                       ReadStorage<'a, Region>,
                       ReadStorage<'a, RegionID>,
                       ReadStorage<'a, Weather>,
                       ReadStorage<'a, RiverID>,
                       ReadStorage<'a, LandMassID>,
                       ReadStorage<'a, TileTopography>,
                       ReadStorage<'a, TileID>);

    fn run(&mut self, (subs, region, region_id, weather, river_id, land_mass_id, tile_topo, tile_id): Self::SystemData) {
        for &sub in subs.iter() {
            let section = Sections::Terr;


            match sub {
                TerrData::Region => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "Region",
                        data: (&region).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
                TerrData::Weather => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "Weather",
                        data: (&weather).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
                TerrData::RiverID => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "RiverID",
                        data: (&tile_id, &river_id).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
                TerrData::LandMassID => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "LandMassID",
                        data: (&region_id, &land_mass_id).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
                TerrData::TileTopography => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "TileTopography",
                        data: (&tile_id, &tile_topo).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
            }
        }
    }
}
