use serde::{Deserialize, Serialize};
use unity_random::Random;

use crate::{
    data::{
        macros::{ability, monster},
        GAME_DATA,
    },
    structs::{
        map::Area,
        monster::{Ability, EMonster},
    },
};

macro_rules! has_starter_ability {
    ($self:expr, $ability:expr, $exclude:expr, false) => {
        for i in 1..3 {
            if $self.has_ability($ability, $self.monsters.starters[i], $exclude) {
                return true;
            }
        }
    };
    ($self:expr, $ability:expr, $exclude:expr, true) => {
        for i in 0..3 {
            if $self.has_ability($ability, $self.monsters.starters[i], $exclude) {
                return true;
            }
        }
    };
}

macro_rules! has_egg_ability {
    ($self:expr, $ability:expr, $areas:expr, $exclude:expr) => {
        for area in $areas {
            if $self.has_ability($ability, $self.monsters.eggs[area as usize], $exclude) {
                return true;
            }
        }
    };
}

#[derive(Clone, Copy)]
pub enum Shift {
    Normal = 0,
    Light = 1,
    Dark = 2,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BraveryData {
    pub shift_offset: u32,

    pub familiar: u32,
    pub swimming: u32,
    pub bex: u32,
    pub cryomancer: Option<u32>,
    pub cryomancer_required: u32,

    pub starters: Vec<u32>,
    pub eggs: Vec<u32>,
    pub end_of_time: Vec<u32>,
    pub army: Vec<Option<u32>>,
}

impl PartialEq for BraveryData {
    fn eq(&self, other: &Self) -> bool {
        self.shift_offset == other.shift_offset
            && self.familiar == other.familiar
            && self.swimming == other.swimming
            && self.bex == other.bex
            && self.cryomancer == other.cryomancer
            && self.cryomancer_required == other.cryomancer_required
            && self.starters == other.starters
            && self.eggs == other.eggs
            && self.end_of_time == other.end_of_time
            && self.army == other.army
    }
}

impl BraveryData {
    const SHIFT_DATA: [Shift; 32] = [
        Shift::Dark,
        Shift::Normal,
        Shift::Dark,
        Shift::Light,
        Shift::Normal,
        Shift::Normal,
        Shift::Dark,
        Shift::Dark,
        Shift::Normal,
        Shift::Light,
        Shift::Normal,
        Shift::Light,
        Shift::Normal,
        Shift::Dark,
        Shift::Normal,
        Shift::Normal,
        Shift::Dark,
        Shift::Light,
        Shift::Normal,
        Shift::Normal,
        Shift::Normal,
        Shift::Light,
        Shift::Light,
        Shift::Normal,
        Shift::Dark,
        Shift::Normal,
        Shift::Normal,
        Shift::Light,
        Shift::Normal,
        Shift::Light,
        Shift::Dark,
        Shift::Normal,
    ];

    pub fn get_area_eggs_shift(&self) -> Vec<Shift> {
        self.eggs
            .iter()
            .enumerate()
            .map(|(i, _)| {
                BraveryData::SHIFT_DATA[(i as u32 + self.shift_offset)
                    .rem_euclid(BraveryData::SHIFT_DATA.len() as u32)
                    as usize]
            })
            .collect::<Vec<Shift>>()
    }

    pub fn get_army_eggs_shift(&self) -> Vec<Shift> {
        self.army
            .iter()
            .enumerate()
            .map(|(i, _)| {
                BraveryData::SHIFT_DATA[(i as u32 + self.shift_offset)
                    .rem_euclid(BraveryData::SHIFT_DATA.len() as u32)
                    as usize]
            })
            .collect::<Vec<Shift>>()
    }
}

pub struct BraveryMode<'a> {
    random: &'a mut Random,

    is_randomizer: bool,
    mapping: &'a Option<Vec<Option<u32>>>,

    monsters: BraveryData,
}

