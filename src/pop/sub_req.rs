use super::*;
use specs::prelude::*;
use crate::{
    networking::*,
    terrain::components::*,
};
use std::collections::HashSet;

pub struct Pop;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum PopData {
    RegionPop,
}

pub struct PopSender {
    pub out: ClientSender
}

impl<'a> System<'a> for PopSender {
    type SystemData = (Read<'a, HashSet<PopData>>,
                       ReadStorage<'a, RegionPop>,
                       ReadStorage<'a, RegionID>);

    fn run(&mut self, (subs, region_pop, region_id): Self::SystemData) {
        for &sub in subs.iter() {
            let section = Sections::Pop;

            match sub {
                PopData::RegionPop => {
                    self.out.sub_push(SubPush {
                        section,
                        component: "RegionPop",
                        data: (&region_id, &region_pop).join().collect::<Vec<_>>(),
                        keys: None,
                    });
                }
            }
        }
    }
}
