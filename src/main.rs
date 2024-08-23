#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::{Arc, Mutex};

use slint::{ComponentHandle, Image, ModelRc, SharedString, StandardListViewItem, VecModel};

use sanctuary_seeder::{
    data::{DISPLAY, VERSION},
    structs::game::Game,
    ui::{enums::condition::Condition, request, types::AppWindow},
};

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;
    let ui_weak = ui.as_weak();

    let game: Arc<Mutex<Option<Game>>> = Arc::new(Mutex::new(None));
    let is_max_seed = Arc::new(Mutex::new(false));
    let conditions: Arc<Mutex<Vec<Condition>>> = Arc::new(Mutex::new(vec![]));

    // Initialize icons and models
    init(&ui);

    // Seed finder's game modes toggled on/off
    ui.on_req_find_modes(request::find_modes(ui_weak.clone(), conditions.clone()));

    // Find seed
    ui.on_req_find(request::find(ui_weak.clone(), conditions.clone()));

    // Condition added
    ui.on_req_add_condition(request::add_condition(ui_weak.clone(), conditions.clone()));

    // Condition removed
    ui.on_req_remove_condition(request::remove_condition(
        ui_weak.clone(),
        conditions.clone(),
    ));

    // Seed table row clicked
    ui.on_req_seed_result(request::seed_result(ui_weak.clone()));

    // Area selected
    ui.on_req_area(request::area(ui_weak.clone(), game.clone()));

    // Seed or game mode changed
    ui.on_req_seed(request::seed(
        ui_weak.clone(),
        game.clone(),
        is_max_seed.clone(),
    ));

    ui.run()
}

fn init(ui: &AppWindow) {
    ui.set_version(SharedString::from(VERSION));

    // Initialize images
    ui.set_champion_icon(Image::from_rgba8(
        DISPLAY.get_icon(&String::from("champion")),
    ));

    ui.set_egg_icon(Image::from_rgba8(DISPLAY.get_icon(&String::from("egg"))));

    ui.set_egg_light_icon(Image::from_rgba8(
        DISPLAY.get_icon(&String::from("egg_light")),
    ));

    ui.set_egg_dark_icon(Image::from_rgba8(
        DISPLAY.get_icon(&String::from("egg_dark")),
    ));

    // Initialize models
    ui.set_values(ModelRc::new(VecModel::from(vec![
        StandardListViewItem::from(SharedString::from("")),
    ])));
    ui.set_effects(ModelRc::new(VecModel::from(vec![
        StandardListViewItem::from(SharedString::from("")),
    ])));
    ui.set_conditions_display(ModelRc::new(VecModel::from(vec![])));
    ui.set_found_seeds(ModelRc::new(VecModel::from(vec![])));
}
