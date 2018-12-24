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
use crate::type_string::print_type_of;
use crate::type_string::type_name_of;
use crate::map::mesh::{Mesh, MeshJson};

static CONNECTION_COUNT: AtomicUsize = ATOMIC_USIZE_INIT;
const ADDRESS: &str = "127.0.0.1:8090";


use crate::systems::subscription_system::SubMsg;

#[derive(Debug, Serialize, Deserialize)]
pub enum ReceiveTypeWrapper {
    Mesh(MeshJson),
    SubMsg(SubMsg),
}

pub struct Server {
    pub out: ws::Sender,
    pub ws_in: ThreadOut<WS_sender>,
    pub mesh_send: ThreadOut<Mesh>,
    pub sub_send: ThreadOut<SubMsg>,
}

impl Handler for Server {
    fn on_open(&mut self, _: Handshake) -> ws::Result<()> {
        println!("Connection Establish");
        let old_connection_count = CONNECTION_COUNT.fetch_add(1, Ordering::SeqCst);
        println!("live connections: {}", old_connection_count + 1);
        Ok(())
    }
    fn on_message(&mut self, msg: Message) -> ws::Result<()> {

        if msg.len() < 100 {println!("Server got message '{}'. ", msg);}

        match msg {
            Message::Text(string) => {
                if string.len() < 200 {println!("Received string: {}", string);}
                if let Ok(msg) = serde_json::from_str(&string) {
                    if string.len() < 200 {println!("Received type: {:?}", msg);}
                    match msg {
                        ReceiveTypeWrapper::Mesh(meshjson) => {
                            let mesh: Mesh = meshjson.into();
                            println!("Mesh Received");
                            self.mesh_send.send(mesh).expect("Couldn't send mesh");
                        },

                        ReceiveTypeWrapper::SubMsg(sub) => {
                            println!("Subscription Message Received");
                            self.sub_send.send(sub).expect("Couldn't send SubMsg");
                        }
                    }
                } else {
                    println!("[WS ERROR] Unrecognized message: {}", string);
                }

            }
            Message::Binary(_) => {
                println!("[WS ERROR]:Can't receive binary messages yet");
            }
        }

        Ok(())
    }

    fn on_close(&mut self, code: CloseCode, reason: &str) {
        println!("WebSocket closing for ({:?}) {}", code, reason);
//        println!("Shutting down server after first connection closes.");
//        self.out.shutdown().unwrap();
    }
}

struct SubscribeMsg {
    mutation: String
}

pub struct WsReturn {
    pub server: JoinHandle<Result<(), Error>>,
    pub out: ClientSender,
    pub mesh_recv: ThreadIn<Mesh>,
    pub sub_recv: ThreadIn<SubMsg>,
}

pub fn create_ws_server() -> Result<WsReturn, Error> {
    let (ws_in, ws_out) = channel();
    let (mesh_send, mesh_recv) = channel();
    let (sub_send, sub_recv) = channel();


    let server: JoinHandle<Result<(), Error>> = thread::spawn(move || {
        let ws = WebSocket::new(|out: WS_sender| {
            Server {
                out,
                ws_in: ws_in.clone(),
                mesh_send: mesh_send.clone(),
                sub_send: sub_send.clone(),
            }
        })?;
        ws_in.send(ws.broadcaster())?;
        ws.listen(ADDRESS)?;
        Ok(())
    });

    let out = ws_out.recv()?;
    while CONNECTION_COUNT.load(Ordering::Relaxed) < 1 {
        wait(10);
    }

    Ok(WsReturn { server, out: ClientSender(out), mesh_recv, sub_recv })
}

#[derive(Clone)]
pub struct ClientSender(WS_sender);

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

    pub fn send_with_wrapper<T: Debug + Serialize>(&self, x: &T) {
        let json = TypeWrapper::wrap_and_json(x)
            .expect(&format!("failed to serialize {:?}", x));
        self.0.send(json)
            .expect(&format!("failed to send {:?}", x));
    }

    pub fn send_str(&self, s: &str) {
        let json = format!("{{ \"msg\": \"{}\" }}", s);
        self.0.send(json)
            .expect(&format!("failed to send string"));
    }
}


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


pub fn wait(millis: u64) {
    thread::sleep(Duration::from_millis(millis));
}

fn counter(out: WS_sender) {
    thread::spawn(move || {
        for i in 0..100000 {
            wait(100);
            out.send(i.to_string()).unwrap_or(());
        }
    });
}
