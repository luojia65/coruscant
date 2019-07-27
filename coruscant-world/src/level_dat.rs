//! Level format parsed as serde favored Rust structs.
//!
//! Ref: https://minecraft.gamepedia.com/Level_format
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Root repersentation of `level.dat` file.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "")]
#[non_exhaustive]
pub struct LevelDat {
    /// Contains all the level data.
    #[serde(rename = "Data")]
    data: Data,
}

/// Container for all the level data.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Data {
    /// A ID/BossEvent collection of bossbars. ID of a bossbar is a string like
    /// `custom::boss`.
    #[serde(rename = "CustomBossEvents")]
    custom_boss_events: HashMap<String, BossEvent>,
    /// Options for datapacks.
    #[serde(rename = "DataPacks")]
    data_packs: DataPacks,
    /// This tag contains level data specific to certain dimensions.
    #[serde(rename = "DimensionData")]
    dimension_data: DimensionData,
    /// The NBT version of the level, 19133.
    #[serde(rename = "version")]
    version_int: i32,
    /// Normally true after a world has been initialized properly after creation.
    /// If the initial simulation was canceled somehow, this can be false and the
    /// world will be re-initialized on next load.
    #[serde(rename = "initialized")]
    initialized: bool,
    /// The name of the level.
    #[serde(rename = "LevelName")]
    level_name: String,
    /// The name of the generator; "default", "flat", "largeBiomes", "amplified", "buffet",
    /// or "debug_all_block_states".
    ///
    /// Not case sensitive, but always written in the case here.
    #[serde(rename = "generatorName")]
    generator_name: String,
    // todo: generator version; generator settings
    /// The [random level seed] used to generate consistent terrain.
    ///
    /// [random level seed]: https://minecraft.gamepedia.com/Seed_(level_generation)
    #[serde(rename = "RandomSeed")]
    random_seed: i64,
    /// True if the map generator should place structures such as villages, strongholds,
    /// and mineshafts. Defaults to true. Always true if the world type is Customized.
    #[serde(rename = "MapFeatures")]
    map_features: bool,
    /// The Unix time in milliseconds when the level was last loaded.
    #[serde(rename = "LastPlayed")]
    last_played: i64,
    /// The estimated size in bytes of the level. Currently not modified or used by Minecraft,
    /// but was previously.
    #[serde(rename = "SizeOnDisk")]
    size_on_disk: i64,
    /// True if cheats are enabled.
    #[serde(rename = "allowCommands")]
    allow_commands: bool,
    /// True if the player will respawn in Spectator on death in singleplayer.
    /// Affects all three game modes.
    #[serde(rename = "hardcore")]
    hardcore: bool,
    /// An integer displaying the [data version].
    ///
    /// [data version]: https://minecraft.gamepedia.com/Data_version
    #[serde(rename = "DataVersion")]
    data_version: i32,
    /// The current difficulty setting. 0 is Peaceful, 1 is Easy, 2 is Normal, and 3 is Hard.
    /// Defaults to 2.
    #[serde(rename = "Difficulty")]
    difficulty: i8,
    /// True if the difficulty has been locked. Defaults to false.
    #[serde(rename = "DifficultyLocked")]
    difficulty_locked: bool,
    /// The number of ticks since the start of the level.
    #[serde(rename = "Time")]
    time: i64,
    /// The time of day. 0 is sunrise, 6000 is mid day, 12000 is sunset, 18000 is mid night,
    /// 24000 is the next day's 0. This value keeps counting past 24000 and does not reset to 0.
    #[serde(rename = "DayTime")]
    day_time: i64,
    /// The default game mode for the singleplayer player when they initially spawn.
    /// 0 is Survival, 1 is Creative, 2 is Adventure, 3 is Spectator.
    ///
    /// Note: Singleplayer worlds do not use this field to save which game mode the player
    /// is currently in.
    #[serde(rename = "GameType")]
    game_type: i32,
    /// The X coordinate of the world spawn.
    #[serde(rename = "SpawnX")]
    spawn_x: i32,
    /// The Y coordinate of the world spawn.
    #[serde(rename = "SpawnY")]
    spawn_y: i32,
    /// The Z coordinate of the world spawn.
    #[serde(rename = "SpawnZ")]
    spawn_z: i32,
    /// Center of the world border on the X coordinate. Defaults to `0`.
    #[serde(rename = "BorderCenterX")]
    border_center_x: f64,
    /// Center of the world border on the Z coordinate. Defaults to `0`.
    #[serde(rename = "BorderCenterZ")]
    border_center_z: f64,
    /// Width of the border. Defaults to 60000000.
    #[serde(rename = "BorderSize")]
    border_size: f64,
    /// Defaults to `5`. // todo: figure out what these are
    #[serde(rename = "BorderSafeZone")]
    border_safe_zone: f64,
    /// Defaults to `5`.
    #[serde(rename = "BorderWarningBlocks")]
    border_warning_blocks: f64,
    /// Defaults to `15`.
    #[serde(rename = "BorderWarningTime")]
    border_warning_time: f64,
    /// Defaults to `60000000`.
    #[serde(rename = "BorderSizeLerpTarget")]
    border_size_lerp_target: f64,
    /// Defaults to `0`.
    #[serde(rename = "BorderSizeLerpTime")]
    border_size_lerp_time: i64,
    /// Defaults to `0.2`.
    #[serde(rename = "BorderDamagePerBlock")]
    border_damage_per_block: f64,
    /// True if the level is currently experiencing rain, snow, and cloud cover.
    #[serde(rename = "raining")]
    raining: bool,
    /// The number of ticks before "raining" is toggled and this value gets set to
    /// another random value.
    #[serde(rename = "rainTime")]
    rain_time: i32,
    /// True if the rain/snow/cloud cover is a lightning storm and dark enough for mobs
    /// to spawn under the sky.
    #[serde(rename = "thundering")]
    thundering: bool,
    /// The number of ticks before "thundering" is toggled and this value gets set to
    /// another random value.
    #[serde(rename = "thunderTime")]
    thunder_time: i32,
    /// The number of ticks until "clear weather" has ended.
    #[serde(rename = "clearWeatherTime")]
    clear_weather_time: i32,
    // #[serde(rename = "Player")]
    // player:
    // #[serde(rename = "GameRules")]
    // game_rules:
    /// The UUID of the current wandering trader in the world.
    #[serde(rename = "WanderingTraderId")]
    wandering_trader_id: String,
    /// The current chance of the wandering trader spawning next attempt.
    ///  
    /// This value is the percentage and will be divided by 10 when loaded by the game,
    /// for example a value of 50 means 5.0% chance.
    #[serde(rename = "WanderingTraderSpawnChance")]
    wandering_trader_spawn_chance: i32,
    /// The amount of ticks until another wandering trader is attempted to spawn.
    #[serde(rename = "WanderingTraderSpawnDelay")]
    wandering_trader_spawn_delay: i32,
    /// Information about the Minecraft version the world was saved in.
    #[serde(rename = "Version")]
    version: Version,
}

