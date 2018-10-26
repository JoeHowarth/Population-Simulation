pub mod tiles;
use specs::prelude::*;
use specs_derive;
use specs::world::{Entity, Generation, Index};


#[derive(Component, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: f32,
    pub y: f32,
}

#[derive(Component, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}

#[derive(Default)]
pub struct DeltaTime(pub f32);

/* don't know why, but Entity::new(...) can't be found..
#[derive(Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct FakeEntity {
    gen: i32,
    id: u32,
}

impl From<Entity> for FakeEntity {
    fn from(e: Entity) -> Self {
        FakeEntity {
            gen: e.gen().id(),
            id: e.id()
        }
    }
}

impl Into<Entity> for FakeEntity {
    fn into(self) -> Entity {
        let i:Index = self.id;
        let g = Generation::new(self.gen);
        Entity::new(self.id , Generation::new(self.gen) )
    }
}
*/
