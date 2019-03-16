use super::*;
use specs_derive;
use specs::prelude::*;


#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct BaseFarmData {
    /// f in (0,1): how fertile the underlying soil is
    pub fertility: f32,
    /// total land area that can be farmed without terracing etc.
    pub arable: f32,
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct FarmData {
    /// Area Under Cultivation: km^2 used for farming currently
    pub auc: f32,
    /// Area cleared and 'ready' to be farmed
    pub cleared: f32,
    /// farming efficiency, may be derived quantity instead..
    pub fe: f32,
}


#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct FoodStock {
    /// amount of stored food in region/tile
    pub food: f32,
    /// total land area that can be farmed without terracing etc.
    pub arable: f32,
}


