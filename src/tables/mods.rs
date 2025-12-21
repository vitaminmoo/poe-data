#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Mods: LazyLock<Vec<ModsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mods.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ModsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#mod_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(10..10 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModTypeRef::new(value as usize)
            },
            r#level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(30..30 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(46..46 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#stat4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(78..78 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#domain: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                ModDomains::from_repr(value as usize)
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(98..98 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#generation_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                ModGenerationType::from_repr(value as usize)
            },
            r#families: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(110..110 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ModFamilyRef::new(value as usize))
                    .collect()
            },
            r#stat1_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(126..126 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat2_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(134..134 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat3_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(142..142 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat4_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(150..150 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#spawn_weight_tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(158..158 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#unknown174: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(174..174 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(190..190 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#granted_effects_per_level: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(206..206 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| GrantedEffectsPerLevelRef::new(value as usize))
                    .collect()
            },
            r#aura_flags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(222..222 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
                    .into_iter()
                    .map(|value| ModAuraFlags::from_repr(value as usize))
                    .collect()
            },
            r#monster_metadata: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(238..238 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_kill_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(246..246 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#chest_mod_type: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(262..262 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ModTypeRef::new(value as usize))
                    .collect()
            },
            r#stat5_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(278..278 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat5: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(286..286 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#full_area_clear_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(302..302 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(318..318 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#generation_weight_tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(334..334 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#generation_weight_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(350..350 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#modify_maps_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(366..366 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#stat6_value: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(382..382 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat6: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(390..390 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(406..406 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown410: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(410).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#crafting_item_class_restrictions: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(411..411 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ItemClassesRef::new(value as usize))
                    .collect()
            },
            r#monster_on_death: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(427..427 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown435: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(435..435 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(439..439 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| AchievementItemsRef::new(value as usize))
                    .collect()
            },
            r#heist_sub_stat_value1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(455..455 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_sub_stat_value2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(459..459 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_stat0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(463..463 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#heist_stat1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(479..479 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#heist_add_stat_value1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(495..495 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#heist_add_stat_value2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(499..499 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#influence_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(503..503 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                InfluenceTypes::from_repr(value as usize)
            },
            r#implicit_tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(507..507 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#unknown523: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(523).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown524: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(524..524 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown528: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(528..528 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown532: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(532..532 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown536: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(536..536 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown540: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(540..540 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown544: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(544..544 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown548: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(548..548 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown552: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(552..552 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown556: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(556..556 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown560: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(560..560 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown564: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(564..564 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown568: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(568..568 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown572: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(572..572 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown576: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(576..576 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown580: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(580..580 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown584: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(584..584 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#buff_template: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(588..588 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffTemplatesRef::new(value as usize)
            },
            r#archnemesis_minion_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(604..604 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(612..612 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#unknown616: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(616..616 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
            r#unknown632: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(632..632 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown636: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(636..636 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| GrantedEffectsPerLevelRef::new(value as usize))
                    .collect()
            },
            r#radius_jewel_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(652..652 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_essence_only_modifier: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(656).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown657: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(657..657 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spawn_weight_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(661..661 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ModsRow {
    pub r#id: String,
    pub r#hash16: u16,
    pub r#mod_type: ModTypeRef,
    pub r#level: i32,
    pub r#stat1: StatsRef,
    pub r#stat2: StatsRef,
    pub r#stat3: StatsRef,
    pub r#stat4: StatsRef,
    pub r#domain: Option<ModDomains>,
    pub r#name: String,
    pub r#generation_type: Option<ModGenerationType>,
    pub r#families: Vec<ModFamilyRef>,
    pub r#stat1_value: (i32, i32),
    pub r#stat2_value: (i32, i32),
    pub r#stat3_value: (i32, i32),
    pub r#stat4_value: (i32, i32),
    pub r#spawn_weight_tags: Vec<TagsRef>,
    pub r#unknown174: Vec<i32>,
    pub r#tags: Vec<TagsRef>,
    pub r#granted_effects_per_level: Vec<GrantedEffectsPerLevelRef>,
    pub r#aura_flags: Vec<Option<ModAuraFlags>>,
    pub r#monster_metadata: String,
    pub r#monster_kill_achievements: Vec<AchievementItemsRef>,
    pub r#chest_mod_type: Vec<ModTypeRef>,
    pub r#stat5_value: (i32, i32),
    pub r#stat5: StatsRef,
    pub r#full_area_clear_achievement_items: Vec<AchievementItemsRef>,
    pub r#achievement_items: Vec<AchievementItemsRef>,
    pub r#generation_weight_tags: Vec<TagsRef>,
    pub r#generation_weight_values: Vec<i32>,
    pub r#modify_maps_achievements: Vec<AchievementItemsRef>,
    pub r#stat6_value: (i32, i32),
    pub r#stat6: StatsRef,
    pub r#max_level: i32,
    pub r#unknown410: bool,
    pub r#crafting_item_class_restrictions: Vec<ItemClassesRef>,
    pub r#monster_on_death: String,
    pub r#unknown435: i32,
    pub r#heist_achievements: Vec<AchievementItemsRef>,
    pub r#heist_sub_stat_value1: i32,
    pub r#heist_sub_stat_value2: i32,
    pub r#heist_stat0: StatsRef,
    pub r#heist_stat1: StatsRef,
    pub r#heist_add_stat_value1: i32,
    pub r#heist_add_stat_value2: i32,
    pub r#influence_type: Option<InfluenceTypes>,
    pub r#implicit_tags: Vec<TagsRef>,
    pub r#unknown523: bool,
    pub r#unknown524: i32,
    pub r#unknown528: i32,
    pub r#unknown532: i32,
    pub r#unknown536: i32,
    pub r#unknown540: i32,
    pub r#unknown544: i32,
    pub r#unknown548: i32,
    pub r#unknown552: i32,
    pub r#unknown556: i32,
    pub r#unknown560: i32,
    pub r#unknown564: i32,
    pub r#unknown568: i32,
    pub r#unknown572: i32,
    pub r#unknown576: i32,
    pub r#unknown580: i32,
    pub r#unknown584: i32,
    pub r#buff_template: BuffTemplatesRef,
    pub r#archnemesis_minion_mod: ModsRef,
    pub r#hash32: u32,
    pub r#unknown616: Vec<i64>,
    pub r#unknown632: i32,
    pub r#unknown636: Vec<GrantedEffectsPerLevelRef>,
    pub r#radius_jewel_type: i32,
    pub r#is_essence_only_modifier: bool,
    pub r#unknown657: i32,
    pub r#spawn_weight_values: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ModsRef(pub usize);

impl Deref for ModsRef {
    type Target = ModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Mods[self.0]
    }
}

impl ModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ModsRow {
        &TABLE_Mods[self.0]
    }
    pub fn get(&self) -> &'static ModsRow {
        &TABLE_Mods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Mods.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ModsRow)> {
        TABLE_Mods.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_Mods.iter() {
            println!("{:?}", row);
        }
    }
}
