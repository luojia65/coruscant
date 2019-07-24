use serde::{Serialize, Deserialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "")]
pub struct LevelDat {
    #[serde(rename = "Data")]
    data: Data,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    #[serde(rename = "CustomBossEvents")]
    custom_boss_enevts: HashMap<String, BossEvent>,
    #[serde(rename = "DataPacks")]
    data_packs: DataPacks,
    #[serde(rename = "DimensionData")]
    dimension_data: DimensionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BossEvent {
    #[serde(rename = "Players")]
    players: Vec<PlayerUuid>,
    #[serde(rename = "Color")]
    color: String,
    #[serde(rename = "CreateWorldFog")]
    create_world_fog: bool,
    #[serde(rename = "DarkenScreen")]
    darken_screen: bool,
    #[serde(rename = "Max")]
    max_value: i32,
    #[serde(rename = "Value")]
    cur_value: i32,
    #[serde(rename = "Name")]
    name: String,
    #[serde(rename = "Overlay")]
    overlay: String,
    #[serde(rename = "PlayBossMusic")]
    play_boss_music: bool,
    #[serde(rename = "Visible")]
    visible: bool,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerUuid {
    #[serde(rename = "L")]
    lower: i64,
    #[serde(rename = "M")]
    higher: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacks {
    #[serde(rename = "Disabled")]
    disabled: Vec<String>,
    #[serde(rename = "Enabled")]
    enabled: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DimensionData {
    #[serde(rename = "1")]
    ender: EnderDimensionData,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct EnderDimensionData {
    #[serde(rename = "DragonFlight")]
    dragon_flight: DragonFlight,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DragonFlight {
    // #[serde(rename = "ExitPortalLocation")]
    // exit_portal_location: Position
}

