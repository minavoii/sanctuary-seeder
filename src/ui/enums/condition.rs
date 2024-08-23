use std::cell::LazyCell;

use crate::{
    data::{
        macros::{monster, relic},
        GAME_DATA,
    },
    ui::enums::{effect::Effect, value::Value},
};

const AREA_COLUMNS: LazyCell<Vec<String>> = LazyCell::new(|| {
    GAME_DATA
        .areas
        .iter()
        .map(|x| x.name.replace(" ", ""))
        .collect::<Vec<String>>()
});

const RELIC_COLUMNS: LazyCell<Vec<String>> = LazyCell::new(|| {
    AREA_COLUMNS
        .iter()
        .map(|x| String::from("Relic.") + x)
        .collect::<Vec<String>>()
});

const BRAVERY_COLUMNS: LazyCell<Vec<String>> = LazyCell::new(|| {
    vec![
        "Familiar",
        "Start1",
        "Start2",
        "Swimming",
        "Bex",
        "Cryomancer",
        "CryomancerRequired",
        "EndOfTime1",
        "EndOfTime2",
        "EndOfTime3",
        "Army1",
        "Army2",
        "Army3",
        "Army4",
        "Army5",
        "Army6",
        "Army7",
    ]
    .iter()
    .map(|x| String::from(*x))
    .chain(AREA_COLUMNS.iter().map(|x| String::from("Bravery.") + x))
    .collect::<Vec<String>>()
});

/// A condition for the seed finder, to use in the SQL query.
pub enum Condition {
    Invalid(String),
    MonsterAvailable(u32),
    RelicAvailable(u32),
    BraveryChest(u32),
    Familiar(u32),
    Starter(u32),
    Swimming(u32),
    Bex(u32),
    CryomancerRequired(u32),
    Cryomancer(u32),
    Army(u32),
    EndOfTime(u32),
    MonsterInArea(u32, u32),
    EggInArea(u32, u32),
    RelicInArea(u32, u32),
    Replacement(u32, u32),
}

impl From<(Value, Effect)> for Condition {
    fn from((value, effect): (Value, Effect)) -> Self {
        match value {
            Value::Monster(monster) => match effect {
                Effect::None => Condition::Invalid(String::from(
                    "Could not determine condition: no effect found.",
                )),
                Effect::Available => Condition::MonsterAvailable(monster),
                Effect::BraveryChest => Condition::BraveryChest(monster),
                Effect::Familiar => Condition::Familiar(monster),
                Effect::Starter => {
                    if monster <= 3 {
                        Condition::Familiar(monster)
                    } else {
                        Condition::Starter(monster)
                    }
                }
                Effect::Swimming => Condition::Swimming(monster),
                Effect::Bex => Condition::Bex(monster),
                Effect::Cryomancer => Condition::Cryomancer(monster),
                Effect::CryomancerRequired => Condition::CryomancerRequired(monster),
                Effect::EndOfTime => Condition::EndOfTime(monster),
                Effect::Army => Condition::Army(monster),
                Effect::InArea(area) => Condition::MonsterInArea(monster, area),
                Effect::EggInArea(area) => Condition::EggInArea(monster, area),
                Effect::Replacement(replacement) => Condition::Replacement(monster, replacement),
            },
            Value::Relic(relic) => match effect {
                Effect::Available => Condition::RelicAvailable(relic),
                Effect::InArea(area) => Condition::RelicInArea(relic, area),
                _ => Condition::Invalid(String::from("Cannot use this condition for a Relic.")),
            },
        }
    }
}

