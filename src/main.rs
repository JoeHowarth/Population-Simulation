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


mod ws_server;
#[macro_use]
mod type_string;
mod systems;
mod components;

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

use ws_server::*;
use type_string::*;
use components::*;
use systems::*;
use specs::world::Generation;
use components::tiles::Tile2Entity;
use specs::ReadStorage;
use specs::shred::DynamicSystemData;
use anymap::AnyMap;


fn main() -> Result<(), Error> {
    println!("Hello, world!");

    // Server thread
    let (server, out) = create_ws_server()?;

    let mut world = World::new();
    world.register::<Position>();
    world.register::<Velocity>();
    world.add_resource(DeltaTime(0.051));
//    world.add_resource(Tile2Entity::new());
    world.add_resource(out.clone());

//    world.register::<tiles::TileAdjacency>();
//    world.register::<tiles::TileId>();
//    world.register::<tiles::TileTopography>();



    world.create_entity().with(Position { x: 4.0, y: 7.0 }).build();
    let d = world.create_entity()
        .with(Position { x: 30.0, y: 8.0 })
        .with(Velocity { x: 10.72, y: 8.3 })
        .build();
    world.create_entity()
        .with(Position { x: 30.0, y: 8.0 })
        .with(Velocity { x: 10.72, y: 18.3 })
        .build();
    world.create_entity()
        .with(Position { x: 300.0, y: 8.0 })
        .with(Velocity { x: -13.72, y: 3.3 })
        .build();



    let mut dispatcher = DispatcherBuilder::new()
        .with(PrintPosSys, "print_pos", &[])
        .with(UpdatePos, "update_pos", &["print_pos"])
        .with(PrintPosSys, "print_updated_pos", &["update_pos"])
        .with(SendSys::<Position>::new("set_pos"), "send_pos", &["update_pos"])
        .build();

    dispatcher.dispatch(&mut world.res);
    world.maintain();


    game_loop(world, dispatcher);


    let _ = server.join();

    Ok(())
}


fn game_loop(mut world: World, mut dispatcher: Dispatcher) {
    let frame_sleep = Duration::from_millis(1500);
    let mut last = Instant::now();
    loop {
        let dt = Instant::now() - last;
        last = Instant::now();
        {
            let mut delta = world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_float_secs() as f32);
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();



        thread::sleep(frame_sleep);
    }
}


#[derive(Serialize, Deserialize, Debug, PartialOrd, PartialEq)]
struct ExData {
    number: f32,
    str_arr: Vec<String>,
    message: String,
}


