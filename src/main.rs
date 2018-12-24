#![feature(duration_float)]
#![allow(dead_code, unused_variables, unused_imports)]
#![feature(core_intrinsics)]


#[macro_use]
extern crate specs_derive;
extern crate specs;
extern crate ws;
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
extern crate failure;
extern crate fnv;
extern crate anymap;
extern crate ws_rs_ex;


use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Instant, Duration};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};

use failure::Error;
//use fnv::{FnvHashMap, FnvHashSet };
use std::collections::HashMap;
use ws::Sender as WsSender;
use serde::Serialize;
use specs::prelude::*;
use specs::Join;

use ws_rs_ex::ws_server::*;
use ws_rs_ex::type_string::*;
use ws_rs_ex::components::*;
use ws_rs_ex::systems::*;
use specs::world::Generation;
use specs::ReadStorage;
use specs::shred::DynamicSystemData;
use anymap::AnyMap;

use ws_rs_ex::*;
use ws_rs_ex::systems::subscription_system::SubscriptionManager;
use ws_rs_ex::map::*;
use ws_rs_ex::components::tiles::{
    FarmData,
    Tile2Entity,
    TileID,
};
use ws_rs_ex::map::mesh::Mesh;


fn main() -> Result<(), Error> {
    println!("Hello, world!");

    // Server thread
    let WsReturn { server, out, mesh_recv, sub_recv } = create_ws_server()?;

    let mut world = World::new();
    world.add_resource(DeltaTime(0.051));
    world.add_resource(out.clone());

    let mesh = mesh_recv.recv()?;
    register_map_ecs(&mesh, &mut world);
    world.add_resource(mesh);


    // send fertility data to display on client
    {
        let (farm_data, ids, mesh): (ReadStorage<FarmData>, ReadStorage<TileID>, ReadExpect<Mesh>) = world.system_data();
        let mut fert: Vec<f32> = vec![-10.; mesh.ids.len()];
        for (fert_opt, &TileID { id }) in (farm_data.maybe(), &ids).join() {
            let val = if let Some(FarmData { fertility, .. }) = fert_opt {
                *fertility
            } else {
                0.0
            };
            fert[id] = val;
        }

//        let fert = normalize_slice(&fert);

        println!("fertility {:?}", &fert);

        out.send(&MutationMsg {
            mutation: "setMapData".into(),
            inner: fert,
        })
    }

    let mut dispatcher = DispatcherBuilder::new()
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();

    let subsciption_manager = SubscriptionManager {
        recv: sub_recv,
        out: out.clone(),
    };

    game_loop(world, dispatcher, subsciption_manager);

    let _ = server.join();

    Ok(())
}


fn game_loop(mut world: World, mut dispatcher: Dispatcher, subscription_manager: SubscriptionManager) {
    let frame_sleep = Duration::from_millis(3500);
    let mut last = Instant::now();
    let mut total_frames = 0;
    loop {
        let dt = Instant::now() - last;
        last = Instant::now();
        {
            let mut delta = world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_float_secs() as f32);
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();

        subscription_manager.dispatch(&mut world);


        if total_frames > 5000 { break; }
        total_frames += 1;
        thread::sleep(frame_sleep);
    }
}


#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
struct ExData {
    number: f32,
    str_arr: Vec<String>,
    message: String,
}
