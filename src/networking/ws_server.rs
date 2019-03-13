use ws::{WebSocket, Handshake, CloseCode, Handler, Message, Sender as WS_sender};
use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread::JoinHandle;
use std::sync::atomic::{AtomicUsize, Ordering, ATOMIC_USIZE_INIT};
use failure::Error;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use crate::map::mesh::{Mesh, MeshJson};

use crate::networking::SubMsg;
use crate::networking::types::{ReceiveTypeWrapper, MapCompTag};
use std::collections::VecDeque;
use crate::networking::CONNECTION_COUNT;


pub struct Server {
    pub out: ws::Sender,
    pub ws_in: ThreadOut<WS_sender>,
    pub sub_send: ThreadOut<SubMsg>,
    pub rec_type_send: ThreadOut<ReceiveTypeWrapper>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        info!("Connection Establish");
        let old_connection_count = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
        info!("live connections: {}", CONNECTION_COUNT.load(Ordering::SeqCst));
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {
        if msg.len() < 100 { debug!("Server got message '{}'. ", msg); }

        match msg {
            Message::Text(string) => {
                if string.len() < 200 { debug!("Received string: {}", string); }
                if let Ok(msg) = serde_json::from_str(&string) {
                    if string.len() < 200 { debug!("Received type: {:?}", msg); }
                    match msg {
                        ReceiveTypeWrapper::SubMsg(sub) => {
                            debug!("Subscription Message Received");
                            self.sub_send.send(sub).expect("Couldn't send SubMsg");
                        }

                        ReceiveTypeWrapper::MapComponentTag(data) => {
//                            self.rec_type_send.send(msg).expect("sending whole recv type falied");
                        }
                    }
                } else {
                    error!("[WS ERROR] Unrecognized message: {}", string);
                }
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
    pub rec_type_recv: ThreadIn<ReceiveTypeWrapper>,
}


#[derive(Clone)]
pub struct ClientSender(pub WS_sender);

impl ClientSender {
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

