mod subscription_system;

use std::fmt::Debug;
use std::marker::PhantomData;

use serde::Serialize;
use serde_json::Value;
use specs::prelude::*;

use ::components::*;
use ::ws_server::ClientSender;


pub struct PrintPosSys;

impl<'a> System<'a> for PrintPosSys {
    type SystemData = ReadStorage<'a, Position>;

    fn run(&mut self, position: Self::SystemData) {
        use specs::Join;

        for pos in (&position).join() {
            println!("Current {:?}", &pos);
        }
    }
}

pub struct UpdatePos;

impl<'a> System<'a> for UpdatePos {
    type SystemData = (ReadStorage<'a, Velocity>,
                       WriteStorage<'a, Position>,
                       Read<'a, DeltaTime>);

    fn run(&mut self, (vel, mut pos, delta): Self::SystemData) {
        use specs::Join;
        let delta = delta.0;

        for (vel, pos) in (&vel, &mut pos).join() {
            pos.x += vel.x * delta;
            pos.y += vel.y * delta;
        }
    }
}

#[derive(Default)]
pub struct SendSys<T: Serialize + Component + Debug> {
    mutation: String,
    phantom: PhantomData<T>,
}

impl<T: Serialize + Component + Debug>  SendSys<T> {
    pub fn new<S: Into<String>>(mutation: S) -> Self {
        SendSys {
            mutation: mutation.into(),
            phantom: PhantomData,
        }
    }
}

#[derive(Serialize, Debug)]
struct MutationMsg<T: Serialize + Debug> {
    mutation: String,
    inner: Vec<T>
}

impl<'a, T: Serialize + Component + Debug> System<'a> for SendSys<T> {
    type SystemData = (ReadStorage<'a, T>,
                       ReadExpect<'a, ClientSender>);

    fn run(&mut self, (t, out): Self::SystemData) {
        use specs::Join;

        let elts = t.join().collect::<Vec<_>>();

        let msg = MutationMsg {
            mutation: self.mutation.clone(),
            inner: elts
        };



        out.send(&msg)

    }
}


