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
extern crate population_simulation;
#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;
//extern crate config;


use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Instant, Duration};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use std::result::Result;

use failure::Error;
//use fnv::{FnvHashMap, FnvHashSet };
use std::collections::HashMap;
use ws::Sender as WsSender;
use serde::Serialize;
use specs::prelude::*;
use specs::Join;

use population_simulation::networking::*;
use population_simulation::components::*;
use population_simulation::systems::*;
use specs::world::Generation;
use specs::ReadStorage;
use specs::shred::DynamicSystemData;
use anymap::AnyMap;

use population_simulation::*;
use population_simulation::networking::subscription_system::SubscriptionManager;
use population_simulation::terrain::{
    map_file_loader::{move_map_files, load_map_file},
    mesh::Mesh,
    components::*,register_map_ecs,
};


fn main() -> Result<(), Error> {
    trace!("Hello, world!");
    let wait_client = create_ws_server()?;

    let _guard = slog_scope::set_global_logger(setup_logger());

    info!("testing logger");

    move_map_files()?;
    let (mesh, mesh_json) = load_map_file(None)?;
    debug!("mesh from file, number of tiles: {}", mesh.ids.len());

    let mut world = World::new();

    register_map_ecs(&mesh, &mut world);
    world.add_resource(mesh);
    world.add_resource(Some(mesh_json));
    world.add_resource(DeltaTime(0.051));


    // blocks until connection established to a client
    let WsReturn { server_thread, out, sub_recv, rec_type_recv } = wait_client()?;
    trace!("after waiting for connect");
    world.add_resource(out.clone());
    send_init_data(&mut world)?;


    // send fertility data to display on client
    // world.exec(send_displayable_for_tag::<River>);
//    world.exec(|x|
//        send_displayable_for_data::<FarmData>(x, |&FarmData { fertility, .. }| fertility)
//    );

    let mut dispatcher = DispatcherBuilder::new().build();
    dispatcher.dispatch(&mut world.res);
    world.maintain();

    let subsciption_manager = SubscriptionManager {
        recv: sub_recv,
        out: out.clone(),
    };

    game_loop(world, dispatcher, subsciption_manager);

    let _ = server_thread.join();

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
