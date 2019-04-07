use super::*;
use crate::{
    prelude::*,
};
use rand::{
    distributions::WeightedIndex
};

//use std::collections::binary_heap::BinaryHeap;


pub fn register_pop_ecs(world: &mut World) {
    {
        world.register::<RegionPop>();
        world.register::<PopEst>();
        init_pop_est(world);
        let (regions, entities): (ReadStorage<Region>, Entities) = world.system_data();
        let updater: Read<LazyUpdate> = world.system_data();


        // temporary
        let mut dist = [1. / 16.; 17]; // uniform
        dist[16] = 0.;
        for (region, e) in (&regions, &entities).join() {
            let rp = RegionPop::new(&dist, 10_000);
            //dbg!(&rp);
            updater.insert(e, rp);
        }
    }
    world.maintain();
}

pub fn init_pop_est(world: &mut World) {
    let (ids, topo, base, adj, mut pop, t2e, entities): (ReadStorage<TileID>, ReadStorage<TileTopography>, ReadStorage<BaseFarmData>, ReadStorage<TileAdjacency>, WriteStorage<PopEst>, ReadExpect<Tile2Entity>, Entities) = world.system_data();

    let v: VecMap<_> = (&ids, &topo, &base, &adj).join()
        .map(|(id, t, b, a)| (id.id, (id, t, b, a)))
        .collect();
    let goodness: Vec<i32> = v.values().map(|(_, t, b, _)| {
        //b.fertility + t.hillratio / 2. + (t.flux * RIVER_FLUX_THRESH).sqrt().sqrt() / 2.
        ((b.fertility + t.hillratio / 2.) * 1000.) as i32
    }).collect();
    let keys: Vec<_> = v.keys().collect();

    let dist = WeightedIndex::new(goodness.iter()).unwrap();

    let mut rng = SmallRng::from_entropy();

    let mut arr = ArrayVec::<[usize; 30]>::new();
    for i in 0..30 {
        arr.push(dist.sample(&mut rng));
    }

    // flood fill out from centers
    let mut heap = BinaryHeap::from_iter(arr.iter().map(|&i| {
        let g = goodness[i];
        WeightedNode { weight: ((g as f32).sqrt() as i32) * 10, inner: &v[keys[i]] }
    }));
    let mut seen = VecMap::with_capacity(goodness.len());

    let mut pop_est = VecMap::with_capacity(v.len());
    while let Some(WeightedNode { weight, inner: (id, t, b, a) }) = heap.pop() {
        let coef = ((t.hillratio / 2. + b.fertility) as f32 / 2.).sqrt();
        let new_weight = (weight as f32 * coef) as i32;
        pop_est.insert(id.id, new_weight);

        seen.insert(id.id, ());
        for &n in a.nbs.iter().filter(|&&n| !seen.contains_key(n)) {
            let e = t2e.get(id.id).unwrap();

            if let Some(x) = v.get(n) {
                heap.push(WeightedNode {
                    weight: new_weight,
                    inner: x,
                });
            }
        }
    }


    dbg!(&pop_est);
    for (k, p) in pop_est {
        pop.insert(t2e[k], PopEst(p as usize)).unwrap();
    }
}


