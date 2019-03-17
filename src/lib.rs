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
#[macro_use]
extern crate lazy_static;
//extern crate config;
#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
extern crate slog;
extern crate slog_async;
extern crate slog_term;
#[macro_use]
extern crate slog_scope;
//#[macro_use]
//extern crate log;
//extern crate env_logger;
extern crate chrono;
extern crate ord_subset;

pub mod networking;
pub mod terrain;
pub mod agriculture;
pub mod pop;
pub mod misc;


use std::{
    sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn},
    thread,
    thread::JoinHandle,
    time::{Instant, Duration},
    fmt::Debug,
    sync::atomic::{AtomicUsize, Ordering},
    result::Result,
    collections::HashMap,
};

use failure::Error;
//use fnv::{FnvHashMap, FnvHashSet };
use ws::Sender as WsSender;
use serde::Serialize;
use anymap::AnyMap;

use specs::{
    world::Generation,
    ReadStorage,
    Join,
    prelude::*,
};

use crate::{
    networking::{
        *,
        sub_req::SubReqDispatcher,
    },
    terrain::{
        map_file_loader::{move_map_files, load_map_file},
        mesh::Mesh,
        components::*,
        init::register_terrain_ecs,
    },
    agriculture::{
        sub_req::*
    },
    misc::{
        *,
        components::DeltaTime,
        core_loop::game_loop,
    },
};

//lazy_static! {
//    pub static ref SETTINGS : RwLock<config::Config> = RwLock::new(config::Config::default());
//}


pub fn setup() -> Result<(), Error> {
    let wait_client = create_ws_server()?;

    let _guard = slog_scope::set_global_logger(setup_slog_logger());

    let mut world = setup_world()?;

    let foo = Some("hi");
    // blocks until connection established to a client
    let WsReturn { server_thread, out, sub_recv, sub_req_recv, rec_type_recv } = wait_client()?;
    trace!("after waiting for connect");
    world.add_resource(out.clone());
    send_init_data(&mut world)?;


    // send fertility data to display on client
    // world.exec(send_displayable_for_tag::<River>);
    /*
    world.exec(|x|
        send_displayable_for_data::<TileTopography>(x, |&TileTopography { area, .. }| area)
    );
    */

    let mut dispatcher = DispatcherBuilder::new()
        .with(misc::time::UpdateDate, "UpdateDate", &[])
        .with(SubReqDispatcher { recv: sub_req_recv }, "SubReqDispatcher", &[])
        .with(AgrSubReq { out: out.clone() }, "AgrSubReq", &[])
        .build();
    dispatcher.setup(&mut world.res);

    dispatcher.dispatch(&mut world.res);
    world.maintain();

    game_loop(world, dispatcher);

    let _ = server_thread.join();

    Ok(())
}


fn setup_world() -> Result<World, Error> {
    move_map_files()?;
    let (mesh, mesh_json) = load_map_file(None)?;
    debug!("mesh from file, number of tiles: {}", mesh.ids.len());

    let mut world = World::new();

    register_terrain_ecs(&mesh, &mut world);
    world.add_resource(mesh);
    world.add_resource(Some(mesh_json));
    world.add_resource(DeltaTime(0.051));
    time::init_date(&mut world);

    agriculture::init::register_agr_ecs(&mut world);
    pop::init::register_pop_ecs(&mut world);

    Ok(world)
}

const TIMESTAMP_FORMAT: &'static str = "%m-%d %H:%M:%S";

/* Log levels
critical
error
warning
info
debug
trace
*/
fn setup_slog_logger() -> slog::Logger {
    use slog::*;

    let decorator = slog_term::TermDecorator::new()
        .force_color()
        .build();
    let drain = slog_async::Async::new(
        slog_term::CompactFormat::new(decorator)
            .use_custom_timestamp(|io: &mut std::io::Write| {
                write!(io, "{}", chrono::Local::now().format(TIMESTAMP_FORMAT))
            })
            .build().fuse()
    )
        .build() .fuse();

    slog::Logger::root(drain, o![])
}

/*
fn setup_env_logger() {
    let env = Env::default()
        .filter_or("MY_LOG_LEVEL", "trace")
        .write_style_or("MY_LOG_STYLE", "always");

    env_logger::init_from_env(env);
}*/
