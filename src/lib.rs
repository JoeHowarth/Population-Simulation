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
extern crate chrono;
extern crate ord_subset;

pub mod networking;
pub mod systems;
pub mod components;
pub mod terrain;
pub mod normalize;
pub mod agriculture;
pub mod time;
pub mod pop;

use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread;
use std::thread::JoinHandle;
use std::time::{Instant, Duration};
use std::fmt::Debug;
use std::sync::atomic::{AtomicUsize, Ordering};

use failure::Error;
//use fnv::{FnvHashMap, FnvHashSet };
use std::collections::HashMap;
use ws::Sender as WsSender;
use serde::Serialize;
use specs::prelude::*;

use crate::networking::*;
use crate::components::*;
use crate::systems::*;
use specs::world::Generation;
//use crate::components::tiles::Tile2Entity;
use specs::ReadStorage;
use specs::shred::DynamicSystemData;
use anymap::AnyMap;
//use std::sync::RwLock;


//lazy_static! {
//    pub static ref SETTINGS : RwLock<config::Config> = RwLock::new(config::Config::default());
//}

const TIMESTAMP_FORMAT: &'static str = "%m-%d %H:%M:%S";

/* Log levels
critical
error
warning
info
debug
trace
*/

pub fn setup_logger() -> slog::Logger {
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
