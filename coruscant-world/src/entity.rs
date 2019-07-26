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

