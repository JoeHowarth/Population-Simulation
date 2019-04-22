use ws::{WebSocket, Handshake, CloseCode, Handler, Message, Sender as WS_sender};
use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread::{JoinHandle, ThreadId};
use std::sync::atomic::{AtomicUsize, Ordering};
use failure::Error;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use crate::terrain::mesh::{Mesh, MeshJson};

use crate::networking::SubMsg;
use crate::networking::{types::*};
use std::collections::VecDeque;
use crate::networking::CONNECTION_COUNT;
use crate::misc::systems::MutationMsg;
use std::sync::{RwLock, RwLockReadGuard};

lazy_static! {
    pub static ref INIT_DATA: RwLock<Option<String>> = RwLock::new(None);
}

pub struct Server {
    pub out: ws::Sender,
    pub ws_in: ThreadOut<WS_sender>,
    pub sub_send: ThreadOut<SubMsg>,
    pub sub_req_send: ThreadOut<SubReq>,
    pub rec_type_send: ThreadOut<ReceiveTypeWrapper>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        info!("Connection Establish");
        let old_connection_count = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
        info!("live connections: {}", CONNECTION_COUNT.load(Ordering::SeqCst));

        let data = INIT_DATA.read()
                            .expect("Couldn't get read on init_data");
        if data.is_some() {
            let s = data.clone().unwrap();
            self.out.send(s).expect("failed to send from on_open");
        }


        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if msg.len() < 100 { debug!("Server got message '{}'. ", msg); }

        use ClientMsg::*;
        match msg {
            Message::Text(string) => {
                let msg: ClientMsg = serde_json::from_str(&string).expect(&format!("Failed to parse ClientMessage: {:?}", &string));
                match msg {
                    SubReq(r) => self.sub_req_send.send(r).expect("error sending"),
                    _ => unreachable!()
                };
            }
            Message::Binary(_) => {
                error!("[WS ERROR]:Can't receive binary messages yet");
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        info!("WebSocket closing for ({:?}) {}", code, reason);
    }
}

pub struct WsReturn {
    pub server_thread: JoinHandle<Result<(), Error>>,
    pub out: ClientSender,
    pub sub_recv: ThreadIn<SubMsg>,
    pub sub_req_recv: ThreadIn<SubReq>,
    pub rec_type_recv: ThreadIn<ReceiveTypeWrapper>,
}


#[derive(Clone)]
pub struct ClientSender(pub WS_sender);

impl ClientSender {
    pub fn sub_push<T: Debug + Clone + Serialize>(&self, sp: SubPush<T>) {
        let sm = ServerMsg::SubPush(sp);
        self.send(&sm);
    }

    pub fn send_json(&self, json: &str) {
        self.0.send(json)
            .expect(&format!("failed to send {}", json));
    }

    pub fn send<T: Debug + Serialize>(&self, x: &T) {
        let json = serde_json::to_string(x)
            .expect(&format!("failed to serialize {:?}", x));
        self.0.send(json)
            .expect(&format!("failed to send {:?}", x));
    }

    /* pub fn send_with_wrapper<T: Debug + Serialize>(&self, x: &T) {
        let json = TypeWrapper::wrap_and_json(x)
            .expect(&format!("failed to serialize {:?}", x));
        self.0.send(json)
            .expect(&format!("failed to send {:?}", x));
    } */

    pub fn send_str(&self, s: &str) {
        let json = format!("{{ \"msg\": \"{}\" }}", s);
        self.0.send(json)
            .expect(&format!("failed to send string"));
    }
}


/*
#[derive(Serialize, Clone, Debug)]
struct TypeWrapper<'a, T: Debug + Serialize> {
    inner: &'a T,
    name: &'a str,
}

impl<'a, T: Debug + Serialize> TypeWrapper<'a, T> {
    pub fn new(inner: &'a T) -> Self {
        Self { inner: inner, name: type_name_of(&*inner) }
    }
    pub fn wrap_and_json(inner: &'a T) -> Result<String, Error> {
        let wrapper = Self::new(inner);
        Ok(serde_json::to_string(&wrapper)?)
    }
}
*/


pub fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

