#![feature(duration_float)]
#![allow(dead_code, unused_variables, unused_imports)]
#![feature(core_intrinsics)]


//#[macro_use]
//extern crate specs_derive;
//extern crate specs;
//extern crate ws;
//#[macro_use]
//extern crate serde_derive;
//extern crate serde;
//extern crate serde_json;
extern crate failure;
//extern crate fnv;
//extern crate anymap;
extern crate population_simulation;
//#[macro_use(o, slog_log, slog_trace, slog_debug, slog_info, slog_warn, slog_error)]
//extern crate slog;
//extern crate slog_async;
//extern crate slog_term;
//#[macro_use]
//extern crate slog_scope;
//extern crate config;


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
use population_simulation::{
    *,
    networking::*,
    terrain::{
        map_file_loader::{move_map_files, load_map_file},
        mesh::Mesh,
        components::*,
        init::register_terrain_ecs,
    },
    agriculture::components::AgrData,
};
use population_simulation::networking::subscription_system::SubscriptionManager;


fn main() -> Result<(), Error> {
    // argument handling

    let x = 1;
    // setup (w/ args)
    setup()
}




