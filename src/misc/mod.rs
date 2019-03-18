pub mod components;
pub mod core_loop;
pub mod normalize;
pub mod time;
pub mod systems;

use specs::prelude::*;
pub use self::{
    time::*,
};

pub fn init_misc_ecs(world: &mut World) {
    init_date(world);
}
