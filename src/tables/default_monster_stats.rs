#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DefaultMonsterStats: LazyLock<Vec<DefaultMonsterStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/defaultmonsterstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DefaultMonsterStatsRow {
            r#display_level: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#damage: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#evasion: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#accuracy: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#life: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#experience: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#ally_life: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#armour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#difficulty: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#minion_damage: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#alt_life1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#alt_damage1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#alt_damage2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#alt_life2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#evasive_evasion: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#ailment_threshold: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#stun_threshold: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DefaultMonsterStatsRow {
    pub r#display_level: String,
    pub r#damage: f32,
    pub r#evasion: i32,
    pub r#accuracy: i32,
    pub r#life: i32,
    pub r#experience: i32,
    pub r#ally_life: i32,
    pub r#armour: i32,
    pub r#difficulty: i32,
    pub r#minion_damage: f32,
    pub r#alt_life1: i32,
    pub r#alt_damage1: f32,
    pub r#alt_damage2: f32,
    pub r#alt_life2: i32,
    pub r#evasive_evasion: i32,
    pub r#ailment_threshold: i32,
    pub r#stun_threshold: i32,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#unknown80: i32,
    pub r#unknown84: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DefaultMonsterStatsRef(pub usize);

impl Deref for DefaultMonsterStatsRef {
    type Target = DefaultMonsterStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DefaultMonsterStats[self.0]
    }
}

impl DefaultMonsterStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DefaultMonsterStatsRow {
        &TABLE_DefaultMonsterStats[self.0]
    }
    pub fn get(&self) -> &'static DefaultMonsterStatsRow {
        &TABLE_DefaultMonsterStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DefaultMonsterStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DefaultMonsterStatsRow)> {
        TABLE_DefaultMonsterStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DefaultMonsterStats.iter() {
            black_box(row);
        }
    }
}
