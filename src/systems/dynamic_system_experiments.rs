use ws_server::ClientSender;
use fnv::{FnvHashMap as HashMap, FnvHashSet as HashSet};
use specs::prelude::*;
use specs::storage::{AnyStorage};
use specs::shred::{Accessor,
                   AccessorCow,
                   ResourceId,
                   DynamicSystemData,
                   Resource,
                   MetaTable};
use specs::shred::cell::{Ref, RefMut};
use serde_json;
use std::marker::PhantomData;



#[derive(Clone, Debug)]
pub struct Dependencies {
    pub reads: HashMap<String, ResourceId>,
    pub writes: HashMap<String, ResourceId>,
}

impl Dependencies {
    pub fn new(read_strs: &[&str], write_strs: &[&str], table: &ResourceTable) -> Dependencies {

        let reads = read_strs.iter()
            .map(|name| ((*name).into(), table.get(name)))
            .collect::<HashMap<String, ResourceId>>();
        let writes = write_strs.iter()
            .map(|name| ((*name).into(), table.get(name)))
            .collect::<HashMap<String, ResourceId>>();
        Dependencies {
            reads,
            writes: Default::default(),
        }
    }
}

impl Accessor for Dependencies {
    fn try_new() -> Option<Self> {
        // there's no default for this
        None
    }

    fn reads(&self) -> Vec<ResourceId> {
        self.reads.values()
            .map(Clone::clone)
            .collect()
    }

    fn writes(&self) -> Vec<ResourceId> {
        self.writes.values()
            .map(Clone::clone)
            .collect()
    }
}

/// A dynamic system that represents and calls the script.
pub struct SubscriptionSystem {
    pub dependencies: Dependencies,
    /// just a dummy, you would want an actual script handle here
    pub out: ClientSender,
}

impl<'a> System<'a> for SubscriptionSystem {
    type SystemData = SubscriptionSystemData<'a>;

    fn run(&mut self, data: Self::SystemData) {
//        let data = data.join().collect::<Vec<_>>();

//        self.out.send(data);
    }

    fn accessor<'b>(&'b self) -> AccessorCow<'a, 'b, Self> {
        AccessorCow::Ref(&self.dependencies)
    }

    fn setup(&mut self, _res: &mut Resources) {
        // this could call a setup function of the script
    }
}

//type ReadStorage<'a, T> = Storage<'a, T, Fetch<'a, MaskedStorage<T>>>;

pub struct SubscriptionSystemData<'a> {
    reads: Vec<Ref<'a, Box<Resource + 'static>>>,
    writes: Vec<RefMut<'a, Box<Resource + 'static>>>,
}

impl<'a> DynamicSystemData<'a> for SubscriptionSystemData<'a> {
    type Accessor = Dependencies;

    fn setup(_accessor: &Dependencies, _res: &mut Resources) {}

    fn fetch(access: &Dependencies, res: &'a Resources) -> Self {
        let meta = res.fetch::<MetaTable<AnyStorage>>();
        let reads = access.reads.values()
            .map(|id| res
                .try_fetch_internal(id.0)
                .expect("bug: the requested resource does not exist")
                .borrow())
            .collect();


        let writes = access.reads.values()
            .map(|id| res
                .try_fetch_internal(id.0)
                .expect("bug: the requested resource does not exist")
                .borrow_mut())
            .collect();

        SubscriptionSystemData { reads, writes }
    }
}

#[derive(Default)]
/// Maps resource names to resource ids.
/// Idea: register with world at same time as create mapping
///       then none missed => no bugs...
pub struct ResourceTable {
    map: HashMap<String, ResourceId>,
}

impl ResourceTable {
    pub fn new() -> Self {
        ResourceTable {
            map: HashMap::default(),
        }
    }

    pub fn register<T: Resource>(&mut self, name: &str) {
        self.map.insert(name.to_owned(), ResourceId::new::<T>());
    }

    pub fn get(&self, name: &str) -> ResourceId {
        *self.map.get(name).unwrap()
    }
}
