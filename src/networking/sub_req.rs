use specs_derive;
use specs::prelude::*;
use super::{
    *,
    types::Sections,
};
use std::{
    sync::mpsc::{channel, Sender, Receiver},
    collections::HashSet,
    hash::Hash,
};
use crate::{
    agriculture::sub_req::*,
    pop::sub_req::*,
    terrain::sub_req::*,
};
use serde::de::DeserializeOwned;

pub struct SubReqDispatcher {
    pub recv: Receiver<SubReq>
}

#[allow(type_alias_bounds)]
type Foo<'a, T: Eq + Hash> = Write<'a, HashSet<T>>;

impl<'a> System<'a> for SubReqDispatcher {
    type SystemData = (Foo<'a, AgrData>,
                       Foo<'a, TerrData>,
                       Foo<'a, PopData>);

    fn run(&mut self, (mut agr, mut terr, mut pop): Self::SystemData) {
        while let Ok(sr) = self.recv.try_recv() {
            match sr.section {
                Sections::Agr => update_sub_reqs(sr, &mut agr),
                Sections::Terr => update_sub_reqs(sr, &mut terr),
                Sections::Pop => update_sub_reqs(sr, &mut pop),
            }.expect("[SubReqDispatcher Error] failed to parse component")
        }
    }
}

pub fn update_sub_reqs<T>(mut sr: SubReq, set: &mut HashSet<T>) -> Result<(), Error>
    where T: Eq + Hash + DeserializeOwned
{
    sr.component.insert(0, '\"');
    sr.component.push('\"');
    dbg!(&(sr.component));
    let comp_type = serde_json::from_str(&(sr.component))?;
    if sr.insert {
        set.insert(comp_type);
    } else {
        set.remove(&comp_type);
    }
    Ok(())
}

