#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterVarieties: LazyLock<Vec<MonsterVarietiesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monstervarieties.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterVarietiesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterTypesRef::new(value as usize)
            },
            r#movement_speed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#object_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#minimum_attack_distance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#maximum_attack_distance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#act_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#ao_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#base_monster_type_index: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(100..100 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(108..108 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#model_size_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown144: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(152..152 + 16).unwrap();
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
            r#experience_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown172: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(172..172 + 16).unwrap();
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
            r#min_agro_range: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(188..188 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_agro_range: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(192..192 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spotlight_colour1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(196..196 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spotlight_colour2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(200..200 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spotlight_colour3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(204..204 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#granted_effects: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(208..208 + 16).unwrap();
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
                    .map(|value| GrantedEffectsRef::new(value as usize))
                    .collect()
            },
            r#ais_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(224..224 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mods2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(232..232 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#stance: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(248..248 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown256: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(256..256 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(272..272 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#damage_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(280..280 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#life_multiplier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(284..284 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#attack_speed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(288..288 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#weapon1_item_visual_identity: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(292..292 + 16).unwrap();
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
                    .map(|value| ItemVisualIdentityRef::new(value as usize))
                    .collect()
            },
            r#weapon2_item_visual_identity: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(308..308 + 16).unwrap();
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
                    .map(|value| ItemVisualIdentityRef::new(value as usize))
                    .collect()
            },
            r#back_item_visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(324..324 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#main_hand_item_class: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(340..340 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#off_hand_item_class: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(356..356 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#helmet_item_visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(372..372 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#unknown388: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(388..388 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#kill_specific_monster_count_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(392..392 + 16).unwrap();
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
            r#special_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(408..408 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#kill_rare_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(424..424 + 16).unwrap();
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
            r#unknown440: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(440).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown441: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(441..441 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown445: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(445..445 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown449: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(449..449 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown453: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(453..453 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown457: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(457..457 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown461: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(461..461 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(465..465 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#unknown467: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(467).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown468: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(468..468 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#kill_while_onslaught_is_active_achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(476..476 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#monster_segment: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(492..492 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterSegmentsRef::new(value as usize)
            },
            r#monster_armour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(508..508 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterArmoursRef::new(value as usize)
            },
            r#kill_while_talisman_is_active_achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(524..524 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#part1_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(540..540 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#part2_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(556..556 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#endgame_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(572..572 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#unknown588: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(588..588 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown604: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(604..604 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown608: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(608..608 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown612: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(612..612 + 16).unwrap();
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
            r#multi_part_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(628..628 + 16).unwrap();
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
                    .map(|value| MultiPartAchievementsRef::new(value as usize))
                    .collect()
            },
            r#unknown644: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(644..644 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sink_animation_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(648..648 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown656: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(656).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown657: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(657..657 + 16).unwrap();
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
            r#unknown673: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(673).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown674: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(674).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown675: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(675).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown676: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(676..676 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown680: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(680..680 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown684: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(684..684 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#ailment_threshold: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(688..688 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sink_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(692..692 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown696: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(696..696 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_conditional_effect_pack: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(700..700 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterConditionalEffectPacksRef::new(value as usize)
            },
            r#unknown716: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(716).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown717: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(717).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown718: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(718..718 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown722: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(722).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown723: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(723..723 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown727: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(727..727 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown731: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(731..731 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown735: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(735..735 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown739: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(739..739 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown743: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(743..743 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown747: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(747..747 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown751: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(751..751 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown755: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(755..755 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown759: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(759..759 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown763: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(763..763 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown767: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(767..767 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#boss_health_bar: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(771).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown772: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(772..772 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown788: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(788..788 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown792: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(792).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown793: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(793..793 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown797: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(797..797 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown801: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(801..801 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown805: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(805..805 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown809: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(809).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown810: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(810..810 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown826: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(826..826 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown830: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(830..830 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown834: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(834).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown835: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(835..835 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown851: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(851..851 + 16).unwrap();
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
            r#unknown867: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(867..867 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown871: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(871..871 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown875: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(875..875 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown879: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(879..879 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown895: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(895).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown896: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(896..896 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown900: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(900).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown901: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(901..901 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#monster_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(905..905 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterCategoriesRef::new(value as usize)
            },
            r#unknown921: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(921..921 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown937: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(937..937 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown941: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(941..941 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown957: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(957).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown958: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(958..958 + 16).unwrap();
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
pub struct MonsterVarietiesRow {
    pub r#id: String,
    pub r#monster_type: MonsterTypesRef,
    pub r#movement_speed: i32,
    pub r#object_size: i32,
    pub r#minimum_attack_distance: i32,
    pub r#maximum_attack_distance: i32,
    pub r#act_files: Vec<String>,
    pub r#ao_files: Vec<String>,
    pub r#base_monster_type_index: String,
    pub r#mods: Vec<ModsRef>,
    pub r#unknown96: i32,
    pub r#unknown100: String,
    pub r#unknown108: String,
    pub r#model_size_multiplier: i32,
    pub r#unknown120: i32,
    pub r#unknown124: i32,
    pub r#unknown128: i64,
    pub r#unknown144: i32,
    pub r#unknown148: i32,
    pub r#tags: Vec<TagsRef>,
    pub r#experience_multiplier: i32,
    pub r#unknown172: Vec<i32>,
    pub r#min_agro_range: i32,
    pub r#max_agro_range: i32,
    pub r#spotlight_colour1: i32,
    pub r#spotlight_colour2: i32,
    pub r#spotlight_colour3: i32,
    pub r#granted_effects: Vec<GrantedEffectsRef>,
    pub r#ais_file: String,
    pub r#mods2: Vec<ModsRef>,
    pub r#stance: String,
    pub r#unknown256: i64,
    pub r#name: String,
    pub r#damage_multiplier: i32,
    pub r#life_multiplier: i32,
    pub r#attack_speed: i32,
    pub r#weapon1_item_visual_identity: Vec<ItemVisualIdentityRef>,
    pub r#weapon2_item_visual_identity: Vec<ItemVisualIdentityRef>,
    pub r#back_item_visual_identity: ItemVisualIdentityRef,
    pub r#main_hand_item_class: ItemClassesRef,
    pub r#off_hand_item_class: ItemClassesRef,
    pub r#helmet_item_visual_identity: ItemVisualIdentityRef,
    pub r#unknown388: i32,
    pub r#kill_specific_monster_count_achievement_items: Vec<AchievementItemsRef>,
    pub r#special_mods: Vec<ModsRef>,
    pub r#kill_rare_achievement_items: Vec<AchievementItemsRef>,
    pub r#unknown440: bool,
    pub r#unknown441: i32,
    pub r#unknown445: i32,
    pub r#unknown449: i32,
    pub r#unknown453: i32,
    pub r#unknown457: i32,
    pub r#unknown461: i32,
    pub r#hash16: u16,
    pub r#unknown467: bool,
    pub r#unknown468: String,
    pub r#kill_while_onslaught_is_active_achievement_item: AchievementItemsRef,
    pub r#monster_segment: MonsterSegmentsRef,
    pub r#monster_armour: MonsterArmoursRef,
    pub r#kill_while_talisman_is_active_achievement_item: AchievementItemsRef,
    pub r#part1_mods: Vec<ModsRef>,
    pub r#part2_mods: Vec<ModsRef>,
    pub r#endgame_mods: Vec<ModsRef>,
    pub r#unknown588: i64,
    pub r#unknown604: i32,
    pub r#unknown608: i32,
    pub r#unknown612: Vec<i64>,
    pub r#multi_part_achievements: Vec<MultiPartAchievementsRef>,
    pub r#unknown644: i32,
    pub r#sink_animation_ao_file: String,
    pub r#unknown656: bool,
    pub r#unknown657: Vec<i64>,
    pub r#unknown673: bool,
    pub r#unknown674: bool,
    pub r#unknown675: bool,
    pub r#unknown676: i32,
    pub r#unknown680: i32,
    pub r#unknown684: f32,
    pub r#ailment_threshold: i32,
    pub r#sink_effect: i32,
    pub r#unknown696: i32,
    pub r#monster_conditional_effect_pack: MonsterConditionalEffectPacksRef,
    pub r#unknown716: bool,
    pub r#unknown717: bool,
    pub r#unknown718: i32,
    pub r#unknown722: bool,
    pub r#unknown723: i32,
    pub r#unknown727: i32,
    pub r#unknown731: i32,
    pub r#unknown735: i32,
    pub r#unknown739: i32,
    pub r#unknown743: i32,
    pub r#unknown747: i32,
    pub r#unknown751: i32,
    pub r#unknown755: i32,
    pub r#unknown759: i32,
    pub r#unknown763: i32,
    pub r#unknown767: i32,
    pub r#boss_health_bar: bool,
    pub r#unknown772: i64,
    pub r#unknown788: i32,
    pub r#unknown792: bool,
    pub r#unknown793: i32,
    pub r#unknown797: i32,
    pub r#unknown801: i32,
    pub r#unknown805: i32,
    pub r#unknown809: bool,
    pub r#unknown810: i64,
    pub r#unknown826: f32,
    pub r#unknown830: f32,
    pub r#unknown834: bool,
    pub r#unknown835: (usize, usize),
    pub r#unknown851: Vec<i64>,
    pub r#unknown867: i32,
    pub r#unknown871: i32,
    pub r#unknown875: f32,
    pub r#unknown879: i64,
    pub r#unknown895: bool,
    pub r#unknown896: i32,
    pub r#unknown900: bool,
    pub r#unknown901: i32,
    pub r#monster_category: MonsterCategoriesRef,
    pub r#unknown921: i64,
    pub r#unknown937: i32,
    pub r#unknown941: i64,
    pub r#unknown957: bool,
    pub r#unknown958: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterVarietiesRef(pub usize);

impl Deref for MonsterVarietiesRef {
    type Target = MonsterVarietiesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterVarieties[self.0]
    }
}

impl MonsterVarietiesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterVarietiesRow {
        &TABLE_MonsterVarieties[self.0]
    }
    pub fn get(&self) -> &'static MonsterVarietiesRow {
        &TABLE_MonsterVarieties[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterVarieties
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterVarietiesRow)> {
        TABLE_MonsterVarieties
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MonsterVarieties.iter() {
            black_box(row);
        }
    }
}
