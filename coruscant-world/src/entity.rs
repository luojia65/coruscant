use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct EntityData {
    #[serde(rename = "id")]
    id: Option<String>,
    #[serde(rename = "Pos")]
    pos: [f64; 3],
    #[serde(rename = "Motion")]
    motion: [f64; 3],
    #[serde(rename = "Rotation")]
    rotation: [f32; 2],
    #[serde(rename = "FallDistance")]
    fall_distance: f32,
    #[serde(rename = "Fire")]
    fire: i16, 
    #[serde(rename = "Air")]
    air: i16,
    #[serde(rename = "OnGround")]
    on_ground: bool,
    #[serde(rename = "NoGravity")]
    no_gravity: bool,
    #[serde(rename = "Dimension")]
    dimension: i32,
    #[serde(rename = "Invulnerable")]
    invulnerable: bool,
    #[serde(rename = "PortalCooldown")]
    portal_cooldown: i32,
    #[serde(rename = "UUIDMost")]
    uuid_most: i64,
    #[serde(rename = "UUIDLeast")]
    uuid_least: i64,
    #[serde(rename = "CustomName")]
    custom_name: String,
    #[serde(rename = "CustomNameVisible")]
    custom_name_visible: bool,
    #[serde(rename = "Slient")]
    slient: bool,
    #[serde(rename = "Passengers")]
    passengers: Vec<EntityData>,
    #[serde(rename = "Glowing")]
    glowing: bool,
    #[serde(rename = "Tags")]
    tags: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Mob {
    #[serde(rename = "Health")]
    health: f32,
    #[serde(rename = "AbsorptionAmount")]
    absorption_amount: f32,
    #[serde(rename = "HurtTime")]
    hurt_time: i16,
    #[serde(rename = "HurtByTimestamp")]
    hurt_by_timestamp: i32,
    #[serde(rename = "DeathTime")]
    death_time: i16,
    #[serde(rename = "FallFlying")]
    fall_flying: bool,
    #[serde(flatten)]
    sleeping_pos: Option<SleepingPos>,
    #[serde(rename = "Brain")]
    brain: Brain,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SleepingPos {
    #[serde(rename = "SleepingX")]
    sleeping_x: i32,
    #[serde(rename = "SleepingY")]
    sleeping_y: i32,
    #[serde(rename = "SleepingZ")]
    sleeping_z: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Brain {
    // #[serde(rename = "memories")]
    // pub memories: Memories,
}

