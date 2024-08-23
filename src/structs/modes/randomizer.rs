use unity_random::Random;

use crate::{
    data::{
        macros::{ability, area, is_monster_in_area, monster},
        GAME_DATA,
    },
    structs::{
        map::{Area, MapArea},
        monster::{Ability, EMonster},
    },
};

pub struct RandomizerMode<'a> {
    random: &'a mut Random,

    pool: Vec<u32>,
    mapping: Vec<Option<u32>>,
}

impl<'a> RandomizerMode<'a> {
    pub fn get_mapping(random: &mut Random) -> Vec<Option<u32>> {
        let mut instance = RandomizerMode {
            random,

            pool: (4..110).collect(),
            //mapping: HashMap::new(),
            mapping: vec![None; 110],
        };

        while !instance.determine_mapping() {}

        instance.mapping
    }

    fn determine_mapping(&mut self) -> bool {
        self.pool = (4..110).collect();
        self.mapping.fill(None);

        let swimming_monster = GAME_DATA.swimming_monsters[self
            .random
            .range_int(0, GAME_DATA.swimming_monsters.len() as i32)
            as usize];

        self.pool.remove(
            self.pool
                .iter()
                .position(|r| r == &swimming_monster)
                .unwrap(),
        );
        //self.mapping.insert(EMonster::Koi as u32, swimming_monster);
        self.mapping[EMonster::Koi as usize] = Some(swimming_monster);

        // All other monsters
        for monster in 4..110 as u32 {
            if monster == EMonster::Koi as u32 {
                continue;
            }

            let randomizer_monster = self.determine_randomizer_monster(
                monster != EMonster::Tanuki as u32
                    && !is_monster_in_area!(Area::BlueCaves, &monster)
                    && !is_monster_in_area!(Area::MountainPath, &monster),
                monster != EMonster::Tanuki as u32
                    && !is_monster_in_area!(Area::BlueCaves, &monster)
                    && !is_monster_in_area!(Area::MountainPath, &monster)
                    && !is_monster_in_area!(Area::AncientWoods, &monster)
                    && !is_monster_in_area!(Area::StrongholdDungeon, &monster)
                    && !is_monster_in_area!(Area::SnowyPeaks, &monster)
                    && !is_monster_in_area!(Area::SunPalace, &monster)
                    && !is_monster_in_area!(Area::MagmaChamber, &monster)
                    && !is_monster_in_area!(Area::MysticalWorkshop, &monster),
            );

            self.pool.remove(
                self.pool
                    .iter()
                    .position(|r| r == &randomizer_monster)
                    .unwrap(),
            );
            //self.mapping.insert(monster, randomizer_monster);
            self.mapping[monster as usize] = Some(randomizer_monster);
        }

        if !self.has_randomizer_monsters_ability(
            Ability::Mount,
            &[
                Area::MountainPath as usize,
                Area::BlueCaves as usize,
                Area::StrongholdDungeon as usize,
                Area::AncientWoods as usize,
                Area::SnowyPeaks as usize,
                Area::SunPalace as usize,
            ],
        ) {
            return false;
        }

        if !self.has_randomizer_monsters_ability(
            Ability::MountOrFlying,
            &[
                Area::MountainPath as usize,
                Area::BlueCaves as usize,
                Area::StrongholdDungeon as usize,
                Area::AncientWoods as usize,
            ],
        ) {
            return false;
        }

        self.has_randomizer_monsters_ability(
            Ability::ImprovedFlying,
            &[
                Area::StrongholdDungeon as usize,
                Area::AncientWoods as usize,
                Area::SnowyPeaks as usize,
                Area::SunPalace as usize,
                Area::MagmaChamber as usize,
                Area::HorizonBeach as usize,
            ],
        ) && self.has_randomizer_secret_vision()
    }

    fn determine_randomizer_monster(
        &mut self,
        allow_improved_flying: bool,
        allow_swimming: bool,
    ) -> u32 {
        let mut monster: u32;

        loop {
            monster = self.pool[self.random.range_int(0, self.pool.len() as i32) as usize];

            if (allow_improved_flying
                || !ability!(Ability::ImprovedFlying)
                    .explore_actions
                    .contains(&monster!(monster).explore_action))
                && (allow_swimming || !GAME_DATA.swimming_monsters.contains(&monster))
            {
                break;
            }
        }

        monster
    }

    fn get_replacement_monster(&self, monster: &u32) -> u32 {
        self.mapping[*monster as usize].or(Some(*monster)).unwrap()
    }

    fn has_randomizer_monsters_ability(&self, ability: Ability, areas: &[usize]) -> bool {
        for area in areas {
            if self.has_monster_in_area_ability(&area!(*area), &ability, false) {
                return true;
            }
        }

        false
    }

    fn has_randomizer_secret_vision(&self) -> bool {
        for area in &GAME_DATA.areas {
            if area.id != Area::ForgottenWorld as u32
                && self.has_monster_in_area_ability(&area, &Ability::SecretVision, true)
            {
                return true;
            }
        }

        false
    }

    fn has_monster_in_area_ability(
        &self,
        area: &MapArea,
        ability: &Ability,
        full_monster_list: bool,
    ) -> bool {
        let monster_list = if full_monster_list {
            &area.monsters
        } else {
            &area.randomizer_check_list
        };

        match ability {
            Ability::Mount => {
                for monster in monster_list {
                    if self.has_randomizer_monster_ability(monster, Ability::Mount) {
                        return true;
                    }
                }
                false
            }

            Ability::MountOrFlying => {
                for monster in monster_list {
                    if self.has_randomizer_monster_ability(monster, Ability::Mount)
                        || self.has_randomizer_monster_ability(monster, Ability::Flying)
                    {
                        return true;
                    }
                }
                false
            }

            Ability::ImprovedFlying => {
                for monster in monster_list {
                    if self.has_randomizer_monster_ability(monster, Ability::ImprovedFlying) {
                        return true;
                    }
                }
                false
            }

            Ability::SecretVision => {
                for monster in monster_list {
                    if self.has_randomizer_monster_ability(monster, Ability::SecretVision) {
                        return true;
                    }
                }
                false
            }

            _ => false,
        }
    }

    fn has_randomizer_monster_ability(&self, monster: &u32, ability: Ability) -> bool {
        ability!(ability)
            .explore_actions
            .contains(&monster!(self.get_replacement_monster(monster)).explore_action)
    }
}