/// Represents one single bossbar in `CustomBossEvents`.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct BossEvent {
    /// A list of players that may see this boss bar.
    #[serde(rename = "Players")]
    players: Vec<PlayerUuid>,
    /// ID of the color of the bossbar. Also sets the color of the display text of the
    /// bossbar, provided that the display text does not explicitly define a color for
    /// itself. See [color codes] for accepted values.
    ///
    /// [color codes]: https://minecraft.gamepedia.com/Formatting_codes#Color_codes
    #[serde(rename = "Color")]
    color: String,
    /// If the bossbar should create fog.
    #[serde(rename = "CreateWorldFog")]
    create_world_fog: bool,
    /// If the bossbar should darken the sky.
    #[serde(rename = "DarkenScreen")]
    darken_screen: bool,
    /// The maximum health of the bossbar.
    #[serde(rename = "Max")]
    max_health: i32,
    /// The current health of the bossbar.
    #[serde(rename = "Value")]
    cur_health: i32,
    /// The display name of the bossbar as a [JSON text component].
    ///
    /// [JSON text component]: https://minecraft.gamepedia.com/Commands#Raw_JSON_text
    #[serde(rename = "Name")]
    name: String,
    /// The ID of the overlay to be shown over the health bar. Accepted values are:
    /// `progress`, `notched_6`, `notched_10`, `notched_12`, and `notched_20`.
    #[serde(rename = "Overlay")]
    overlay: String,
    /// If the bossbar should initiate boss music.
    #[serde(rename = "PlayBossMusic")]
    play_boss_music: bool,
    /// If the bossbar should be visible to the listed players.
    #[serde(rename = "Visible")]
    visible: bool,
}

