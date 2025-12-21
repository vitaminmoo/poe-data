#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapPins: LazyLock<Vec<MapPinsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/mappins.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MapPinsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#world_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#world_areas_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
                    .map(|value| WorldAreasRef::new(value as usize))
                    .collect()
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#quest_flags1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
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
            r#act: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#quest_flags2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
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
            r#quest_flags3: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
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
            r#quest_flag1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(108..108 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(124..124 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#parent_map_pin: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                MapPinsRef::new(value as usize)
            },
            r#metadata_id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(140..140 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#quest_flag2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(164..164 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown180: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(180..180 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown188: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(188).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown189: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(189..189 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown193: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(193..193 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown197: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(197).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapPinsRow {
    pub r#id: String,
    pub r#world_area: WorldAreasRef,
    pub r#world_areas_keys: Vec<WorldAreasRef>,
    pub r#name: String,
    pub r#flavour_text: String,
    pub r#quest_flags1: Vec<QuestFlagsRef>,
    pub r#act: i32,
    pub r#quest_flags2: Vec<QuestFlagsRef>,
    pub r#quest_flags3: Vec<QuestFlagsRef>,
    pub r#quest_flag1: QuestFlagsRef,
    pub r#unknown124: String,
    pub r#parent_map_pin: MapPinsRef,
    pub r#metadata_id: String,
    pub r#unknown148: i64,
    pub r#quest_flag2: QuestFlagsRef,
    pub r#unknown180: String,
    pub r#unknown188: bool,
    pub r#unknown189: i32,
    pub r#unknown193: i32,
    pub r#unknown197: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapPinsRef(pub usize);

impl Deref for MapPinsRef {
    type Target = MapPinsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapPins[self.0]
    }
}

impl MapPinsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapPinsRow {
        &TABLE_MapPins[self.0]
    }
    pub fn get(&self) -> &'static MapPinsRow {
        &TABLE_MapPins[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapPins.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapPinsRow)> {
        TABLE_MapPins.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapPins.iter() {
            black_box(row);
        }
    }
}
