use std::sync::{Arc, Mutex};

use slint::{ComponentHandle, Image, SharedPixelBuffer, SharedString, Weak};

use crate::{
    data::{DISPLAY, GAME_DATA},
    structs::{game::Game, game_manager, modes::bravery::Shift, monster::EMonster},
    ui::{
        dialog,
        types::{AppWindow, MonsterDisplayInfo, RelicDisplayInfo},
    },
};

/// Updates all monster and relic displays.
pub fn update_displays(
    ui_weak: Weak<AppWindow>,
    game: Arc<Mutex<Option<Game>>>,
    is_max_seed: Arc<Mutex<bool>>,
    mut seed_str: SharedString,
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
    area_id: i32,
) {
    let ui_weak = ui_weak.clone();

    let mut game = game.lock().unwrap();
    let mut is_max_seed = is_max_seed.lock().unwrap();

    if seed_str.is_empty() {
        *game = None;
        *is_max_seed = false;
        clear_displays(&ui_weak, true, true, true);
        return;
    }

    // Maximum: 999 999
    if seed_str.len() > 6 {
        seed_str = SharedString::from("999999");

        let seed_str = seed_str.clone();
        ui_weak
            .upgrade_in_event_loop(move |ui| ui.set_seed(seed_str.to_owned()))
            .unwrap();

        // Prevents generating the same game over and over again
        if is_max_seed.to_owned() == true {
            return;
        }
    }

    // Check if seed is a valid u32
    if let Ok(seed) = seed_str.parse::<u32>() {
        *is_max_seed = seed == 999999;

        if !is_randomizer && !is_bravery && !is_relic {
            *game = None;
            clear_displays(&ui_weak, true, true, true);
            return;
        }

        let new_game = game_manager::generate_game(seed, is_randomizer, is_bravery, is_relic);

        if new_game.is_bad_seed() {
            *game = None;
            clear_displays(&ui_weak, true, true, true);

            ui_weak
                .upgrade_in_event_loop(move |ui| {
                    dialog::show_message(
                        format!("Error: Seed {seed} is invalid in Randomizer + Bravery modes. The game will fail to generate this seed, and it cannot be used."),
                        ui.window().position(),
                        ui.window().size(),
                    )
                })
                .unwrap();
            return;
        }

        // Don't update if we call `update_eggs_ui` and set the area to 0
        let mut should_update_relic = area_id != 0;

        if is_randomizer {
            // Area selected
            if area_id != 0 {
                update_randomizer_ui(&ui_weak, &new_game, (area_id - 1) as u32);
            }
            // Bravery eggs
            else if is_bravery {
                update_eggs_ui(&ui_weak, &new_game);
            }
        }

        if is_bravery {
            update_bravery_ui(&ui_weak, &new_game);

            // Bravery eggs
            if !is_randomizer {
                should_update_relic = false;
                update_eggs_ui(&ui_weak, &new_game);
            }
        }

        // Don't display relics alongside the bravery eggs display
        if is_relic && should_update_relic {
            update_relics_ui(&ui_weak, &new_game, (area_id - 1) as u32);
        }

        clear_displays(
            &ui_weak, // Bravery eggs but not in bravery mode
            !is_bravery && area_id == 0,
            !is_bravery,
            !is_relic || !should_update_relic,
        );

        *game = Some(new_game);
    }
    // The user could copy/paste invalid values
    else {
        *game = None;
        *is_max_seed = false;
        clear_displays(&ui_weak, true, true, true);

        ui_weak
            .upgrade_in_event_loop(move |ui| ui.set_seed(SharedString::from("")))
            .unwrap();
    }
}

