use crate::data::macros::{area, monster};

pub enum Effect {
    None,
    Available,
    BraveryChest,
    Familiar,
    Starter,
    Swimming,
    Bex,
    Cryomancer,
    CryomancerRequired,
    EndOfTime,
    Army,
    InArea(u32),
    EggInArea(u32),
    Replacement(u32),
}

impl From<u32> for Effect {
    fn from(value: u32) -> Self {
        match value {
            0 => Effect::Available,
            1 => Effect::BraveryChest,
            2 => Effect::Familiar,
            3 => Effect::Starter,
            4 => Effect::Swimming,
            5 => Effect::Bex,
            6 => Effect::Cryomancer,
            7 => Effect::CryomancerRequired,
            8 => Effect::EndOfTime,
            9 => Effect::Army,
            10..=22 => Effect::EggInArea(value - 10),
            23..=35 => Effect::InArea(value - 23),
            _ => Effect::Replacement(value - 36),
        }
    }
}

impl From<(u32, bool, bool, bool)> for Effect {
    fn from((value, is_randomizer, is_bravery, is_relic): (u32, bool, bool, bool)) -> Self {
        if is_bravery {
            // Bravery (+ Randomizer) (+ Relic)
            match value {
                0 => Effect::Available,
                1 => Effect::BraveryChest,
                2 => Effect::Familiar,
                3 => Effect::Starter,
                4 => Effect::Swimming,
                5 => Effect::Bex,
                6 => Effect::Cryomancer,
                7 => Effect::CryomancerRequired,
                8 => Effect::EndOfTime,
                9 => Effect::Army,
                10..=22 => Effect::EggInArea(value - 10),
                _ => {
                    if is_randomizer {
                        match value {
                            23..=35 => Effect::InArea(value - 23),
                            _ => Effect::Replacement(value - 36),
                        }
                    } else if is_relic {
                        Effect::InArea(value - 23)
                    } else {
                        Effect::None
                    }
                }
            }
        } else if is_randomizer {
            if is_relic {
                // Randomizer + Relic
                match value {
                    0 => Effect::Available,
                    1..=13 => Effect::InArea(value - 1),
                    _ => Effect::Replacement(value - 14),
                }
            } else {
                // Randomizer
                match value {
                    0..=12 => Effect::InArea(value),
                    _ => Effect::Replacement(value - 13),
                }
            }
        } else if is_relic {
            Effect::Available
        } else {
            Effect::None
        }
    }
}

impl ToString for Effect {
    fn to_string(&self) -> String {
        match self {
            Effect::None => format!("has no effect"),
            Effect::Available => format!("is available"),
            Effect::BraveryChest => format!("is in a Bravery area chest"),
            Effect::Familiar => format!("is your spectral familiar"),
            Effect::Starter => format!("is a starter"),
            Effect::Swimming => format!("is given at the Sun Palace"),
            Effect::Bex => format!("is given by Bex"),
            Effect::Cryomancer => format!("is given by the Cryomancer"),
            Effect::CryomancerRequired => format!("is wanted by the Cryomancer"),
            Effect::EndOfTime => format!("is in Eternity's End"),
            Effect::Army => format!("is in the Bravery Monster Army"),
            Effect::EggInArea(value) => format!("egg is in {}", area!(*value).name),
            Effect::InArea(value) => format!("is in {}", area!(*value).name),
            Effect::Replacement(value) => format!("=> {}", monster!(*value).name),
        }
    }
}
