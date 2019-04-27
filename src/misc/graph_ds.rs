use crate::prelude::*;
use specs::join::JoinIter;
use serde_json::value::Value::Array;

pub trait AdjList {
    fn get_nbs<'a>(&self, e: Entity) -> ArrayVec<[usize; 256]>;
    //fn nbs_ent_u(&self, id: usize, map: &[Entity]) -> Vec<Entity>;
    //fn nbs_ent(&self, e: &Entity) -> ArrayVec<[Entity; 256]>;
}

pub struct Graph<'a, A: AdjList> {
    adj: A,
    id_map: Read<'a, VecMap<Entity>>,
    ents: Entities<'a>,
}

impl<'a, A: AdjList> Graph<'a, A> {
    pub fn new(adj: A, id_map: Read<'a, VecMap<Entity>>, ents: Entities<'a>) -> Self {
        Graph { adj, id_map, ents }
    }

    pub fn nbs_iter<T: Join>(&mut self, res: &mut JoinIter<T>, id: usize) -> impl Iterator<Item=<T as Join>::Type> {
        dbg!(id);
        let e: Entity = self.id_map[id];

        let nbs = self.adj.get_nbs(e);

        let v: Vec<_> = nbs.iter()
                           .map(|&id| self.id_map.get(id).expect("boom2"))
                           .filter_map(|&e| res.get(e, &self.ents)).collect();
        v.into_iter()
    }
}

impl<'a> AdjList for ReadStorage<'a, RegionAdjacency> {
    fn get_nbs(&self, e: Entity) -> ArrayVec<[usize; 256]> {
        let ra = self.get(e).expect("boom");
        ArrayVec::from_iter(ra.nbs.iter().map(|&i| i))
    }
}

pub fn test_graph_ds(world: &mut World) {
    println!("in test graph ds");
    let (r, rp, rfd): (ReadStorage<Region>, ReadStorage<RegionPop>, ReadStorage<RegBaseFarmData>) = world.system_data();
    let adj: ReadStorage<RegionAdjacency> = world.system_data();
    let (r2e, ents): (Read<Region2Entity>, Entities) = world.system_data();

    let mut graph = Graph::new(adj, r2e, ents);
    let mut res = (&rp, &rfd).join();


    for Region { id, .. } in r.join() {
        dbg!(id);
        graph.nbs_iter(&mut res, *id).for_each(|(rp, rfd)| { dbg!(rfd); });
    }
}
