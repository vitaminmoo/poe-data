#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ActiveSkills: LazyLock<Vec<ActiveSkillsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/activeskills.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActiveSkillsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#displayed_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#action_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ActionTypesRef::new(value as usize)
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_skill_target_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
                    .map(|value| ActiveSkillTargetTypes::from_repr(value as usize))
                    .collect()
            },
            r#active_skill_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#website_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#website_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hide_on_website: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#granted_effect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(97..97 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(105).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#skill_totem_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                SkillTotems::from_repr(value as usize)
            },
            r#is_manually_casted: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(110).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#input_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(111..111 + 16).unwrap();
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
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#output_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(127..127 + 16).unwrap();
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
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#minion_active_skill_types: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(143..143 + 16).unwrap();
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
                    .map(|value| ActiveSkillTypeRef::new(value as usize))
                    .collect()
            },
            r#unknown159: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(159).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_gem: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(160).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#secondary_skill_specific_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(161..161 + 16).unwrap();
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
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#skill_mine: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(177..177 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#alternate_skill_targeting_behaviour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(181..181 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AlternateSkillTargetingBehavioursRef::new(value as usize)
            },
            r#unknown197: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(197).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#ai_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(198..198 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stat_context_flags: {
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
                    .map(|value| VirtualStatContextFlagsRef::new(value as usize))
                    .collect()
            },
            r#unknown222: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(222).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown223: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(223).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown224: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(224).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#shape_shift_form: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(225..225 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ShapeShiftFormsRef::new(value as usize)
            },
            r#video_clip: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(241..241 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#character_audio_event: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(249..249 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharacterAudioEventsRef::new(value as usize)
            },
            r#ai_script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(265..265 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown273: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(273..273 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown289: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(289).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#short_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(290..290 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stat_description_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(298..298 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stat_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(302..302 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#weapon_requirements: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(310..310 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ActiveSkillWeaponRequirementRef::new(value as usize)
            },
            r#unknown326: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(326..326 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ActiveSkillsRow {
    pub r#id: String,
    pub r#displayed_name: String,
    pub r#description: String,
    pub r#action_type: ActionTypesRef,
    pub r#icon_dds_file: String,
    pub r#active_skill_target_types: Vec<Option<ActiveSkillTargetTypes>>,
    pub r#active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#website_description: String,
    pub r#website_image: String,
    pub r#hide_on_website: bool,
    pub r#granted_effect: String,
    pub r#unknown105: bool,
    pub r#skill_totem_id: Option<SkillTotems>,
    pub r#is_manually_casted: bool,
    pub r#input_stats: Vec<StatsRef>,
    pub r#output_stats: Vec<StatsRef>,
    pub r#minion_active_skill_types: Vec<ActiveSkillTypeRef>,
    pub r#unknown159: bool,
    pub r#is_gem: bool,
    pub r#secondary_skill_specific_stats: Vec<StatsRef>,
    pub r#skill_mine: i32,
    pub r#alternate_skill_targeting_behaviour: AlternateSkillTargetingBehavioursRef,
    pub r#unknown197: bool,
    pub r#ai_file: String,
    pub r#stat_context_flags: Vec<VirtualStatContextFlagsRef>,
    pub r#unknown222: bool,
    pub r#unknown223: bool,
    pub r#unknown224: bool,
    pub r#shape_shift_form: ShapeShiftFormsRef,
    pub r#video_clip: String,
    pub r#character_audio_event: CharacterAudioEventsRef,
    pub r#ai_script: String,
    pub r#unknown273: i64,
    pub r#unknown289: bool,
    pub r#short_description: String,
    pub r#stat_description_type: i32,
    pub r#stat_description: String,
    pub r#weapon_requirements: ActiveSkillWeaponRequirementRef,
    pub r#unknown326: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActiveSkillsRef(pub usize);

impl Deref for ActiveSkillsRef {
    type Target = ActiveSkillsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActiveSkills[self.0]
    }
}

impl ActiveSkillsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActiveSkillsRow {
        &TABLE_ActiveSkills[self.0]
    }
    pub fn get(&self) -> &'static ActiveSkillsRow {
        &TABLE_ActiveSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActiveSkills.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActiveSkillsRow)> {
        TABLE_ActiveSkills
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
        for row in TABLE_ActiveSkills.iter() {
            black_box(row);
        }
    }
}
