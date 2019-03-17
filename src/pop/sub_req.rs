use super::*;

pub struct Pop;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum PopData {
    RegionPop,
}
