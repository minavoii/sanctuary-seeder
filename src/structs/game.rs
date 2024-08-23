use serde::{Deserialize, Serialize};

use crate::structs::modes::{bravery::BraveryData, relic::RelicData};

#[derive(Serialize, Deserialize)]
pub struct Game {
    pub seed: u32,
    pub is_randomizer: bool,
    pub is_bravery: bool,
    pub is_relic: bool,

    pub mapping: Option<Vec<Option<u32>>>,
    pub bravery_data: Option<BraveryData>,
    pub relics: Option<RelicData>,
}

impl PartialEq for Game {
    fn eq(&self, other: &Self) -> bool {
        self.seed == other.seed
            && self.is_randomizer == other.is_randomizer
            && self.is_bravery == other.is_bravery
            && self.is_relic == other.is_relic
            && self.mapping == other.mapping
            && self.bravery_data == other.bravery_data
            && self.relics == other.relics
    }
}

impl Game {
    /// Returns true if this game fails to generate.
    ///
    /// This can happen for certain seeds with both Bravery and Randomizer modes enabled.
    pub fn is_bad_seed(&self) -> bool {
        self.is_bravery && self.bravery_data.is_none()
    }
}
