
use specs::prelude::*;
use serde::{Serialize,
            Deserialize};
use serde_json;
use std::{fmt::Debug,
          sync::mpsc::{Sender, Receiver, channel, TryRecvError},
          marker::PhantomData};
use crate::networking::ClientSender;
use crate::systems::MutationMsg;
use crate::map::mesh::Mesh;
use crate::components::tiles::FarmData;

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

pub struct SubscriptionManager {
    pub recv: Receiver<SubMsg>,
    pub out: ClientSender,
}

impl SubscriptionManager {

    pub fn dispatch(&self, world: &mut World) {
        match self.recv.try_recv() {
            Ok(sub) => {

                let inner = match sub.data_req {
                    Subscribable::Height => {
                        let mesh = world.read_resource::<Mesh>();
                        mesh.height.clone()
                    },
                    /*
                    Subscribable::Fertility => {
                        let farmdata = world.read_storage::<FarmData>();
                        let data = farmdata.join().collect();
                    },
                    Subscribable::Other(var) => {
                        println!("Subscribe by string not implemented");
                        vec![]
                    },
                    */
                    _ => {
                        warn!("only subscribe to height implemented yet");
                        vec![]
                    }
                };

                let msg = MutationMsg {
                    mutation: sub.mutation,
                    inner,
                };

                self.out.send(&msg);

            },
            Err(TryRecvError::Disconnected) => {
                error!("bad, subscription manager disconnected!");
            },
            Err(TryRecvError::Empty) => {}
        }
    }

}



#[derive(Debug, Serialize, Deserialize)]
pub enum Subscribable {
    Height,
    Rivers,
    Fertility,
    Other(String),
    Num(usize),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SubMsg {
    mutation: String,
    data_req: Subscribable
}
