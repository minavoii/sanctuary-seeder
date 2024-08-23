use std::{
    sync::{Arc, Mutex},
    thread,
};

use slint::{ComponentHandle, Model, ModelRc, SharedString, StandardListViewItem, VecModel, Weak};

use crate::{
    structs::game::Game,
    ui::{
        condition,
        enums::{condition::Condition, effect::Effect, value::Value},
        finder, seed_info,
        types::AppWindow,
    },
};

pub fn find_modes(
    ui_weak: Weak<AppWindow>,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> impl FnMut(bool, bool, bool) {
    move |is_randomizer, is_bravery, is_relic| {
        let ui = ui_weak.unwrap();
        let conditions = conditions.clone();

        *conditions.lock().unwrap() = vec![];

        ui.set_conditions_display(ModelRc::new(VecModel::from(vec![])));
        ui.set_values(condition::init_values(is_randomizer, is_bravery, is_relic));
        ui.set_effects(condition::init_effects(is_randomizer, is_bravery, is_relic));
        ui.set_item1(0);
        ui.set_item2(0);
    }
}

pub fn find(
    ui_weak: Weak<AppWindow>,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> impl FnMut(bool, bool, bool) {
    move |is_randomizer, is_bravery, is_relic| {
        let ui = ui_weak.unwrap();
        let conditions = conditions.clone();

        finder::find(
            ui_weak.clone(),
            is_randomizer,
            is_bravery,
            is_relic,
            conditions,
            ui.window().position(),
            ui.window().size(),
        );
    }
}

pub fn add_condition(
    ui_weak: Weak<AppWindow>,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> impl FnMut(i32, i32) {
    move |item1, item2| {
        let ui = ui_weak.unwrap();
        let conditions = conditions.clone();

        let is_randomizer = ui.get_is_randomizer_finder();
        let is_bravery = ui.get_is_bravery_finder();
        let is_relic = ui.get_is_relic_finder();

        let value = Value::from((item1 as u32, is_randomizer, is_bravery));
        let effect = Effect::from((item2 as u32, is_randomizer, is_bravery, is_relic));

        condition::add_condition(
            conditions,
            ui.get_conditions_display(),
            value,
            effect,
            is_randomizer,
            is_bravery,
            ui.window().position(),
            ui.window().size(),
        );
    }
}

pub fn remove_condition(
    ui_weak: Weak<AppWindow>,
    conditions: Arc<Mutex<Vec<Condition>>>,
) -> impl FnMut(i32) {
    move |row| {
        let ui = ui_weak.unwrap();
        let conditions = conditions.clone();

        condition::remove_condition(conditions, ui.get_conditions_display(), row);
    }
}

pub fn seed_result(ui_weak: Weak<AppWindow>) -> impl FnMut(i32) {
    move |row_id| {
        let ui = ui_weak.unwrap();
        let rows = ui.get_found_seeds();

        if let Some(model) = rows
            .as_any()
            .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
        {
            let row = model.iter().nth(row_id as usize);

            if let Some(row) = row {
                let seed = row.row_data(0).unwrap().text;
                let is_bravery = ui.get_is_bravery_finder();

                ui.set_is_randomizer(ui.get_is_randomizer_finder());
                ui.set_is_bravery(is_bravery);
                ui.set_is_relic(ui.get_is_relic_finder());
                ui.set_seed(seed.clone());
                ui.invoke_req_seed(seed);
                ui.set_tab(if is_bravery { 1 } else { 0 });
            }
        }
    }
}

pub fn area(ui_weak: Weak<AppWindow>, game: Arc<Mutex<Option<Game>>>) -> impl FnMut(i32) {
    move |area_id| {
        let ui_weak = ui_weak.clone();
        let game = game.clone();

        thread::spawn(move || seed_info::update_area(ui_weak, game, area_id));
    }
}

pub fn seed(
    ui_weak: Weak<AppWindow>,
    game: Arc<Mutex<Option<Game>>>,
    is_max_seed: Arc<Mutex<bool>>,
) -> impl FnMut(SharedString) {
    move |seed_str| {
        let ui_weak = ui_weak.clone();
        let ui = ui_weak.unwrap();

        let game = game.clone();
        let is_max_seed = is_max_seed.clone();

        let is_randomizer = ui.get_is_randomizer();
        let is_bravery = ui.get_is_bravery();
        let is_relic = ui.get_is_relic();
        let area_id = ui.get_area();

        thread::spawn(move || {
            seed_info::update_displays(
                ui_weak,
                game,
                is_max_seed,
                seed_str,
                is_randomizer,
                is_bravery,
                is_relic,
                area_id,
            )
        });
    }
}
