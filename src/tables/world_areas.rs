#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct WorldAreas {
    pub r#id: String,
    pub r#name: String,
    pub r#act: i32,
    pub r#is_town: bool,
    pub r#has_waypoint: bool,
    pub r#connections_world_areas_keys: Vec<WorldAreasRow>,
    pub r#area_level: i32,
    pub r#hash16: i16,
    pub r#loading_screen_dds_file: String,
    pub r#topologies_keys: Vec<TopologiesRow>,
    pub r#parent_town_world_areas_key: Option<WorldAreasRow>,
    pub r#bosses_monster_varieties_keys: Vec<MonsterVarietiesRow>,
    pub r#monsters_monster_varieties_keys: Vec<MonsterVarietiesRow>,
    pub r#spawn_weight_tags_keys: Vec<TagsRow>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#is_map_area: bool,
    pub r#full_clear_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#achievement_items_key: Option<AchievementItemsRow>,
    pub r#mods_keys: Vec<ModsRow>,
    pub r#vaal_area: Vec<WorldAreasRow>,
    pub r#max_level: i32,
    pub r#area_type_tags: Vec<TagsRow>,
    pub r#is_hideout: bool,
    pub r#inflection: String,
    pub r#tags: Vec<TagsRow>,
    pub r#is_vaal_area: bool,
    pub r#is_labyrinth_airlock: bool,
    pub r#is_labyrinth_area: bool,
    pub r#twinned_full_clear_achievement_items_key: Option<AchievementItemsRow>,
    pub r#enter_achievement_items_key: Option<AchievementItemsRow>,
    pub r#tsi_file: String,
    pub r#waypoint_activation_achievement_items_keys: Vec<AchievementItemsRow>,
    pub r#is_unique_map_area: bool,
    pub r#is_labyrinth_boss_area: bool,
    pub r#first_entry_npc_text_audio_key: Option<NPCTextAudioRow>,
    pub r#first_entry_sound_effects_key: Option<SoundEffectsRow>,
    // column_ref first_entry_np_cs_key
    pub r#environments_key: Option<EnvironmentsRow>,
    pub r#league_chance1: Option<WorldAreaLeagueChancesRow>,
    pub r#league_chance2: Option<WorldAreaLeagueChancesRow>,
    pub r#ruleset: Option<RulesetsRow>,
    pub r#quest_flag: Option<QuestFlagsRow>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_WorldAreas: LazyLock<Vec<WorldAreas>> = LazyLock::new(|| {
    RAW_TABLE_WorldAreas
        .iter()
        .map(|x| {
            WorldAreas {
                r#id: x.r#id.clone(),
                r#name: x.r#name.clone(),
                r#act: x.r#act.clone(),
                r#is_town: x.r#is_town.clone(),
                r#has_waypoint: x.r#has_waypoint.clone(),
                r#connections_world_areas_keys: x
                    .r#connections_world_areas_keys
                    .iter()
                    .copied()
                    .map(WorldAreasRow)
                    .collect(),
                r#area_level: x.r#area_level.clone(),
                r#hash16: x.r#hash16.clone(),
                r#loading_screen_dds_file: x.r#loading_screen_dds_file.clone(),
                r#topologies_keys: x
                    .r#topologies_keys
                    .iter()
                    .copied()
                    .map(TopologiesRow)
                    .collect(),
                r#parent_town_world_areas_key: x.r#parent_town_world_areas_key.map(WorldAreasRow),
                r#bosses_monster_varieties_keys: x
                    .r#bosses_monster_varieties_keys
                    .iter()
                    .copied()
                    .map(MonsterVarietiesRow)
                    .collect(),
                r#monsters_monster_varieties_keys: x
                    .r#monsters_monster_varieties_keys
                    .iter()
                    .copied()
                    .map(MonsterVarietiesRow)
                    .collect(),
                r#spawn_weight_tags_keys: x
                    .r#spawn_weight_tags_keys
                    .iter()
                    .copied()
                    .map(TagsRow)
                    .collect(),
                r#spawn_weight_values: x.r#spawn_weight_values.clone(),
                r#is_map_area: x.r#is_map_area.clone(),
                r#full_clear_achievement_items_keys: x
                    .r#full_clear_achievement_items_keys
                    .iter()
                    .copied()
                    .map(AchievementItemsRow)
                    .collect(),
                r#achievement_items_key: x.r#achievement_items_key.map(AchievementItemsRow),
                r#mods_keys: x.r#mods_keys.iter().copied().map(ModsRow).collect(),
                r#vaal_area: x.r#vaal_area.iter().copied().map(WorldAreasRow).collect(),
                r#max_level: x.r#max_level.clone(),
                r#area_type_tags: x.r#area_type_tags.iter().copied().map(TagsRow).collect(),
                r#is_hideout: x.r#is_hideout.clone(),
                r#inflection: x.r#inflection.clone(),
                r#tags: x.r#tags.iter().copied().map(TagsRow).collect(),
                r#is_vaal_area: x.r#is_vaal_area.clone(),
                r#is_labyrinth_airlock: x.r#is_labyrinth_airlock.clone(),
                r#is_labyrinth_area: x.r#is_labyrinth_area.clone(),
                r#twinned_full_clear_achievement_items_key: x
                    .r#twinned_full_clear_achievement_items_key
                    .map(AchievementItemsRow),
                r#enter_achievement_items_key: x
                    .r#enter_achievement_items_key
                    .map(AchievementItemsRow),
                r#tsi_file: x.r#tsi_file.clone(),
                r#waypoint_activation_achievement_items_keys: x
                    .r#waypoint_activation_achievement_items_keys
                    .iter()
                    .copied()
                    .map(AchievementItemsRow)
                    .collect(),
                r#is_unique_map_area: x.r#is_unique_map_area.clone(),
                r#is_labyrinth_boss_area: x.r#is_labyrinth_boss_area.clone(),
                r#first_entry_npc_text_audio_key: x
                    .r#first_entry_npc_text_audio_key
                    .map(NPCTextAudioRow),
                r#first_entry_sound_effects_key: x
                    .r#first_entry_sound_effects_key
                    .map(SoundEffectsRow),
                // column_ref first_entry_np_cs_key
                r#environments_key: x.r#environments_key.map(EnvironmentsRow),
                r#league_chance1: x.r#league_chance1.map(WorldAreaLeagueChancesRow),
                r#league_chance2: x.r#league_chance2.map(WorldAreaLeagueChancesRow),
                r#ruleset: x.r#ruleset.map(RulesetsRow),
                r#quest_flag: x.r#quest_flag.map(QuestFlagsRow),
            }
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldAreasRow(pub usize);

impl Deref for WorldAreasRow {
    type Target = WorldAreas;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldAreas[self.0]
    }
}

impl WorldAreasRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldAreas {
        &TABLE_WorldAreas[self.0]
    }
    pub fn get(&self) -> &'static WorldAreas {
        &TABLE_WorldAreas[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldAreas.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldAreas)> {
        TABLE_WorldAreas
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_WorldAreas: LazyLock<Vec<WorldAreasRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/WorldAreas.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct WorldAreasRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "Act")]
    pub r#act: i32,
    #[serde(rename = "IsTown")]
    pub r#is_town: bool,
    #[serde(rename = "HasWaypoint")]
    pub r#has_waypoint: bool,
    #[serde(rename = "Connections_WorldAreasKeys")]
    pub r#connections_world_areas_keys: Vec<usize>,
    #[serde(rename = "AreaLevel")]
    pub r#area_level: i32,
    #[serde(rename = "HASH16")]
    pub r#hash16: i16,
    #[serde(rename = "LoadingScreen_DDSFile")]
    pub r#loading_screen_dds_file: String,
    #[serde(rename = "TopologiesKeys")]
    pub r#topologies_keys: Vec<usize>,
    #[serde(rename = "ParentTown_WorldAreasKey")]
    pub r#parent_town_world_areas_key: Option<usize>,
    #[serde(rename = "Bosses_MonsterVarietiesKeys")]
    pub r#bosses_monster_varieties_keys: Vec<usize>,
    #[serde(rename = "Monsters_MonsterVarietiesKeys")]
    pub r#monsters_monster_varieties_keys: Vec<usize>,
    #[serde(rename = "SpawnWeight_TagsKeys")]
    pub r#spawn_weight_tags_keys: Vec<usize>,
    #[serde(rename = "SpawnWeight_Values")]
    pub r#spawn_weight_values: Vec<i32>,
    #[serde(rename = "IsMapArea")]
    pub r#is_map_area: bool,
    #[serde(rename = "FullClear_AchievementItemsKeys")]
    pub r#full_clear_achievement_items_keys: Vec<usize>,
    #[serde(rename = "AchievementItemsKey")]
    pub r#achievement_items_key: Option<usize>,
    #[serde(rename = "ModsKeys")]
    pub r#mods_keys: Vec<usize>,
    #[serde(rename = "VaalArea")]
    pub r#vaal_area: Vec<usize>,
    #[serde(rename = "MaxLevel")]
    pub r#max_level: i32,
    #[serde(rename = "AreaTypeTags")]
    pub r#area_type_tags: Vec<usize>,
    #[serde(rename = "IsHideout")]
    pub r#is_hideout: bool,
    #[serde(rename = "Inflection")]
    pub r#inflection: String,
    #[serde(rename = "Tags")]
    pub r#tags: Vec<usize>,
    #[serde(rename = "IsVaalArea")]
    pub r#is_vaal_area: bool,
    #[serde(rename = "IsLabyrinthAirlock")]
    pub r#is_labyrinth_airlock: bool,
    #[serde(rename = "IsLabyrinthArea")]
    pub r#is_labyrinth_area: bool,
    #[serde(rename = "TwinnedFullClear_AchievementItemsKey")]
    pub r#twinned_full_clear_achievement_items_key: Option<usize>,
    #[serde(rename = "Enter_AchievementItemsKey")]
    pub r#enter_achievement_items_key: Option<usize>,
    #[serde(rename = "TSIFile")]
    pub r#tsi_file: String,
    #[serde(rename = "WaypointActivation_AchievementItemsKeys")]
    pub r#waypoint_activation_achievement_items_keys: Vec<usize>,
    #[serde(rename = "IsUniqueMapArea")]
    pub r#is_unique_map_area: bool,
    #[serde(rename = "IsLabyrinthBossArea")]
    pub r#is_labyrinth_boss_area: bool,
    #[serde(rename = "FirstEntry_NPCTextAudioKey")]
    pub r#first_entry_npc_text_audio_key: Option<usize>,
    #[serde(rename = "FirstEntry_SoundEffectsKey")]
    pub r#first_entry_sound_effects_key: Option<usize>,
    #[serde(rename = "FirstEntry_NPCsKey")]
    pub r#first_entry_np_cs_key: String,
    #[serde(rename = "EnvironmentsKey")]
    pub r#environments_key: Option<usize>,
    #[serde(rename = "LeagueChance1")]
    pub r#league_chance1: Option<usize>,
    #[serde(rename = "LeagueChance2")]
    pub r#league_chance2: Option<usize>,
    #[serde(rename = "Ruleset")]
    pub r#ruleset: Option<usize>,
    #[serde(rename = "QuestFlag")]
    pub r#quest_flag: Option<usize>,
}
