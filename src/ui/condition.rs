use std::sync::{Arc, Mutex};

use slint::{
    Model, ModelRc, PhysicalPosition, PhysicalSize, SharedString, StandardListViewItem, VecModel,
};

use crate::{
    data::GAME_DATA,
    structs::monster::EMonster,
    ui::{
        dialog,
        enums::{condition::Condition, effect::Effect, value::Value},
    },
};

/// Adds a condition to the seed finder.
pub fn add_condition(
    conditions: Arc<Mutex<Vec<Condition>>>,
    rows: ModelRc<ModelRc<StandardListViewItem>>,
    value: Value,
    effect: Effect,
    is_randomizer: bool,
    is_bravery: bool,
    position: PhysicalPosition,
    size: PhysicalSize,
) {
    let condition = Condition::from((value, effect));
    let error = is_valid(&condition, is_randomizer, is_bravery);

    if let Some(err) = error {
        dialog::show_message(format!("Invalid condition: {}", err), position, size);
        return;
    }

    if let Some(model) = rows
        .as_any()
        .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
    {
        let condition_str = condition.to_string();
        let exists = model
            .iter()
            .any(|x| x.iter().nth(0).unwrap().text == condition_str);

        if !exists {
            let row = vec![
                StandardListViewItem::from(SharedString::from(condition_str)),
                StandardListViewItem::from(SharedString::from("x")),
            ];

            model.push(ModelRc::new(VecModel::from(row)));

            let mut conditions = conditions.lock().unwrap();
            conditions.push(condition);
        }
    }
}

/// Removes a condition from the seed finder.
pub fn remove_condition(
    conditions: Arc<Mutex<Vec<Condition>>>,
    rows: ModelRc<ModelRc<StandardListViewItem>>,
    row: i32,
) {
    if let Some(model) = rows
        .as_any()
        .downcast_ref::<VecModel<ModelRc<StandardListViewItem>>>()
    {
        model.remove(row as usize);

        let mut conditions = conditions.lock().unwrap();
        conditions.remove(row as usize);
    }
}

/// Initializes the values of the conditions' combobox.
pub fn init_values(
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
) -> ModelRc<StandardListViewItem> {
    let mut values = vec![];

    if is_randomizer || is_bravery {
        values.extend(
            GAME_DATA
                .monsters
                .iter()
                // Do not include Bard as it is an hard-coded reward
                .filter(|x| x.id != EMonster::Bard as u32)
                .map(|x| StandardListViewItem::from(SharedString::from(&x.name))),
        );
    }

    if is_relic {
        values.extend(
            GAME_DATA
                .relics
                .iter()
                .map(|x| StandardListViewItem::from(SharedString::from(&x.name))),
        );
    }

    if values.len() == 0 {
        values.push(StandardListViewItem::from(SharedString::from("")));
    }

    ModelRc::new(VecModel::from(values))
}

/// Initializes the effects of the conditions' combobox.
pub fn init_effects(
    is_randomizer: bool,
    is_bravery: bool,
    is_relic: bool,
) -> ModelRc<StandardListViewItem> {
    let mut effects = vec![];

    if is_relic || is_bravery {
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Available.to_string(),
        )));
    }

    if is_bravery {
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::BraveryChest.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Familiar.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Starter.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Swimming.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Bex.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Cryomancer.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::CryomancerRequired.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::EndOfTime.to_string(),
        )));
        effects.push(StandardListViewItem::from(SharedString::from(
            Effect::Army.to_string(),
        )));

        for i in 0..GAME_DATA.areas.len() as u32 {
            effects.push(StandardListViewItem::from(SharedString::from(
                Effect::EggInArea(i).to_string(),
            )));
        }

        if is_randomizer || is_relic {
            for i in 0..GAME_DATA.areas.len() as u32 {
                effects.push(StandardListViewItem::from(SharedString::from(
                    Effect::InArea(i).to_string(),
                )));
            }
        }

        if is_randomizer {
            // Do not include Bard as it is an hard-coded reward
            for i in 0..(GAME_DATA.monsters.len() as u32 - 1) {
                effects.push(StandardListViewItem::from(SharedString::from(
                    Effect::Replacement(i).to_string(),
                )));
            }
        }
    } else if is_randomizer {
        for i in 0..GAME_DATA.areas.len() as u32 {
            effects.push(StandardListViewItem::from(SharedString::from(
                Effect::InArea(i).to_string(),
            )));
        }

        for i in 0..GAME_DATA.monsters.len() as u32 {
            effects.push(StandardListViewItem::from(SharedString::from(
                Effect::Replacement(i).to_string(),
            )));
        }
    }

    if effects.len() == 0 {
        effects.push(StandardListViewItem::from(SharedString::from("")));
    }

    ModelRc::new(VecModel::from(effects))
}

fn is_valid(condition: &Condition, is_randomizer: bool, is_bravery: bool) -> Option<&str> {
    if let Condition::Invalid(err) = condition {
        return Some(err.as_str());
    }

    if let Condition::MonsterAvailable(_) = condition {
        if is_randomizer && !is_bravery {
            return Some("Every monster is available in Randomizer mode.");
        }
    }

    if let Condition::BraveryChest(id) = condition {
        if *id <= 3 {
            return Some("Spectral familiars cannot be obtained in Bravery area chests.");
        }
    }

    if let Condition::Swimming(id) = condition {
        if !GAME_DATA.swimming_monsters.contains(id) {
            return Some("This monster cannot be given at the Sun Palace.");
        }
    }

    if let Condition::Bex(id) = condition {
        if *id <= 3 {
            return Some("Spectral familiars cannot be obtained from Bex.");
        }
    }

    if let Condition::CryomancerRequired(id) = condition {
        if *id <= 3 {
            return Some("Spectral familiars cannot be wanted by the Cryomancer.");
        }
    }

    if let Condition::EggInArea(id, _) = condition {
        if *id <= 3 {
            return Some("Spectral familiars cannot be obtained in Bravery area chests.");
        }
    }

    if let Condition::MonsterInArea(id, _) = condition {
        if *id <= 3 {
            return Some("Spectral familiars cannot be found in any area.");
        }
    }

    if let Condition::Replacement(monster, replacement) = condition {
        if *monster <= 3 {
            return Some(
                "Spectral familiars cannot be replaced by any monster. You may be looking for the \"Eternity's End\" condition.",
            );
        }

        if *monster == *replacement {
            return Some("Cannot replace a monster with itself.");
        }
    }

    None
}