/// Updates all the area displays.
pub fn update_area(ui_weak: Weak<AppWindow>, game: Arc<Mutex<Option<Game>>>, area_id: i32) {
    let ui_weak = ui_weak.clone();
    let game = game.lock().unwrap();

    if let Some(game) = &*game {
        // Bravery eggs display
        if area_id == 0 {
            if game.is_bravery {
                update_eggs_ui(&ui_weak, game);
                clear_displays(&ui_weak, false, false, true);
            } else {
                clear_displays(&ui_weak, true, false, true);
            }
        }
        // Area selected
        else {
            update_randomizer_ui(&ui_weak, game, (area_id - 1) as u32);

            if game.is_relic {
                update_relics_ui(&ui_weak, game, (area_id - 1) as u32);
            }
        }
    }
}

/// Updates the randomizer mode monster displays.
pub fn update_randomizer_ui(ui_weak: &Weak<AppWindow>, game: &Game, area_id: u32) {
    if let Some(mapping) = &game.mapping {
        let tanuki = DISPLAY.get_monster(
            mapping[EMonster::Tanuki as usize].unwrap(),
            Some(EMonster::Tanuki as u32),
            false,
            false,
            Shift::Normal,
        );
        let mut displays = DISPLAY.get_by_area(mapping, &game.bravery_data, area_id);

        ui_weak
            .upgrade_in_event_loop(move |ui| {
                ui.set_tanuki(tanuki.into());
                ui.set_area_monster14(displays.remove(13).into());
                ui.set_area_monster13(displays.remove(12).into());
                ui.set_area_monster12(displays.remove(11).into());
                ui.set_area_monster11(displays.remove(10).into());
                ui.set_area_monster10(displays.remove(9).into());
                ui.set_area_monster9(displays.remove(8).into());
                ui.set_area_monster8(displays.remove(7).into());
                ui.set_area_monster7(displays.remove(6).into());
                ui.set_area_monster6(displays.remove(5).into());
                ui.set_area_monster5(displays.remove(4).into());
                ui.set_area_monster4(displays.remove(3).into());
                ui.set_area_monster3(displays.remove(2).into());
                ui.set_area_monster2(displays.remove(1).into());
                ui.set_area_monster1(displays.remove(0).into());
            })
            .unwrap();
    } else {
        clear_displays(&ui_weak, true, false, false);
    }
}

/// Updates the `Bravery Eggs` monster displays.
pub fn update_eggs_ui(ui_weak: &Weak<AppWindow>, game: &Game) {
    if let Some(bravery) = &game.bravery_data {
        let mut displays = DISPLAY.get_bravery(bravery);

        ui_weak
            .upgrade_in_event_loop(move |ui| {
                ui.set_tanuki(DISPLAY.get_monster_empty().into());
                ui.set_area_monster13(displays.eggs.remove(12).into());
                ui.set_area_monster12(displays.eggs.remove(11).into());
                ui.set_area_monster11(displays.eggs.remove(10).into());
                ui.set_area_monster10(displays.eggs.remove(9).into());
                ui.set_area_monster9(displays.eggs.remove(8).into());
                ui.set_area_monster8(displays.eggs.remove(7).into());
                ui.set_area_monster7(displays.eggs.remove(6).into());
                ui.set_area_monster6(displays.eggs.remove(5).into());
                ui.set_area_monster5(displays.eggs.remove(4).into());
                ui.set_area_monster4(displays.eggs.remove(3).into());
                ui.set_area_monster3(displays.eggs.remove(2).into());
                ui.set_area_monster2(displays.eggs.remove(1).into());
                ui.set_area_monster1(displays.eggs.remove(0).into());
                ui.set_area(0);
            })
            .unwrap();
    }
}

