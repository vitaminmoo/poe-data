#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternatePassiveSkills: LazyLock<Vec<AlternatePassiveSkillsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/alternatepassiveskills.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AlternatePassiveSkillsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#alternate_tree_version: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AlternateTreeVersionsRef::new(value as usize)
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#passive_type: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            r#stat1: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat2: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat3: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat4: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat5: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#stat6: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(104..104 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#conqueror_index: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#random: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(120..120 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(128..128 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dds_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(136..136 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(144..144 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
            r#conqueror_version: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(160..160 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#conqueror_spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(164..164 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AlternatePassiveSkillsRow {
    pub r#id: String,
    pub r#alternate_tree_version: AlternateTreeVersionsRef,
    pub r#name: String,
    pub r#passive_type: Vec<i32>,
    pub r#stats: Vec<StatsRef>,
    pub r#stat1: (i32, i32),
    pub r#stat2: (i32, i32),
    pub r#stat3: (i32, i32),
    pub r#stat4: (i32, i32),
    pub r#stat5: (i32, i32),
    pub r#stat6: (i32, i32),
    pub r#spawn_weight: i32,
    pub r#conqueror_index: i32,
    pub r#random: (i32, i32),
    pub r#flavour_text: String,
    pub r#dds_icon: String,
    pub r#achievement_items: Vec<AchievementItemsRef>,
    pub r#conqueror_version: i32,
    pub r#conqueror_spawn_weight: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternatePassiveSkillsRef(pub usize);

impl Deref for AlternatePassiveSkillsRef {
    type Target = AlternatePassiveSkillsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternatePassiveSkills[self.0]
    }
}

impl AlternatePassiveSkillsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternatePassiveSkillsRow {
        &TABLE_AlternatePassiveSkills[self.0]
    }
    pub fn get(&self) -> &'static AlternatePassiveSkillsRow {
        &TABLE_AlternatePassiveSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternatePassiveSkills.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternatePassiveSkillsRow)> {
        TABLE_AlternatePassiveSkills.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AlternatePassiveSkills.iter() {
            black_box(row);
        }
    }
}
