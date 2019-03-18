use crate::terrain::mesh::{Mesh, MeshJson};
use super::{SubMsg};
use std::fmt::Debug;
use serde::{Serialize, Deserialize};


#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ReceiveTypeWrapper {
    MapComponentTag(MapCompTag),
    SubMsg(SubMsg),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MapCompTag {
    name: String,
    data: Vec<usize>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMsg {
    Action(Action),
    SubReq(SubReq),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Action {
    section: Sections,
    body: String
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SubReq {
    pub section: Sections,
    pub insert: bool,
    pub component: String,
    pub keys: Option<Vec<String>>
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Sections {
    Agr,
    Pop,
    Terr,
    Date,
}

#[derive(Debug, Clone, Serialize)]
pub enum ServerMsg<'a, T: Debug + Clone + Serialize> {
    SubPush(SubPush<'a, T>),
    Other(String)
}

#[derive(Debug, Clone, Serialize)]
pub struct SubPush<'a, T: Debug + Clone + Serialize> {
    pub section: Sections,
    pub component: &'a str,
    pub keys: Option<Vec<String>>,
    pub data: T,
}
