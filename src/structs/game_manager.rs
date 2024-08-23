use unity_random::Random;

use crate::structs::{
    game::Game,
    modes::{bravery::BraveryMode, randomizer::RandomizerMode, relic::RelicMode},
};

pub fn generate_game(seed: u32, is_randomizer: bool, is_bravery: bool, is_relic: bool) -> Game {
    let mut random = Random::new();

    random.init_state(seed as i32);

    let mapping = if is_randomizer {
        Some(RandomizerMode::get_mapping(&mut random))
    } else {
        None
    };

    let bravery_data = if is_bravery {
        BraveryMode::get_monsters(&mut random, is_randomizer, &mapping)
    } else {
        None
    };

    Game {
        seed,
        is_randomizer,
        is_bravery,
        is_relic,
        // Only generate if the seed didn't fail to generate bravery data
        relics: if is_relic && (!is_bravery || (is_bravery && bravery_data.is_some())) {
            RelicMode::get_relics(&mut random, is_bravery, &bravery_data)
        } else {
            None
        },
        bravery_data,
        mapping,
    }
}
