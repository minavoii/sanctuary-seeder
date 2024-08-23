use serde::{Deserialize, Serialize};

pub enum Area {
    MountainPath,
    BlueCaves,
    StrongholdDungeon,
    AncientWoods,
    SnowyPeaks,
    SunPalace,
    HorizonBeach,
    MagmaChamber,
    MysticalWorkshop,
    Underworld,
    AbandonedTower,
    BlobBurg,
    ForgottenWorld,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MapArea {
    pub id: u32,
    pub name: String,
    pub monsters: Vec<u32>,
    pub wild_monsters: Vec<u32>,
    pub randomizer_check_list: Vec<u32>,
    pub champions: Vec<u32>,
    pub area_data: Vec<u32>,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AreaData {
    pub area_id: u32,
    pub scene_id: u32,
    pub scene_name: String,
    pub chests: Vec<u32>,
}