impl ToString for Condition {
    fn to_string(&self) -> String {
        match self {
            Condition::Invalid(error) => error.to_owned(),
            Condition::MonsterAvailable(monster) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::Available.to_string()
                )
            }
            Condition::RelicAvailable(relic) => {
                format!("{} {}", relic!(*relic).name, Effect::Available.to_string())
            }
            Condition::BraveryChest(monster) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::BraveryChest.to_string()
                )
            }
            Condition::Familiar(monster) => format!(
                "{} {}",
                monster!(*monster).name,
                Effect::Familiar.to_string()
            ),
            Condition::Starter(monster) => format!(
                "{} {}",
                monster!(*monster).name,
                Effect::Starter.to_string()
            ),
            Condition::Swimming(monster) => format!(
                "{} {}",
                monster!(*monster).name,
                Effect::Swimming.to_string()
            ),
            Condition::Bex(monster) => {
                format!("{} {}", monster!(*monster).name, Effect::Bex.to_string())
            }
            Condition::Cryomancer(monster) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::Cryomancer.to_string()
                )
            }
            Condition::CryomancerRequired(monster) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::CryomancerRequired.to_string()
                )
            }
            Condition::Army(monster) => {
                format!("{} {}", monster!(*monster).name, Effect::Army.to_string())
            }
            Condition::EndOfTime(monster) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::EndOfTime.to_string()
                )
            }
            Condition::MonsterInArea(monster, area) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::InArea(*area).to_string()
                )
            }
            Condition::EggInArea(monster, area) => {
                format!(
                    "{} {}",
                    monster!(*monster).name,
                    Effect::EggInArea(*area).to_string()
                )
            }
            Condition::RelicInArea(relic, area) => {
                format!(
                    "{} {}",
                    relic!(*relic).name,
                    Effect::InArea(*area).to_string()
                )
            }
            Condition::Replacement(monster, replacement) => format!(
                "{} {}",
                monster!(*monster).name,
                Effect::Replacement(*replacement).to_string()
            ),
        }
    }
}

impl Condition {
    pub fn to_sql(&self) -> String {
        match self {
            Condition::Invalid(_) => String::from(""),
            Condition::MonsterAvailable(id) => format!("{id} IN ({})", BRAVERY_COLUMNS.join(",")),
            Condition::RelicAvailable(relic) => {
                format!("{} IN ({})", relic!(*relic).id, RELIC_COLUMNS.join(","))
            }
            Condition::BraveryChest(id) => format!(
                "{id} IN ({})",
                AREA_COLUMNS
                    .iter()
                    .map(|x| String::from("Bravery.") + x)
                    .collect::<Vec<String>>()
                    .join(",")
            ),
            Condition::Familiar(id) => format!("{}={id}", BRAVERY_COLUMNS[0]),
            Condition::Starter(id) => {
                format!(
                    "({}={id} OR {}={id})",
                    BRAVERY_COLUMNS[1], BRAVERY_COLUMNS[2]
                )
            }
            Condition::Swimming(id) => format!("{}={id}", BRAVERY_COLUMNS[3]),
            Condition::Bex(id) => format!("{}={id}", BRAVERY_COLUMNS[4]),
            Condition::Cryomancer(id) => format!("{}={id}", BRAVERY_COLUMNS[5]),
            Condition::CryomancerRequired(id) => format!("{}={id}", BRAVERY_COLUMNS[6]),
            Condition::EndOfTime(id) => format!(
                "{id} IN ({},{},{})",
                BRAVERY_COLUMNS[7], BRAVERY_COLUMNS[8], BRAVERY_COLUMNS[9]
            ),
            Condition::Army(id) => format!(
                "{id} IN ({},{},{},{},{},{},{})",
                BRAVERY_COLUMNS[10],
                BRAVERY_COLUMNS[11],
                BRAVERY_COLUMNS[12],
                BRAVERY_COLUMNS[13],
                BRAVERY_COLUMNS[14],
                BRAVERY_COLUMNS[15],
                BRAVERY_COLUMNS[16]
            ),
            Condition::MonsterInArea(monster, area) => {
                format!(
                    "{monster} IN ({})",
                    GAME_DATA.areas[*area as usize]
                        .wild_monsters
                        .iter()
                        .map(|x| format!("M{x}"))
                        .collect::<Vec<String>>()
                        .join(",")
                )
            }
            Condition::EggInArea(monster, area) => {
                format!("Bravery.{}={monster}", AREA_COLUMNS[*area as usize])
            }
            Condition::RelicInArea(relic, area) => {
                format!(
                    "Relic.{}={}",
                    AREA_COLUMNS[*area as usize],
                    relic!(*relic).id
                )
            }
            Condition::Replacement(monster, replacement) => {
                format!("Randomizer.M{}={replacement}", monster - 4)
            }
        }
    }
}
