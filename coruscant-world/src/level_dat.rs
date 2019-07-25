//! Level format parsed as serde favored Rust structs.
//! 
//! Ref: https://minecraft.gamepedia.com/Level_format

use serde::{Serialize, Deserialize};
use std::collections::HashMap;

/// Root tag of `level.dat` file. This tag should have an empty name.
#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "")]
pub struct LevelDat {
    #[serde(rename = "Data")]
    pub data: Data,
}

/// This tag contains all the level data. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Data {
    /// A ID/BossEvent collection of bossbars. ID of a bossbar is a string like
    /// `custom::boss`.
    #[serde(rename = "CustomBossEvents")]
    pub custom_boss_enevts: HashMap<String, BossEvent>,
    /// Options for datapacks. 
    #[serde(rename = "DataPacks")]
    pub data_packs: DataPacks,
    /// This tag contains level data specific to certain dimensions. 
    #[serde(rename = "DimensionData")]
    pub dimension_data: DimensionData,
    /// The NBT version of the level, 19133.
    #[serde(rename = "version")]
    pub version_int: i32,
    /// Normally true after a world has been initialized properly after creation. 
    /// If the initial simulation was canceled somehow, this can be false and the 
    /// world will be re-initialized on next load.
    #[serde(rename = "initialized")]
    pub initialized: bool,
    /// The name of the level.
    #[serde(rename = "LevelName")]
    pub level_name: String,
    /// The name of the generator; "default", "flat", "largeBiomes", "amplified", "buffet", 
    /// or "debug_all_block_states". 
    /// 
    /// Not case sensitive, but always written in the case here.
    #[serde(rename = "generatorName")]
    pub generator_name: String,
    // todo: generator version; generator settings
    /// The [random level seed] used to generate consistent terrain.
    /// 
    /// [random level seed]: https://minecraft.gamepedia.com/Seed_(level_generation)
    #[serde(rename = "RandomSeed")]
    pub random_seed: i64,
    /// True if the map generator should place structures such as villages, strongholds, 
    /// and mineshafts. Defaults to true. Always true if the world type is Customized.
    #[serde(rename = "MapFeatures")]
    pub map_features: bool,
    /// The Unix time in milliseconds when the level was last loaded.
    #[serde(rename = "LastPlayed")]
    pub last_played: i64,
    /// The estimated size in bytes of the level. Currently not modified or used by Minecraft, 
    /// but was previously.
    #[serde(rename = "SizeOnDisk")]
    pub size_on_disk: i64,
    /// True if cheats are enabled.
    #[serde(rename = "allowCommands")]
    pub allow_commands: bool,
    /// True if the player will respawn in Spectator on death in singleplayer. 
    /// Affects all three game modes.
    #[serde(rename = "hardcore")]
    pub hardcore: bool,
    /// An integer displaying the [data version].
    /// 
    /// [data version]: https://minecraft.gamepedia.com/Data_version
    #[serde(rename = "DataVersion")]
    pub data_version: i32,
    /// The current difficulty setting. 0 is Peaceful, 1 is Easy, 2 is Normal, and 3 is Hard. 
    /// Defaults to 2.
    #[serde(rename = "Difficulty")]
    pub difficulty: i8,
    /// True if the difficulty has been locked. Defaults to false.
    #[serde(rename = "DifficultyLocked")]
    pub difficulty_locked: bool,
    /// The number of ticks since the start of the level.
    #[serde(rename = "Time")]
    pub time: i64,
    /// The time of day. 0 is sunrise, 6000 is mid day, 12000 is sunset, 18000 is mid night, 
    /// 24000 is the next day's 0. This value keeps counting past 24000 and does not reset to 0.
    #[serde(rename = "DayTime")]
    pub day_time: i64,
    /// The default game mode for the singleplayer player when they initially spawn. 
    /// 0 is Survival, 1 is Creative, 2 is Adventure, 3 is Spectator. 
    /// 
    /// Note: Singleplayer worlds do not use this field to save which game mode the player 
    /// is currently in.
    #[serde(rename = "GameType")]
    pub game_type: i32,
    /// The X coordinate of the world spawn.
    #[serde(rename = "SpawnX")]
    pub spawn_x: i32,
    /// The Y coordinate of the world spawn.
    #[serde(rename = "SpawnY")]
    pub spawn_y: i32,
    /// The Z coordinate of the world spawn.
    #[serde(rename = "SpawnZ")]
    pub spawn_z: i32,
    /// Center of the world border on the X coordinate. Defaults to `0`.
    #[serde(rename = "BorderCenterX")]
    pub border_center_x: f64,
    /// Center of the world border on the Z coordinate. Defaults to `0`.
    #[serde(rename = "BorderCenterZ")]
    pub border_center_z: f64,
    /// Width of the border. Defaults to 60000000.
    #[serde(rename = "BorderSize")]
    pub border_size: f64,
    /// Defaults to `5`. // todo: figure out what these are
    #[serde(rename = "BorderSafeZone")]
    pub border_safe_zone: f64,
    /// Defaults to `5`.
    #[serde(rename = "BorderWarningBlocks")]
    pub border_warning_blocks: f64,
    /// Defaults to `15`.
    #[serde(rename = "BorderWarningTime")]
    pub border_warning_time: f64,
    /// Defaults to `60000000`.
    #[serde(rename = "BorderSizeLerpTarget")]
    pub border_size_lerp_target: f64,
    /// Defaults to `0`.
    #[serde(rename = "BorderSizeLerpTime")]
    pub border_size_lerp_time: i64,
    /// Defaults to `0.2`.
    #[serde(rename = "BorderDamagePerBlock")]
    pub border_damage_per_block: f64,
    /// True if the level is currently experiencing rain, snow, and cloud cover.
    #[serde(rename = "raining")]
    pub raining: bool,
    /// The number of ticks before "raining" is toggled and this value gets set to 
    /// another random value.
    #[serde(rename = "rainTime")]
    pub rain_time: i32,
    /// True if the rain/snow/cloud cover is a lightning storm and dark enough for mobs 
    /// to spawn under the sky.
    #[serde(rename = "thundering")]
    pub thundering: bool,
    /// The number of ticks before "thundering" is toggled and this value gets set to 
    /// another random value.
    #[serde(rename = "thunderTime")]
    pub thunder_time: i32,
    /// The number of ticks until "clear weather" has ended.
    #[serde(rename = "clearWeatherTime")]
    pub clear_weather_time: i32,
    // #[serde(rename = "Player")]
    // player: 
    // #[serde(rename = "GameRules")]
    // game_rules: 
    /// The UUID of the current wandering trader in the world.
    #[serde(rename = "WanderingTraderId")]
    pub wandering_trader_id: String,
    /// The current chance of the wandering trader spawning next attempt.
    ///  
    /// This value is the percentage and will be divided by 10 when loaded by the game, 
    /// for example a value of 50 means 5.0% chance.
    #[serde(rename = "WanderingTraderSpawnChance")]
    pub wandering_trader_spawn_chance: i32,
    /// The amount of ticks until another wandering trader is attempted to spawn.
    #[serde(rename = "WanderingTraderSpawnDelay")]
    pub wanding_trader_spawn_delay: i32,
    /// Information about the Minecraft version the world was saved in. 
    #[serde(rename = "Version")]
    pub version: Version,
}

