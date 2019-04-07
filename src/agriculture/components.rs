use super::*;
use specs_derive;
use specs::prelude::*;
use vec_map::VecMap;
use crate::terrain::*;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum AgrData {
    BaseFarmData,
    FarmData,
    FoodStock,
}

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
    // farming efficiency, may be derived quantity instead..
    //pub fe: f32,
}


#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct FoodStock {
    /// amount of stored food in region/tile
    pub bushels: f32,
}

#[derive(Component, Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq)]
pub struct RegBaseFarmData {
    /// f in (0,1): how fertile the underlying soil is
    pub fertility: f32,
    /// total land area that can be farmed without terracing etc.
    pub arable: f32,
}

impl RegBaseFarmData {
    pub fn from_tile(tile: &BaseFarmData) -> RegBaseFarmData {
        RegBaseFarmData {
            fertility: tile.fertility,
            arable: tile.arable
        }
    }

    pub fn merge(a: usize, b: usize, all: &mut VecMap<RegBaseFarmData>, topo: &VecMap<RegionTopography>) {
        let n = topo[a].tiles as f32;
        let m = topo[b].tiles as f32;

        let mut r = all.remove(a).unwrap();
        let b = all.remove(b).unwrap();

        r.fertility = (r.fertility * n + b.fertility * m) / (n + m);
        r.arable += b.arable;

        all.insert(a, r);
    }
}



