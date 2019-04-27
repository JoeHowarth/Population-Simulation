pub mod components;
pub mod core_loop;
pub mod normalize;
pub mod time;
pub mod systems;
pub mod graph_ds;

use crate::prelude::*;
pub use self::{
    time::*,
};

pub fn init_misc_ecs(world: &mut World) {
    init_date(world);
}

#[derive(PartialOrd, PartialEq, Clone, Debug)]
pub struct WeightedNode<T: PartialOrd + PartialEq + Clone + Debug> {
    pub weight: i32,
    pub inner: T,
}

impl<T: PartialOrd + PartialEq + Clone + Debug> Ord for WeightedNode<T> {
    fn cmp(&self, other: &WeightedNode<T>) -> Ordering {
        self.weight.cmp(&other.weight)
    }
}

impl<T: PartialOrd + PartialEq + Clone + Debug> Eq for WeightedNode<T> {}
