use serde::{Deserialize, Serialize};
use std::{fs, path::Path};

use map::{AreaData, MapArea};
use monster::{ExploreAbility, ExploreAction, Monster, MonsterType};
use relic::Relic;

mod atlas;
mod sql;

#[path = "../src/structs/map.rs"]
mod map;

#[path = "../src/structs/monster.rs"]
mod monster;

#[path = "../src/structs/relic.rs"]
mod relic;

fn main() {
    println!("cargo::rerun-if-changed=./res/json/");
    println!("cargo::rerun-if-changed=./res/icons/");

    if cfg!(target_os = "windows") {
        let mut res = winresource::WindowsResource::new();
        res.set_icon("./res/icons/app/Krakaturtle.ico");
        res.compile().unwrap();
    }

    for dir in ["data", "atlas", "tables"] {
        fs::create_dir_all(format!("./res/out/{dir}")).ok();
    }

    sql::export_sql_dir("./res/tables", "./res/out/tables").unwrap();

    to_rmp::<AreaData>("./res/json/AreaData.json");
    to_rmp::<ExploreAbility>("./res/json/ExploreAbilities.json");
    to_rmp::<ExploreAction>("./res/json/ExploreActions.json");
    to_rmp::<MapArea>("./res/json/MonsterAreas.json");
    to_rmp::<Monster>("./res/json/MonsterJournalList.json");
    to_rmp::<MonsterType>("./res/json/MonsterTypes.json");
    to_rmp::<Relic>("./res/json/Relics.json");
    to_rmp::<u32>("./res/json/SwimmingMonsterList.json");

    atlas::create_atlas(
        "./res/icons/monsters/",
        "./res/out/atlas/monsters.png",
        512,
        1024,
    );

    atlas::create_atlas_joined(
        vec!["./res/icons/relics/", "./res/icons/other/"],
        "./res/out/atlas/icons.png",
        128,
        128,
    );

    slint_build::compile("ui/appwindow.slint").unwrap();
}

fn to_rmp<T>(path: &str)
where
    for<'a> T: Serialize + Deserialize<'a>,
{
    if let Ok(text) = fs::read_to_string(path) {
        let object: Vec<T> = serde_json::from_str(&text).unwrap();
        let rmp: Vec<u8> = rmp_serde::to_vec(&object).unwrap();

        let output = Path::new(path).with_extension("dat");
        let output = output.file_name().unwrap().to_str().unwrap();

        fs::write(format!("./res/out/data/{}", output), &rmp).unwrap();
    }
}
