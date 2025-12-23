#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionAreas: LazyLock<Vec<ExpeditionAreasRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/expeditionareas.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ExpeditionAreasRow {
            r#area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#pos_x: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#pos_y: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tags: {
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
                values.into_iter().map(|value| TagsRef::new(value as usize)).collect()
            },
            r#unknown40: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#text_audio: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#completion_achievements: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionAreasRow {
    pub r#area: WorldAreasRef,
    pub r#pos_x: i32,
    pub r#pos_y: i32,
    pub r#tags: Vec<TagsRef>,
    pub r#unknown40: Vec<i32>,
    pub r#unknown56: bool,
    pub r#text_audio: NPCTextAudioRef,
    pub r#completion_achievements: Vec<AchievementItemsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionAreasRef(pub usize);

impl Deref for ExpeditionAreasRef {
    type Target = ExpeditionAreasRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionAreas[self.0]
    }
}

impl ExpeditionAreasRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionAreasRow {
        &TABLE_ExpeditionAreas[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionAreasRow {
        &TABLE_ExpeditionAreas[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionAreas.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionAreasRow)> {
        TABLE_ExpeditionAreas.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ExpeditionAreas.iter() {
            black_box(row);
        }
    }
}
