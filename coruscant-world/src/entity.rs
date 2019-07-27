use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct EntityData {
    #[serde(rename = "id")]
    pub id: Option<String>,
    #[serde(rename = "Pos")]
    pub pos: [f64; 3],
    #[serde(rename = "Motion")]
    pub motion: [f64; 3],
    #[serde(rename = "Rotation")]
    pub rotation: [f32; 2],
    #[serde(rename = "FallDistance")]
    pub fall_distance: f32,
    #[serde(rename = "Fire")]
    pub fire: i16, 
    #[serde(rename = "Air")]
    pub air: i16,
    #[serde(rename = "OnGround")]
    pub on_ground: bool,
    #[serde(rename = "NoGravity")]
    pub no_gravity: bool,
    #[serde(rename = "Dimension")]
    pub dimension: i32,
    #[serde(rename = "Invulnerable")]
    pub invulnerable: bool,
    #[serde(rename = "PortalCooldown")]
    pub portal_cooldown: i32,
    #[serde(rename = "UUIDMost")]
    pub uuid_most: i64,
    #[serde(rename = "UUIDLeast")]
    pub uuid_least: i64,
    #[serde(rename = "CustomName")]
    pub custom_name: String,
    #[serde(rename = "CustomNameVisible")]
    pub custom_name_visible: bool,
    #[serde(rename = "Slient")]
    pub slient: bool,
    #[serde(rename = "Passengers")]
    pub passengers: Vec<EntityData>,
    #[serde(rename = "Glowing")]
    pub glowing: bool,
    #[serde(rename = "Tags")]
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Mob {
    #[serde(rename = "Health")]
    pub health: f32,
    #[serde(rename = "AbsorptionAmount")]
    pub absorption_amount: f32,
    #[serde(rename = "HurtTime")]
    pub hurt_time: i16,
    #[serde(rename = "HurtByTimestamp")]
    pub hurt_by_timestamp: i32,
    #[serde(rename = "DeathTime")]
    pub death_time: i16,
    #[serde(rename = "FallFlying")]
    pub fall_flying: bool,
    #[serde(flatten)]
    pub sleeping_pos: Option<SleepingPos>,
    #[serde(rename = "Brain")]
    pub brain: Brain,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct SleepingPos {
    #[serde(rename = "SleepingX")]
    pub sleeping_x: i32,
    #[serde(rename = "SleepingY")]
    pub sleeping_y: i32,
    #[serde(rename = "SleepingZ")]
    pub sleeping_z: i32,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Brain {
    // #[serde(rename = "memories")]
    // pub memories: HashMap<String, Memory>,
}

