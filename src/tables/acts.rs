#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Acts: LazyLock<Vec<ActsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/acts.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ui_title: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#act_number: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_end_game: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(41..41 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown45: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
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
            r#unknown61: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(61..61 + 16).unwrap();
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
            r#unknown77: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(77..77 + 16).unwrap();
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
            r#unknown93: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown109: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(109..109 + 16).unwrap();
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
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(125..125 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ActsRow {
    pub r#id: String,
    pub r#ui_title: String,
    pub r#act_number: i32,
    pub r#unknown20: String,
    pub r#unknown28: String,
    pub r#unknown36: i32,
    pub r#is_end_game: bool,
    pub r#unknown41: i32,
    pub r#unknown45: Vec<i64>,
    pub r#unknown61: Vec<i64>,
    pub r#unknown77: Vec<i64>,
    pub r#unknown93: i64,
    pub r#unknown109: Vec<i64>,
    pub r#description: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActsRef(pub usize);

impl Deref for ActsRef {
    type Target = ActsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Acts[self.0]
    }
}

impl ActsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn get(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Acts.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActsRow)> {
        TABLE_Acts.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Acts.iter() {
            black_box(row);
        }
    }
}
