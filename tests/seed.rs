use std::{
    fs::{self, File},
    path::Path,
};

use sanctuary_seeder::structs::{game::Game, game_manager};

#[test]
fn seed() {
    let mut games = vec![];

    for dir in ["all_modes", "bravery_relic", "randomizer_relic", "relic"] {
        let dir_path = Path::new("./tests/seeds/").join(dir);

        for entry in fs::read_dir(dir_path).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();

            let game: Game = serde_json::from_reader(File::open(path).unwrap()).unwrap();

            games.push(game);
        }
    }

    for game in games {
        let new_game = game_manager::generate_game(
            game.seed,
            game.is_randomizer,
            game.is_bravery,
            game.is_relic,
        );

        // Bad seeds - the game fails to generate, no need to check anything else
        // Seed 32410 is one of them.
        if game.is_bad_seed() {
            assert!(new_game.is_bad_seed());
        } else {
            assert_eq!(game.mapping, new_game.mapping);
            assert_eq!(game.bravery_data, new_game.bravery_data);
            assert_eq!(game.relics, new_game.relics);
        }
    }
}