impl<'a> BraveryMode<'a> {
    pub fn get_monsters(
        random: &mut Random,
        is_randomizer: bool,
        mapping: &Option<Vec<Option<u32>>>,
    ) -> Option<BraveryData> {
        let mut instance = BraveryMode {
            random,

            is_randomizer,
            mapping,

            monsters: BraveryData {
                shift_offset: 0,

                familiar: 0,
                swimming: EMonster::Koi as u32,
                bex: 0,
                cryomancer: None,
                cryomancer_required: 0,

                starters: vec![],
                eggs: vec![],
                end_of_time: vec![],
                army: vec![],
            },
        };

        instance.monsters.swimming = GAME_DATA.swimming_monsters[instance
            .random
            .range_int(0, GAME_DATA.swimming_monsters.len() as i32)
            as usize];

        instance.monsters.bex = instance.determine_random_monster(true, false, false);
        instance.determine_starters();

        let mut tries = 0;

        while !instance.determine_eggs() {
            tries += 1;

            if tries % 100 == 0 {
                instance.determine_starters();
            }

            // Some seeds cannot generate, and will freeze the game upon creation
            // The algorithm is not able to generate randomizer and/or bravery monsters
            // and will try forever
            if tries > 10000 {
                return None;
            }
        }

        instance.monsters.cryomancer = Some(instance.determine_random_monster(true, false, true));
        instance.monsters.cryomancer_required = instance.determine_cryomancer_required();
        instance.determine_army();

        for _ in 0..3 {
            let monster = instance.determine_random_monster(true, true, true);
            instance.monsters.end_of_time.push(monster);
        }

        instance.monsters.shift_offset = instance.random.range_int(0, 1000) as u32;

        Some(instance.monsters)
    }

    fn determine_random_monster(
        &mut self,
        allow_improved_flying: bool,
        allow_swimming: bool,
        allow_familiar: bool,
    ) -> u32 {
        let monster = self.random.range_int(
            if allow_familiar { 0 } else { 4 },
            GAME_DATA.monsters.len() as i32 - 1,
        ) as u32;

        if (!allow_improved_flying && self.has_ability(Ability::ImprovedFlying, monster, None))
            || (!allow_swimming && GAME_DATA.swimming_monsters.contains(&monster))
            || self.was_monster_already_determined(monster)
        {
            return self.determine_random_monster(
                allow_improved_flying,
                allow_swimming,
                allow_familiar,
            );
        } else {
            monster
        }
    }

    fn was_monster_already_determined(&self, monster: u32) -> bool {
        if monster == self.monsters.familiar
            || monster == self.monsters.swimming
            || self.monsters.cryomancer.is_some_and(|x| x == monster)
            || monster == self.monsters.bex
            || self.monsters.eggs.contains(&monster)
            || self.monsters.end_of_time.contains(&monster)
            || self.monsters.army.contains(&Some(monster))
        {
            return true;
        }

        for i in 0..3 {
            if self.monsters.starters.len() > i && self.monsters.starters[i] == monster {
                return true;
            }
        }

        false
    }

    fn has_ability(&self, ability: Ability, monster: u32, exclude: Option<u32>) -> bool {
        !exclude.is_some_and(|f| f == monster)
            && ability!(ability)
                .explore_actions
                .contains(&monster!(monster).explore_action)
    }

