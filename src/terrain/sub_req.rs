use super::*;

pub struct Terr;

#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialOrd, PartialEq, Eq, Ord, Hash)]
pub enum TerrData {
    Region,
    Weather,
    RiverID,
    LandMassID,
    TileTopography,
    TileAdjacency,
}
