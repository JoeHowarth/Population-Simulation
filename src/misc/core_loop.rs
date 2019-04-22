use specs::{
    world::Generation,
    ReadStorage,
    Join,
    prelude::*,
};

use std::{
    sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn},
    thread,
    thread::JoinHandle,
    time::{Instant, Duration},
    //fmt::Debug,
    //sync::atomic::{AtomicUsize, Ordering},
    result::Result,
//    collections::HashMap,
};

use crate::{
    networking::{
        *,
    },
    terrain::*,
    pop::*,
    agriculture::*,
    misc::{
        time::Date,
        components::DeltaTime
    },
};
use chrono::Datelike;

pub fn game_loop(mut world: World, mut dispatcher_daily: Dispatcher) {
    let frame_target = Duration::from_millis(1000);
    let mut last = Instant::now();
    let mut total_frames = 0;
    loop {
        let start = Instant::now();
        let dt = start - last;
        last = Instant::now();
        {
            let mut delta = world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_secs_f64() as f32);
        }


        dispatcher_daily.dispatch(&mut world.res);
        world.maintain();



        if total_frames > 500_000 { break; }
        total_frames += 1;

        let used = Instant::now() - start;
        let sleep_time = std::cmp::max(frame_target, used) - used;
        if total_frames % 100 == 0 {
            info!("Utilization: {:.2}%", (1. - sleep_time.as_secs_f64() / frame_target.as_secs_f64()) * 100.);
        }


        thread::sleep(sleep_time);
    }
}