    fn determine_starters(&mut self) {
        self.monsters.starters.clear();

        self.monsters.familiar = self.random.range_int(0, 4) as u32;
        self.monsters.starters.push(self.monsters.familiar);

        for _ in 0..2 {
            let monster = self.determine_random_monster(false, false, false);
            self.monsters.starters.push(monster);

            // AddMonsterByPrefab() makes a call to UnityEngine.Object.Instantiate
            //   which calls Object.Internal_CloneSingle
            //   and may generate another number with UnityEngine.Random.Range(int, int)
            // This is likely due to the flying animation frames, and is always consistent
            if monster == EMonster::Vaero as u32
                || monster == EMonster::Frosty as u32
                || monster == EMonster::MadEye as u32
                || monster == EMonster::Nightwing as u32
                || monster == EMonster::Toxiquus as u32
                || monster == EMonster::Magmamoth as u32
                || monster == EMonster::Glowfly as u32
                || monster == EMonster::Raduga as u32
                || monster == EMonster::Kanko as u32
                || monster == EMonster::Glowdra as u32
                || monster == EMonster::Draconov as u32
                || monster == EMonster::Thanatos as u32
                || monster == EMonster::Vertraag as u32
                || monster == EMonster::Ascendant as u32
                || monster == EMonster::Amberlagna as u32
            {
                // self.random.skip(1);
                self.random.value();
            }
        }
    }

    fn determine_eggs(&mut self) -> bool {
        self.monsters.eggs.clear();

        for area in &GAME_DATA.areas {
            let mut egg = 0;
            let mut best_rating = -1.0;

            for monster in &area.monsters {
                let replacement = self.get_replacement_monster(monster);

                if !self.was_monster_already_determined(replacement) {
                    let rating = self.random.range_float(0.0, 1.0);

                    if rating > best_rating {
                        egg = replacement;
                        best_rating = rating;
                    }
                }
            }

            if (
                egg == 0
                    || (!self.was_monster_already_determined(EMonster::Tanuki as u32)
                        && self.random.range_float(0.0, 1.0) < 0.100_000_001_490_116_12)
                // = 1f
            ) && (egg == 0 || self.random.range_float(0.0, 1.0) > best_rating)
            {
                egg = EMonster::Tanuki as u32;
            }

            self.monsters.eggs.push(egg);
        }

        self.has_breakwall_monster(None)
            && self.has_mount_monster(None)
            && self.has_mount_or_flying_monster(None)
            && self.has_improved_flying_monster(None)
            && self.has_secret_vision_monster(None)
    }

    fn determine_cryomancer_required(&mut self) -> u32 {
        let mut best_monster: u32 = 0;
        let mut best_rating: f32 = -1.;

        for egg in self.monsters.eggs.clone() {
            if let Some(res) = self.check_cryomancer_required(egg, best_rating) {
                (best_monster, best_rating) = res;
            }
        }

        for i in 1..3 {
            if let Some(res) =
                self.check_cryomancer_required(self.monsters.starters[i], best_rating)
            {
                (best_monster, best_rating) = res;
            }
        }

        if let Some((monster, _)) = self.check_cryomancer_required(self.monsters.bex, best_rating) {
            best_monster = monster;
        }

        best_monster
    }

    fn check_cryomancer_required(&mut self, monster: u32, best_rating: f32) -> Option<(u32, f32)> {
        if (self.has_ability(Ability::BreakWall, monster, None)
            && !self.has_breakwall_monster(Some(monster)))
            || (self.has_ability(Ability::ImprovedFlying, monster, None)
                && !self.has_breakwall_monster(Some(monster)))
            || (self.has_ability(Ability::SecretVision, monster, None)
                && !self.has_secret_vision_monster(Some(monster)))
            || (self.has_ability(Ability::Mount, monster, None)
                && !self.has_mount_monster(Some(monster)))
        {
            return None;
        }

        let rating = self.random.range_float(0., 1.);

        if rating <= best_rating as f32 {
            return None;
        }

        let best_monster = monster;
        let best_rating = rating;

        Some((best_monster, best_rating))
    }

    fn determine_army(&mut self) {
        self.determine_army_monster(Ability::Ignite);
        self.determine_army_monster(Ability::Light);
        self.determine_army_monster(Ability::Crush);
        self.determine_army_monster(Ability::BigRock);
        self.determine_army_monster(Ability::Grappling);
        self.determine_army_monster(Ability::BlobForm);
        self.determine_army_monster(Ability::Levitate);
    }