/// Updates the Bravery mode monster displays.
pub fn update_bravery_ui(ui_weak: &Weak<AppWindow>, game: &Game) {
    if let Some(bravery) = &game.bravery_data {
        let mut displays = DISPLAY.get_bravery(bravery);

        ui_weak
            .upgrade_in_event_loop(move |ui| {
                ui.set_familiar(displays.familiar.into());
                ui.set_starter2(displays.starters.remove(2).into());
                ui.set_starter1(displays.starters.remove(1).into());
                ui.set_swimming(displays.swimming.into());

                ui.set_cryomancer(displays.cryomancer.into());
                ui.set_cryomancer_required(displays.cryomancer_required.into());
                ui.set_bex(displays.bex.into());

                ui.set_end_of_time3(displays.end_of_time.remove(2).into());
                ui.set_end_of_time2(displays.end_of_time.remove(1).into());
                ui.set_end_of_time1(displays.end_of_time.remove(0).into());

                ui.set_army7(if displays.army.len() == 7 {
                    displays.army.remove(6).into()
                } else {
                    DISPLAY.get_monster_empty().into()
                });
                ui.set_army6(displays.army.remove(5).into());
                ui.set_army5(displays.army.remove(4).into());
                ui.set_army4(displays.army.remove(3).into());
                ui.set_army3(displays.army.remove(2).into());
                ui.set_army2(displays.army.remove(1).into());
                ui.set_army1(displays.army.remove(0).into());
            })
            .unwrap();
    }
}

/// Updates the relic display.
pub fn update_relics_ui(ui_weak: &Weak<AppWindow>, game: &Game, area_id: u32) {
    if let Some(relics) = &game.relics {
        let id = relics.list[area_id as usize];
        let name = &GAME_DATA.relics.iter().find(|x| x.id == id).unwrap().name;

        ui_weak
            .upgrade_in_event_loop(move |ui| {
                ui.set_relic(RelicDisplayInfo {
                    name: SharedString::from(name),
                    sprite: Image::from_rgba8(DISPLAY.get_icon(name)),
                })
            })
            .unwrap();
    }
}

/// Clears the specified monster displays and/or the relic display.
pub fn clear_displays(ui_weak: &Weak<AppWindow>, randomizer: bool, bravery: bool, relic: bool) {
    ui_weak
        .upgrade_in_event_loop(move |ui| {
            let empty_display = MonsterDisplayInfo::from(DISPLAY.get_monster_empty());

            if randomizer {
                ui.set_tanuki(empty_display.clone());
                ui.set_area_monster1(empty_display.clone());
                ui.set_area_monster2(empty_display.clone());
                ui.set_area_monster3(empty_display.clone());
                ui.set_area_monster4(empty_display.clone());
                ui.set_area_monster5(empty_display.clone());
                ui.set_area_monster6(empty_display.clone());
                ui.set_area_monster7(empty_display.clone());
                ui.set_area_monster8(empty_display.clone());
                ui.set_area_monster9(empty_display.clone());
                ui.set_area_monster10(empty_display.clone());
                ui.set_area_monster11(empty_display.clone());
                ui.set_area_monster12(empty_display.clone());
                ui.set_area_monster13(empty_display.clone());
                ui.set_area_monster14(empty_display.clone());
            }

            if bravery {
                ui.set_familiar(empty_display.clone());
                ui.set_starter1(empty_display.clone());
                ui.set_starter2(empty_display.clone());
                ui.set_swimming(empty_display.clone());
                ui.set_cryomancer_required(empty_display.clone());
                ui.set_cryomancer(empty_display.clone());
                ui.set_bex(empty_display.clone());
                ui.set_end_of_time1(empty_display.clone());
                ui.set_end_of_time2(empty_display.clone());
                ui.set_end_of_time3(empty_display.clone());
                ui.set_army1(empty_display.clone());
                ui.set_army2(empty_display.clone());
                ui.set_army3(empty_display.clone());
                ui.set_army4(empty_display.clone());
                ui.set_army5(empty_display.clone());
                ui.set_army6(empty_display.clone());
                ui.set_army7(empty_display.clone());
            }

            if relic {
                ui.set_relic(RelicDisplayInfo {
                    name: SharedString::from(""),
                    sprite: Image::from_rgba8(SharedPixelBuffer::new(1, 1)),
                });
            }
        })
        .unwrap();
}