/// Represents one single bossbar in `CustomBossEvents`.
#[derive(Serialize, Deserialize, Debug)]
pub struct BossEvent {
    /// A list of players that may see this boss bar. 
    #[serde(rename = "Players")]
    pub players: Vec<PlayerUuid>,
    /// ID of the color of the bossbar. Also sets the color of the display text of the
    /// bossbar, provided that the display text does not explicitly define a color for 
    /// itself. See [color codes] for accepted values.
    /// 
    /// [color codes]: https://minecraft.gamepedia.com/Formatting_codes#Color_codes
    #[serde(rename = "Color")]
    pub color: String,
    /// If the bossbar should create fog.
    #[serde(rename = "CreateWorldFog")]
    pub create_world_fog: bool,
    /// If the bossbar should darken the sky.
    #[serde(rename = "DarkenScreen")]
    pub darken_screen: bool,
    /// The maximum health of the bossbar.
    #[serde(rename = "Max")]
    pub max_health: i32,
    /// The current health of the bossbar.
    #[serde(rename = "Value")]
    pub cur_health: i32,
    /// The display name of the bossbar as a [JSON text component].
    /// 
    /// [JSON text component]: https://minecraft.gamepedia.com/Commands#Raw_JSON_text
    #[serde(rename = "Name")]
    pub name: String,
    /// The ID of the overlay to be shown over the health bar. Accepted values are: 
    /// `progress`, `notched_6`, `notched_10`, `notched_12`, and `notched_20`.
    #[serde(rename = "Overlay")]
    pub overlay: String,
    /// If the bossbar should initiate boss music.
    #[serde(rename = "PlayBossMusic")]
    pub play_boss_music: bool,
    /// If the bossbar should be visible to the listed players.
    #[serde(rename = "Visible")]
    pub visible: bool,
}

