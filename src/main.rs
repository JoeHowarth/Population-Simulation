#![feature(duration_float)]
#![allow(dead_code, unused_variables, unused_imports)]
#![feature(core_intrinsics)]


extern crate failure;
extern crate population_simulation;


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
        mesh::Mesh,
        components::*,
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




