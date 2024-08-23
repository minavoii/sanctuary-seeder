pub enum Value {
    Monster(u32),
    Relic(u32),
}

impl From<(u32, bool, bool)> for Value {
    fn from((id, is_randomizer, is_bravery): (u32, bool, bool)) -> Self {
        if is_randomizer || is_bravery {
            match id {
                // Do not include Bard as it is an hard-coded reward
                0..=109 => Value::Monster(id),
                110.. => Value::Relic(id - 110),
            }
        } else {
            Value::Relic(id)
        }
    }
}
