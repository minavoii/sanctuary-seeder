use crate::{
    data::macros::load_data,
    structs::{
        map::{AreaData, MapArea},
        monster::{ExploreAbility, ExploreAction, Monster, MonsterType},
        relic::Relic,
    },
};

pub struct GameData {
    pub monsters: Vec<Monster>,
    pub monster_types: Vec<MonsterType>,
    pub swimming_monsters: Vec<u32>,

    pub actions: Vec<ExploreAction>,
    pub abilities: Vec<ExploreAbility>,

    pub areas: Vec<MapArea>,
    pub area_data: Vec<AreaData>,
    pub relics: Vec<Relic>,
}

impl GameData {
    pub fn new() -> GameData {
        GameData {
            monsters: load_data!("../../res/out/data/MonsterJournalList.dat", Vec<Monster>),
            monster_types: load_data!("../../res/out/data/MonsterTypes.dat", Vec<MonsterType>),
            swimming_monsters: load_data!("../../res/out/data/SwimmingMonsterList.dat", Vec<u32>),

            actions: load_data!("../../res/out/data/ExploreActions.dat", Vec<ExploreAction>),
            abilities: load_data!(
                "../../res/out/data/ExploreAbilities.dat",
                Vec<ExploreAbility>
            ),

            areas: load_data!("../../res/out/data/MonsterAreas.dat", Vec<MapArea>),
            area_data: load_data!("../../res/out/data/AreaData.dat", Vec<AreaData>),
            relics: load_data!("../../res/out/data/Relics.dat", Vec<Relic>),
        }
    }
}
