#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffTemplates: LazyLock<Vec<BuffTemplatesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/bufftemplates.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BuffTemplatesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#buff_definition: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
            },
            r#buff_stat_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
            r#aura_radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
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
            r#unknown60: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
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
            r#buff_visual: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
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
            r#unknown113: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(121).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown122: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(122..122 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown138: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(138).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffTemplatesRow {
    pub r#id: String,
    pub r#buff_definition: BuffDefinitionsRef,
    pub r#buff_stat_values: Vec<i32>,
    pub r#aura_radius: i32,
    pub r#unknown44: Vec<i32>,
    pub r#unknown60: Vec<i32>,
    pub r#buff_visual: BuffVisualsRef,
    pub r#unknown92: f32,
    pub r#unknown96: bool,
    pub r#stats: Vec<StatsRef>,
    pub r#unknown113: i32,
    pub r#unknown117: i32,
    pub r#unknown121: bool,
    pub r#unknown122: StatsRef,
    pub r#unknown138: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffTemplatesRef(pub usize);

impl Deref for BuffTemplatesRef {
    type Target = BuffTemplatesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffTemplates[self.0]
    }
}

impl BuffTemplatesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffTemplatesRow {
        &TABLE_BuffTemplates[self.0]
    }
    pub fn get(&self) -> &'static BuffTemplatesRow {
        &TABLE_BuffTemplates[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffTemplates.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffTemplatesRow)> {
        TABLE_BuffTemplates
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
        for row in TABLE_BuffTemplates.iter() {
            black_box(row);
        }
    }
}
