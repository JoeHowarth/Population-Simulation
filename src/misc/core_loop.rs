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
        subscription_system::SubscriptionManager,
    },
    terrain::*,
    pop::*,
    agriculture::*,
    misc::components::DeltaTime,
};

pub fn game_loop(mut world: World, mut dispatcher: Dispatcher) {
    let frame_target = Duration::from_millis(3000);
    let mut last = Instant::now();
    let mut total_frames = 0;
    loop {
        let start = Instant::now();
        let dt = start - last;
        last = Instant::now();
        {
            let mut delta = world.write_resource::<DeltaTime>();
            *delta = DeltaTime(dt.as_float_secs() as f32);
        }

        dispatcher.dispatch(&mut world.res);
        world.maintain();


        if total_frames > 5000 { break; }
        total_frames += 1;

        let used = Instant::now() - start;
        let sleep_time = std::cmp::max(frame_target, used) - used;
        info!("Utilization: {:.2}%", (1. - sleep_time.as_float_secs() / frame_target.as_float_secs()) * 100.);


        thread::sleep(sleep_time);
    }
}