    fn determine_army_monster(&mut self, ability: Ability) {
        if self.has_endgame_ability(ability) {
            let monster = self.determine_random_monster(true, false, true);
            self.monsters.army.push(Some(monster));
            return;
        }

        let mut monster = None;
        let mut best_rating = -1.;

        for i in 4..110 {
            if ability!(ability)
                .explore_actions
                .contains(&monster!(i).explore_action)
                && !self.was_monster_already_determined(i)
            {
                let rating = self.random.range_float(0., 1.);

                if rating > best_rating {
                    monster = Some(i);
                    best_rating = rating;
                }
            }
        }

        if monster.is_some() {
            self.monsters.army.push(monster);
        }
    }

    fn get_replacement_monster(&self, monster: &u32) -> u32 {
        if self.is_randomizer {
            if let Some(id) = self.mapping.as_ref().unwrap()[*monster as usize] {
                return id;
            }
        }

        *monster
    }

    fn has_endgame_ability(&self, ability: Ability) -> bool {
        for i in 1..3 {
            if self.has_ability(ability, self.monsters.starters[i], None)
                && self.monsters.starters[i] != self.monsters.cryomancer_required
            {
                return true;
            }
        }

        for (i, egg) in self.monsters.eggs.iter().enumerate() {
            if i != Area::ForgottenWorld as usize
                && self.has_ability(ability, *egg, None)
                && *egg != self.monsters.cryomancer_required
            {
                return true;
            }
        }

        self.has_ability(ability, self.monsters.bex, None)
            && self.monsters.bex != self.monsters.cryomancer_required
    }

    fn has_breakwall_monster(&self, exclude: Option<u32>) -> bool {
        has_starter_ability!(self, Ability::BreakWall, exclude, true);

        has_egg_ability!(
            self,
            Ability::BreakWall,
            [Area::BlueCaves, Area::MountainPath],
            exclude
        );

        false
    }

    fn has_mount_monster(&self, exclude: Option<u32>) -> bool {
        has_starter_ability!(self, Ability::Mount, exclude, false);

        has_egg_ability!(
            self,
            Ability::Mount,
            [
                Area::BlueCaves,
                Area::MountainPath,
                Area::StrongholdDungeon,
                Area::AncientWoods,
                Area::SnowyPeaks,
                Area::SunPalace
            ],
            exclude
        );

        false
    }

    fn has_mount_or_flying_monster(&self, exclude: Option<u32>) -> bool {
        has_starter_ability!(self, Ability::Mount, exclude, false);
        has_starter_ability!(self, Ability::Flying, exclude, true);

        has_egg_ability!(
            self,
            Ability::Mount,
            [
                Area::BlueCaves,
                Area::MountainPath,
                Area::StrongholdDungeon,
                Area::AncientWoods
            ],
            exclude
        );

        has_egg_ability!(
            self,
            Ability::Flying,
            [
                Area::BlueCaves,
                Area::MountainPath,
                Area::StrongholdDungeon,
                Area::AncientWoods
            ],
            exclude
        );

        false
    }

    fn has_improved_flying_monster(&self, exclude: Option<u32>) -> bool {
        has_egg_ability!(
            self,
            Ability::ImprovedFlying,
            [
                Area::StrongholdDungeon,
                Area::AncientWoods,
                Area::SnowyPeaks,
                Area::SunPalace,
                Area::HorizonBeach,
                Area::MagmaChamber
            ],
            exclude
        );

        false
    }

    fn has_secret_vision_monster(&self, exclude: Option<u32>) -> bool {
        has_starter_ability!(self, Ability::SecretVision, exclude, false);

        for (i, egg) in self.monsters.eggs.iter().enumerate() {
            if i != Area::ForgottenWorld as usize
                && self.has_ability(Ability::SecretVision, *egg, exclude)
            {
                return true;
            }
        }

        self.has_ability(Ability::SecretVision, self.monsters.bex, exclude)
    }
}
