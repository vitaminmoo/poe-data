#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemVisualIdentity: LazyLock<Vec<ItemVisualIdentityRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemvisualidentity.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemVisualIdentityRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#inventory_sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#ao_file2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(42..42 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#marauder_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(50..50 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#ranger_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(66..66 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#witch_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(82..82 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#duelist_dex_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(98..98 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#templar_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(114..114 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#shadow_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(130..130 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#scion_sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#marauder_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(162..162 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ranger_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(170..170 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#witch_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(178..178 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#duelist_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(186..186 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#templar_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(194..194 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#shadow_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(202..202 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#scion_shape: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(210..210 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown218: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(218..218 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown234: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(234..234 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#pickup_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(238..238 + 16).unwrap();
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
            r#sm_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(254..254 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#identify_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(270..270 + 16).unwrap();
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
            r#epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(286..286 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#corrupt_achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(294..294 + 16).unwrap();
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
            r#is_alternate_art: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(310).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown311: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(311).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#create_corrupted_jewel_achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(312..312 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#animation_location: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(328..328 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown336: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(336..336 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown344: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(344..344 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown352: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(352..352 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown360: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(360..360 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown368: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(368..368 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown376: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(376..376 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown384: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(384..384 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown392: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(392..392 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown400: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(400..400 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown408: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(408..408 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown416: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(416..416 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown424: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(424..424 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_atlas_of_worlds_map_icon: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(432).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_tier16_icon: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(433).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown434: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(434..434 + 16).unwrap();
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
            r#unknown450: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(450).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown451: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(451..451 + 16).unwrap();
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
            r#unknown467: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(467..467 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown483: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(483..483 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown499: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(499..499 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown515: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(515..515 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown531: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(531..531 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown547: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(547..547 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown563: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(563..563 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown579: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(579..579 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown587: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(587..587 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown595: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(595..595 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown603: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(603..603 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown611: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(611..611 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown619: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(619..619 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown627: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(627..627 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown635: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(635..635 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown643: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(643..643 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#composition: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(651..651 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown655: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(655..655 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown671: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(671..671 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown687: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(687..687 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown703: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(703..703 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown719: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(719..719 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown735: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(735..735 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown751: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(751..751 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown767: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(767..767 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown783: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(783..783 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown799: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(799..799 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown815: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(815..815 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown831: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(831..831 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown839: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(839..839 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown847: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(847..847 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown855: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(855..855 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown863: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(863..863 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown871: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(871..871 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown879: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(879..879 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown887: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(887..887 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown903: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(903..903 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown919: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(919..919 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown935: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(935..935 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown951: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(951..951 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemVisualIdentityRow {
    pub r#id: String,
    pub r#dds_file: String,
    pub r#ao_file: String,
    pub r#inventory_sound_effect: SoundEffectsRef,
    pub r#hash16: u16,
    pub r#ao_file2: String,
    pub r#marauder_sm_files: Vec<String>,
    pub r#ranger_sm_files: Vec<String>,
    pub r#witch_sm_files: Vec<String>,
    pub r#duelist_dex_sm_files: Vec<String>,
    pub r#templar_sm_files: Vec<String>,
    pub r#shadow_sm_files: Vec<String>,
    pub r#scion_sm_files: Vec<String>,
    pub r#marauder_shape: String,
    pub r#ranger_shape: String,
    pub r#witch_shape: String,
    pub r#duelist_shape: String,
    pub r#templar_shape: String,
    pub r#shadow_shape: String,
    pub r#scion_shape: String,
    pub r#unknown218: i64,
    pub r#unknown234: i32,
    pub r#pickup_achievement_items: Vec<AchievementItemsRef>,
    pub r#sm_files: Vec<String>,
    pub r#identify_achievement_items: Vec<AchievementItemsRef>,
    pub r#epk_file: String,
    pub r#corrupt_achievement_items: Vec<AchievementItemsRef>,
    pub r#is_alternate_art: bool,
    pub r#unknown311: bool,
    pub r#create_corrupted_jewel_achievement_item: AchievementItemsRef,
    pub r#animation_location: String,
    pub r#unknown336: String,
    pub r#unknown344: String,
    pub r#unknown352: String,
    pub r#unknown360: String,
    pub r#unknown368: String,
    pub r#unknown376: String,
    pub r#unknown384: String,
    pub r#unknown392: String,
    pub r#unknown400: String,
    pub r#unknown408: String,
    pub r#unknown416: String,
    pub r#unknown424: String,
    pub r#is_atlas_of_worlds_map_icon: bool,
    pub r#is_tier16_icon: bool,
    pub r#unknown434: Vec<i32>,
    pub r#unknown450: bool,
    pub r#unknown451: Vec<i64>,
    pub r#unknown467: Vec<String>,
    pub r#unknown483: Vec<String>,
    pub r#unknown499: Vec<String>,
    pub r#unknown515: Vec<String>,
    pub r#unknown531: Vec<String>,
    pub r#unknown547: Vec<String>,
    pub r#unknown563: Vec<String>,
    pub r#unknown579: String,
    pub r#unknown587: String,
    pub r#unknown595: String,
    pub r#unknown603: String,
    pub r#unknown611: String,
    pub r#unknown619: String,
    pub r#unknown627: String,
    pub r#unknown635: String,
    pub r#unknown643: String,
    pub r#composition: i32,
    pub r#unknown655: i64,
    pub r#unknown671: i64,
    pub r#unknown687: i64,
    pub r#unknown703: i64,
    pub r#unknown719: Vec<String>,
    pub r#unknown735: Vec<String>,
    pub r#unknown751: Vec<String>,
    pub r#unknown767: Vec<String>,
    pub r#unknown783: Vec<String>,
    pub r#unknown799: Vec<String>,
    pub r#unknown815: i64,
    pub r#unknown831: String,
    pub r#unknown839: String,
    pub r#unknown847: String,
    pub r#unknown855: String,
    pub r#unknown863: String,
    pub r#unknown871: String,
    pub r#unknown879: String,
    pub r#unknown887: i64,
    pub r#unknown903: i64,
    pub r#unknown919: i64,
    pub r#unknown935: i64,
    pub r#unknown951: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemVisualIdentityRef(pub usize);

impl Deref for ItemVisualIdentityRef {
    type Target = ItemVisualIdentityRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemVisualIdentity[self.0]
    }
}

impl ItemVisualIdentityRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemVisualIdentityRow {
        &TABLE_ItemVisualIdentity[self.0]
    }
    pub fn get(&self) -> &'static ItemVisualIdentityRow {
        &TABLE_ItemVisualIdentity[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemVisualIdentity
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemVisualIdentityRow)> {
        TABLE_ItemVisualIdentity
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
        for row in TABLE_ItemVisualIdentity.iter() {
            black_box(row);
        }
    }
}
