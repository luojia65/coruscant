use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use coruscant_nbt::as_nbt_array;

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
    #[serde(rename = "Attributes")]
    pub attributes: Vec<Attribute>,
    #[serde(rename = "ActiveEffects")]
    pub active_effects: Vec<ActiveEffect>,
    // #[serde(rename = "HandItems")]
    // pub hand_items: Vec<Item>,
    // #[serde(rename = "ArmorItems")]
    // pub armor_items: Vec<Item>,
    #[serde(rename = "HandDropChances")]
    pub hand_drop_chances: Vec<f32>,
    #[serde(rename = "ArmorDropChances")]
    pub armor_drop_chances: Vec<f32>,
    #[serde(rename = "DeathLootTable")]
    pub death_loot_table: Option<String>,
    #[serde(rename = "DeathLootTableSeed")]
    pub death_loot_table_seed: Option<i64>,
    #[serde(rename = "CanPickUpLoot")]
    pub can_pick_up_loot: bool,
    #[serde(rename = "NoAI")]
    pub no_ai: bool,
    #[serde(rename = "PersistenceRequired")]
    pub persistence_required: bool,
    #[serde(rename = "LeftHanded")]
    pub left_handed: bool,
    #[serde(rename = "Team")]
    pub team: String,
    #[serde(rename = "Leashed")]
    pub leashed: bool,
    #[serde(rename = "Leash")]
    pub leash: Option<Leash>,
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
    #[serde(rename = "memories")]
    pub memories: HashMap<String, Memory>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Memory {
    MemoryPosition { 
        #[serde(rename = "pos")]
        #[serde(serialize_with = "as_nbt_array")]
        pos: [i32; 3],
        #[serde(rename = "dimension")]
        dimension: String,
    },
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Attribute {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Base")]
    pub base: f64,
    #[serde(rename = "Modifiers")]
    pub modifiers: Vec<Modifier>,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Modifier {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Amount")]
    pub amount: f64,
    #[serde(rename = "Operation")]
    pub operation: i32,
    #[serde(rename = "UUIDMost")]
    pub uuid_most: i64,
    #[serde(rename = "UUIDLeast")]
    pub uuid_least: i64,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ActiveEffect {
    #[serde(rename = "Id")]
    pub id: i8,
    #[serde(rename = "Amplifier")]
    pub amplifier: i8,
    #[serde(rename = "Duration")]
    pub duration: i32,
    #[serde(rename = "Ambient")]
    pub ambient: bool,
    #[serde(rename = "ShowParticles")]
    pub show_particles: bool,
    #[serde(rename = "ShowIcon")]
    pub show_icon: bool,
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub enum Leash {
    ToEntity {
        #[serde(rename = "UUIDMost")]
        uuid_most: i64,
        #[serde(rename = "UUIDLeast")]
        uuid_least: i64,
    },
    ToFence {
        #[serde(rename = "X")]
        x: i32,
        #[serde(rename = "Y")]
        y: i32,
        #[serde(rename = "Z")]
        z: i32,
    },
}