/// Uuid of a single player.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct PlayerUuid {
    /// The least significant bits of the player's Universally Unique IDentifier.
    /// This is joined with M to form the player's unique ID.
    #[serde(rename = "L")]
    least: i64,
    /// The most significant bits of the player's Universally Unique IDentifier.
    /// This is joined with L to form the player's unique ID.
    #[serde(rename = "M")]
    most: i64,
}

/// Options for datapacks.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DataPacks {
    /// List of disabled datapacks.
    #[serde(rename = "Disabled")]
    disabled: Vec<String>,
    /// List of enabled datapacks. By default, this is populated with a single string "vanilla".
    #[serde(rename = "Enabled")]
    enabled: Vec<String>,
}

/// Level data specific to certain dimensions.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DimensionData {
    /// Data for The End
    #[serde(rename = "1")]
    the_end: EnderDimensionData,
}

/// Level data for The End
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct EnderDimensionData {
    /// Data for the ender dragon fight. Only appears after the end is entered.
    ///
    /// `None` for the end is not entered.
    #[serde(rename = "DragonFlight")]
    dragon_flight: Option<DragonFlight>,
}

/// Data for the ender dragon fight.
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct DragonFlight {
    /// Location of the End's exit portal that the ender dragon flies to upon it's death
    #[serde(rename = "ExitPortalLocation")]
    exit_portal_location: ExitPortalLocation,
    /// Contains a list of locations of the [End gateway portals] that haven't been spawned.
    ///
    /// [End gateway portals]: https://minecraft.gamepedia.com/End_gateway_portal
    #[serde(rename = "Gateways")]
    gateways: Vec<i32>,
    /// If the dragon is currently alive. // todo: true for alive?
    #[serde(rename = "DragonKilled")]
    dragon_killed: bool,
    /// The least significant bits of the current Ender Dragon's Universally Unique IDentifier.
    /// This is joined with DragonUUIDMost to form the dragon's unique ID.
    #[serde(rename = "DragonUUIDLeast")]
    dragon_uuid_least: i64,
    /// The most significant bits of the current Ender Dragon's Universally Unique IDentifier.
    /// This is joined with DragonUUIDLeast to form the dragon's unique ID.
    #[serde(rename = "DragonUUIDMost")]
    dragon_uuid_most: i64,
    /// If the ender dragon has ever been defeated. Used to determine EXP given by dragon.
    #[serde(rename = "PreviouslyKilled")]
    previously_killed: bool,
}

/// Location of the End's exit portal that the ender dragon flies to upon it's death
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct ExitPortalLocation {
    /// The X coordinate of the portal.
    #[serde(rename = "X")]
    x: i8,
    /// The Y coordinate of the portal.
    #[serde(rename = "Y")]
    y: i8,
    /// The Z coordinate of the portal.
    #[serde(rename = "Z")]
    z: i8,
}

/// Information about the Minecraft version the world was saved in.
#[derive(Serialize, Deserialize, Default, Clone, Debug, PartialEq)]
#[non_exhaustive]
pub struct Version {
    #[serde(rename = "Id")]
    pub id: i32,
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Snapshot")]
    pub snapshot: bool,
}

impl Version {
    pub fn new(id: i32, name: String, snapshot: bool) -> Version {
        Version { id, name, snapshot }
    }
}