/// Uuid of a single player.
#[derive(Serialize, Deserialize, Debug)]
pub struct PlayerUuid {
    /// The least significant bits of the player's Universally Unique IDentifier. 
    /// This is joined with M to form the player's unique ID.
    #[serde(rename = "L")]
    pub least: i64,
    /// The most significant bits of the player's Universally Unique IDentifier. 
    /// This is joined with L to form the player's unique ID.
    #[serde(rename = "M")]
    pub most: i64,
}

/// Options for datapacks.
#[derive(Serialize, Deserialize, Debug)]
pub struct DataPacks {
    /// List of disabled datapacks. 
    #[serde(rename = "Disabled")]
    pub disabled: Vec<String>,
    /// List of enabled datapacks. By default, this is populated with a single string "vanilla". 
    #[serde(rename = "Enabled")]
    pub enabled: Vec<String>,
}

/// Level data specific to certain dimensions. 
#[derive(Serialize, Deserialize, Debug)]
pub struct DimensionData {
    /// Data for The End 
    #[serde(rename = "1")]
    pub the_end: EnderDimensionData,
}

/// Level data for The End 
#[derive(Serialize, Deserialize, Debug)]
pub struct EnderDimensionData {
    /// Data for the ender dragon fight. Only appears after the end is entered. 
    /// 
    /// `None` for the end is not entered.
    #[serde(rename = "DragonFlight")]
    pub dragon_flight: Option<DragonFlight>,
}

///  Data for the ender dragon fight. 
#[derive(Serialize, Deserialize, Debug)]
pub struct DragonFlight {
    /// Location of the End's exit portal that the ender dragon flies to upon it's death 
    #[serde(rename = "ExitPortalLocation")]
    pub exit_portal_location: ExitPortalLocation,
    /// Contains a list of locations of the [End gateway portals] that haven't been spawned. 
    /// 
    /// [End gateway portals]: https://minecraft.gamepedia.com/End_gateway_portal
    #[serde(rename = "Gateways")]
    pub gateways: Vec<i32>,
    /// If the dragon is currently alive. // todo: true for alive?
    #[serde(rename = "DragonKilled")]
    pub dragon_killed: bool,
    /// The least significant bits of the current Ender Dragon's Universally Unique IDentifier. 
    /// This is joined with DragonUUIDMost to form the dragon's unique ID.
    #[serde(rename = "DragonUUIDLeast")]
    pub dragon_uuid_least: i64,
    /// The most significant bits of the current Ender Dragon's Universally Unique IDentifier. 
    /// This is joined with DragonUUIDLeast to form the dragon's unique ID.
    #[serde(rename = "DragonUUIDMost")]
    pub dragon_uuid_most: i64,
    /// If the ender dragon has ever been defeated. Used to determine EXP given by dragon.
    #[serde(rename = "PreviouslyKilled")]
    pub previously_killed: bool,
}

/// Location of the End's exit portal that the ender dragon flies to upon it's death 
#[derive(Serialize, Deserialize, Debug)]
pub struct ExitPortalLocation {
    /// The X coordinate of the portal.
    #[serde(rename = "X")]
    pub x: i8,
    /// The Y coordinate of the portal.
    #[serde(rename = "Y")]
    pub y: i8,
    /// The Z coordinate of the portal.
    #[serde(rename = "Z")]
    pub z: i8,
}

/// Information about the Minecraft version the world was saved in. 
#[derive(Serialize, Deserialize, Debug)]
pub struct Version {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Snapshot")]
    pub snapshot: bool,
}
