#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_QuestStates: LazyLock<Vec<QuestStatesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/queststates.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| QuestStatesRow {
            r#quest: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRef::new(value as usize)
            },
            r#order: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#flags_present: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
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
                    .map(|value| QuestFlagsRef::new(value as usize))
                    .collect()
            },
            r#flags_missing: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
                    .map(|value| QuestFlagsRef::new(value as usize))
                    .collect()
            },
            r#text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#message: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(61..61 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#map_pins_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(69..69 + 16).unwrap();
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
                    .map(|value| MapPinsRef::new(value as usize))
                    .collect()
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#map_pins_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(89..89 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#map_pins_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MapPinsRef::new(value as usize)
            },
            r#unknown113: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
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
            r#unknown129: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(129).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown130: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(130..130 + 16).unwrap();
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
            r#unknown146: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
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
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#sound_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(166..166 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SoundEffectsRef::new(value as usize)
            },
            r#unknown182: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(182..182 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown190: {
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
            },
            r#unknown206: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(206).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown207: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(207).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct QuestStatesRow {
    pub r#quest: QuestRef,
    pub r#order: i32,
    pub r#flags_present: Vec<QuestFlagsRef>,
    pub r#flags_missing: Vec<QuestFlagsRef>,
    pub r#text: String,
    pub r#unknown60: bool,
    pub r#message: String,
    pub r#map_pins_keys: Vec<MapPinsRef>,
    pub r#unknown85: i32,
    pub r#map_pins_text: String,
    pub r#map_pins_key: MapPinsRef,
    pub r#unknown113: Vec<i64>,
    pub r#unknown129: bool,
    pub r#unknown130: Vec<i32>,
    pub r#unknown146: Vec<i32>,
    pub r#unknown162: i32,
    pub r#sound_effect: SoundEffectsRef,
    pub r#unknown182: String,
    pub r#unknown190: Vec<i64>,
    pub r#unknown206: bool,
    pub r#unknown207: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct QuestStatesRef(pub usize);

impl Deref for QuestStatesRef {
    type Target = QuestStatesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_QuestStates[self.0]
    }
}

impl QuestStatesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static QuestStatesRow {
        &TABLE_QuestStates[self.0]
    }
    pub fn get(&self) -> &'static QuestStatesRow {
        &TABLE_QuestStates[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_QuestStates.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static QuestStatesRow)> {
        TABLE_QuestStates
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
        for row in TABLE_QuestStates.iter() {
            black_box(row);
        }
    }
}
