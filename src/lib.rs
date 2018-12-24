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


pub mod ws_server;
#[macro_use]
pub mod type_string;
pub mod systems;
pub mod components;
pub mod map;

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

use crate::ws_server::*;
use crate::type_string::*;
use crate::components::*;
use crate::systems::*;
use specs::world::Generation;
use crate::components::tiles::Tile2Entity;
use specs::ReadStorage;
use specs::shred::DynamicSystemData;
use anymap::AnyMap;



