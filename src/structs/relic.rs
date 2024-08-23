use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Relic {
    pub id: u32,
    pub name: String,
    pub monster_type_restriction: u32,
}
