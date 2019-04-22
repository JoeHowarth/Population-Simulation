pub mod ws_server;
pub mod types;
pub mod sub_req;

use ws::{WebSocket, Handshake, CloseCode, Handler, Message, Sender as WS_sender};
use std::sync::mpsc::{channel, Sender as ThreadOut, Receiver as ThreadIn};
use std::thread::JoinHandle;
use std::sync::atomic::{AtomicUsize, Ordering};
use failure::Error;
use std::thread;
use std::time::Duration;
use serde::{Deserialize, Serialize};
use serde_json;
use std::fmt::Debug;
use specs::prelude::*;

use crate::terrain::mesh::{MeshWrapper, Mesh, MeshJson};
use crate::terrain::components::*;
pub use self::ws_server::*;
pub use self::types::*;
pub use self::sub_req::*;
use crate::misc::systems::MutationMsg;


pub static CONNECTION_COUNT: AtomicUsize = AtomicUsize::new(0);
const ADDRESS: &str = "127.0.0.1:8090";

pub fn create_ws_server() -> Result<impl FnOnce() -> Result<WsReturn, Error>, Error> {
    let (ws_in, ws_out) = channel();
    let (sub_req_send, sub_req_recv) = channel();

    let server_thread: JoinHandle<Result<(), Error>> = thread::spawn(move || {
        let ws = WebSocket::new(|out: WS_sender| {
            Server {
                out,
                ws_in: ws_in.clone(),
                sub_req_send: sub_req_send.clone(),
            }
        })?;
        ws_in.send(ws.broadcaster())?;
        ws.listen(ADDRESS)?;
        Ok(())
    });

    Ok(move || {
        let out = ws_out.recv()?;
        let mut i = 0;
        while CONNECTION_COUNT.load(Ordering::SeqCst) == 0 {
            wait(100);
            if i > 10 {
                info!("waiting...   {}", CONNECTION_COUNT.load(Ordering::SeqCst));
                i = 0;
            }
            i += 1;
        }
        trace!("not blocking anymore");
        Ok(WsReturn { server_thread, out: ClientSender(out), sub_req_recv})
    })
}

pub fn send_displayable_for_tag<T: Component>((storage, ids, mesh, out):
                                          (ReadStorage<T>,
                                           ReadStorage<TileID>,
                                           ReadExpect<Mesh>,
                                           ReadExpect<ClientSender>))
{
    let mut data: Vec<f32> = vec![0.; mesh.ids.len()];

    let mut count = 0;
    for (_, &TileID { id }) in (&storage, &ids).join() {
        count += 1;
        data[id] = 1.0;
    }
    debug!("count: {}", count);

    out.send(&MutationMsg {
        mutation: "setMapData".into(),
        inner: data,
    });
}

pub fn send_displayable_for_data<T: Component>((storage, ids, mesh, out):
                                           (ReadStorage<T>,
                                            ReadStorage<TileID>,
                                            ReadExpect<Mesh>,
                                            ReadExpect<ClientSender>), f: fn(&T) -> f32)
{
    let mut data: Vec<f32> = vec![0.; mesh.ids.len()];

    let mut count = 0;
    for (d, &TileID { id }) in (&storage, &ids).join() {
        count += 1;
        data[id] = f(d);
    }
    debug!("count: {}", count);

    out.send(&MutationMsg {
        mutation: "setMapData".into(),
        inner: data,
    });
}

pub fn send_init_data(world: &mut World) -> Result<(), Error> {
    use crate::networking::ws_server::INIT_DATA;
    // send mesh to client
    info!("sending mesh to client...");
    world.exec(|(mut mesh_json, out): (WriteExpect<Option<MeshJson>>, ReadExpect<ClientSender>)| {
        let inner = std::mem::replace(&mut mesh_json.as_mut(), None);
        let inner = inner.expect("No mesh_json loaded when sending to client");
        let wrapper = MutationMsg {mutation: "setHMesh".into(), inner};
        out.send(&wrapper);
        let mut init_data = INIT_DATA.write().expect("can't get INIT_DATA lock");
        let json = serde_json::to_string(&wrapper)
            .expect(&format!("failed to serialize {:?}", &wrapper));
        *init_data = Some(json);
    });
    trace!("sent");

    Ok(())
    // other initial data goes here

}
