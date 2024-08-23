use std::sync::LazyLock;

use crate::{structs::game_data::GameData, ui::display::Display};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

pub static GAME_DATA: LazyLock<GameData> = LazyLock::new(|| GameData::new());
pub static DISPLAY: LazyLock<Display> = LazyLock::new(|| Display::new(&GAME_DATA));

pub mod macros {
    macro_rules! monster {
        ($monster:expr) => {
            crate::data::GAME_DATA.monsters[$monster as usize]
        };
    }

    macro_rules! area {
        ($area:expr) => {
            crate::data::GAME_DATA.areas[$area as usize]
        };
    }

    macro_rules! relic {
        ($relic:expr) => {
            crate::data::GAME_DATA.relics[$relic as usize]
        };
    }

    macro_rules! ability {
        ($ability:expr) => {
            crate::data::GAME_DATA.abilities[$ability as usize]
        };
    }

    macro_rules! is_monster_in_area {
        ($area:expr, $monster:expr) => {
            crate::data::GAME_DATA.areas[$area as usize]
                .monsters
                .contains($monster)
        };
    }

    macro_rules! load_data {
        ($path:expr, $type:ty) => {
            rmp_serde::from_slice::<$type>(std::include_bytes!($path)).unwrap()
        };
    }

    pub(crate) use {ability, area, is_monster_in_area, load_data, monster, relic};
}
