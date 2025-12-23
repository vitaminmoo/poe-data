#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkills: LazyLock<Vec<PassiveSkillsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/passiveskills.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| PassiveSkillsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#stat1_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat2_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat3_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat4_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#passive_skill_graph_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(50..50 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#characters: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(58..58 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| CharactersRef::new(value as usize)).collect()
            },
            r#is_keystone: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(74).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_notable: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(75).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(76..76 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_just_icon: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(84).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#is_jewel_socket: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(101).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#ascendancy: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(102..102 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AscendancyRef::new(value as usize)
            },
            r#is_ascendancy_starting_node: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(118).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#reminder_strings: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(119..119 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ReminderTextRef::new(value as usize)).collect()
            },
            r#skill_points_granted: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(135..135 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_multiple_choice: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(139).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_multiple_choice_option: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(140).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stat5_value: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(141..141 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#passive_skill_buffs: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BuffTemplatesRef::new(value as usize)).collect()
            },
            r#is_anointment_only: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(161).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_expansion: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(166).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_proxy_passive: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(167).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#skill_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                PassiveSkillTypes::from_repr(value as usize)
            },
            r#mastery_group: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(172..172 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillMasteryGroupsRef::new(value as usize)
            },
            r#group: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(188..188 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AtlasPassiveSkillTreeGroupTypeRef::new(value as usize)
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(204..204 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#atlasnode_group: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(220..220 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown228: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(228..228 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown232: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(232..232 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown236: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(236..236 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown240: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(240..240 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown244: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(244..244 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown248: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(248).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown249: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(249..249 + 16).unwrap();
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
            r#unknown265: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(265..265 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown269: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(269..269 + 16).unwrap();
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
            r#unknown285: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(285).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#keystone_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(286..286 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown302: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(302).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_attribute: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(303).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#atlas_sub_tree: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(304..304 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AtlasPassiveSkillSubTreesRef::new(value as usize)
            },
            r#is_root_of_atlas_tree: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(320).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#granted_skill: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(321..321 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SkillGemsRef::new(value as usize)
            },
            r#weapon_points_granted: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(337..337 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_free: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(341).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown342: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(342).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unlocked_by: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(343..343 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| PassiveSkillsRef::new(value as usize)).collect()
            },
            r#visible_for_ascendancy: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(359..359 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AscendancyRef::new(value as usize)
            },
            r#unknown375: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(375).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#node_frame_art: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(376..376 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillsRow {
    pub r#id: String,
    pub r#icon_dds_file: String,
    pub r#stats: Vec<StatsRef>,
    pub r#stat1_value: i32,
    pub r#stat2_value: i32,
    pub r#stat3_value: i32,
    pub r#stat4_value: i32,
    pub r#passive_skill_graph_id: u16,
    pub r#name: String,
    pub r#characters: Vec<CharactersRef>,
    pub r#is_keystone: bool,
    pub r#is_notable: bool,
    pub r#flavour_text: String,
    pub r#is_just_icon: bool,
    pub r#achievement_item: AchievementItemsRef,
    pub r#is_jewel_socket: bool,
    pub r#ascendancy: AscendancyRef,
    pub r#is_ascendancy_starting_node: bool,
    pub r#reminder_strings: Vec<ReminderTextRef>,
    pub r#skill_points_granted: i32,
    pub r#is_multiple_choice: bool,
    pub r#is_multiple_choice_option: bool,
    pub r#stat5_value: i32,
    pub r#passive_skill_buffs: Vec<BuffTemplatesRef>,
    pub r#is_anointment_only: bool,
    pub r#unknown162: i32,
    pub r#is_expansion: bool,
    pub r#is_proxy_passive: bool,
    pub r#skill_type: Option<PassiveSkillTypes>,
    pub r#mastery_group: PassiveSkillMasteryGroupsRef,
    pub r#group: AtlasPassiveSkillTreeGroupTypeRef,
    pub r#sound_effect: SoundEffectsRef,
    pub r#atlasnode_group: String,
    pub r#unknown228: i32,
    pub r#unknown232: i32,
    pub r#unknown236: i32,
    pub r#unknown240: i32,
    pub r#unknown244: i32,
    pub r#unknown248: bool,
    pub r#unknown249: Vec<i64>,
    pub r#unknown265: i32,
    pub r#unknown269: Vec<i64>,
    pub r#unknown285: bool,
    pub r#keystone_id: i64,
    pub r#unknown302: bool,
    pub r#is_attribute: bool,
    pub r#atlas_sub_tree: AtlasPassiveSkillSubTreesRef,
    pub r#is_root_of_atlas_tree: bool,
    pub r#granted_skill: SkillGemsRef,
    pub r#weapon_points_granted: i32,
    pub r#is_free: bool,
    pub r#unknown342: bool,
    pub r#unlocked_by: Vec<PassiveSkillsRef>,
    pub r#visible_for_ascendancy: AscendancyRef,
    pub r#unknown375: bool,
    pub r#node_frame_art: PassiveSkillTreeNodeFrameArtRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillsRef(pub usize);

impl Deref for PassiveSkillsRef {
    type Target = PassiveSkillsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkills[self.0]
    }
}

impl PassiveSkillsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillsRow {
        &TABLE_PassiveSkills[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillsRow {
        &TABLE_PassiveSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkills.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillsRow)> {
        TABLE_PassiveSkills.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkills.iter() {
            black_box(row);
        }
    }
}
