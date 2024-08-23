use serde::{Deserialize, Serialize};
use unity_random::Random;

use crate::{
    data::{
        macros::{area, monster},
        GAME_DATA,
    },
    structs::modes::bravery::BraveryData,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct RelicData {
    pub list: Vec<u32>,
    pub area_chests: Vec<(String, u32)>,
}

impl PartialEq for RelicData {
    fn eq(&self, other: &Self) -> bool {
        self.list == other.list && self.area_chests == other.area_chests
    }
}

pub struct RelicMode<'a> {
    random: &'a mut Random,

    is_bravery: bool,
    bravery_data: &'a Option<BraveryData>,

    list: Vec<u32>,
}
impl<'a> RelicMode<'a> {
    pub fn get_relics(
        random: &mut Random,
        is_bravery: bool,
        bravery_data: &Option<BraveryData>,
    ) -> Option<RelicData> {
        let mut instance = RelicMode {
            random,

            is_bravery,
            bravery_data,

            list: vec![],
        };

        let mut area_chests = vec![];

        for area in 0..GAME_DATA.areas.len() as u32 {
            let random_relic = instance.get_random_relic(area);
            let random_chest = instance.get_random_chest_in_area(area);

            instance.list.push(random_relic);
            area_chests.push(random_chest);
        }

        Some(RelicData {
            list: instance.list,
            area_chests,
        })
    }

    fn get_random_relic(&mut self, area: u32) -> u32 {
        let random_relic =
            &GAME_DATA.relics[self.random.range_int(0, GAME_DATA.relics.len() as i32) as usize];

        if self.list.contains(&random_relic.id) {
            return self.get_random_relic(area);
        }

        if self.is_bravery && random_relic.monster_type_restriction != 0 {
            let bravery_data = self.bravery_data.as_ref().unwrap();
            let mut monster_type_list: Vec<u32> = vec![];

            for (i, egg) in bravery_data.eggs.iter().enumerate() {
                if i as u32 == area {
                    for monster_type in &monster!(*egg as u32).monster_types {
                        monster_type_list.push(*monster_type);
                    }
                }
            }

            for starter in &bravery_data.starters {
                for monster_type in &monster!(*starter as u32).monster_types {
                    monster_type_list.push(*monster_type);
                }
            }

            if !monster_type_list.contains(&random_relic.monster_type_restriction) {
                return self.get_random_relic(area);
            }
        }

        random_relic.id
    }

    fn get_random_chest_in_area(&mut self, area: u32) -> (String, u32) {
        let data = &area!(area).area_data;

        let area_data_id = data[self.random.range_int(0, data.len() as i32) as usize];
        let area_data = GAME_DATA
            .area_data
            .iter()
            .find(|x| x.scene_id == area_data_id)
            .unwrap();

        (
            area_data.scene_name.to_owned(),
            area_data.chests[self.random.range_int(0, area_data.chests.len() as i32) as usize],
        )
    }
}
