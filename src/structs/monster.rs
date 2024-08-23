use serde::{Deserialize, Serialize};

pub enum EMonster {
    Koi = 49,
    Tanuki = 50,
    Dodo = 52,
    Skorch = 68,
    Shockhopper = 75,
    Bard = 110,
    Vaero = 7,
    Frosty = 15,
    MadEye = 20,
    Nightwing = 21,
    Toxiquus = 22,
    Magmamoth = 25,
    Glowfly = 27,
    Raduga = 32,
    Kanko = 51,
    Glowdra = 64,
    Draconov = 65,
    Thanatos = 90,
    Vertraag = 99,
    Ascendant = 101,
    Amberlagna = 107,
}

#[derive(Clone, Copy)]
pub enum Ability {
    BreakWall,
    Mount,
    Flying,
    ImprovedFlying,
    SecretVision,
    Ignite,
    Light,
    Crush,
    BigRock,
    Grappling,
    BlobForm,
    Levitate,
    MountOrFlying,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Monster {
    pub id: u32,
    pub name: String,
    pub explore_action: u32,
    pub monster_types: Vec<u32>,
}

#[derive(Deserialize, Serialize)]
pub struct MonsterType {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
pub struct ExploreAction {
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExploreAbility {
    pub name: String,
    pub explore_actions: Vec<u32>,
}
