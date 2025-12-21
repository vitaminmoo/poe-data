#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCDialogueStyles: LazyLock<Vec<NPCDialogueStylesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/npcdialoguestyles.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| NPCDialogueStylesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#header_base_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#buttom_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#banner_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#header_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
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
                // array_mutator column.array == true
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
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
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCDialogueStylesRef::new(value as usize)
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(100..100 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown108: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(108..108 + 16).unwrap();
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
            r#unknown124: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(124..124 + 16).unwrap();
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
            r#unknown140: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(140..140 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown144: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(148..148 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown156: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(156..156 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(164..164 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown168: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(168..168 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown172: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(172..172 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown176: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(176).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown177: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(177..177 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCDialogueStylesRow {
    pub r#id: String,
    pub r#header_base_file: String,
    pub r#buttom_file: String,
    pub r#banner_files: Vec<String>,
    pub r#header_files: Vec<String>,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
    pub r#unknown76: Vec<i32>,
    pub r#unknown92: NPCDialogueStylesRef,
    pub r#unknown100: String,
    pub r#unknown108: Vec<i32>,
    pub r#unknown124: Vec<i32>,
    pub r#unknown140: i32,
    pub r#unknown144: i32,
    pub r#unknown148: String,
    pub r#unknown156: String,
    pub r#unknown164: i32,
    pub r#unknown168: i32,
    pub r#unknown172: i32,
    pub r#unknown176: bool,
    pub r#unknown177: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCDialogueStylesRef(pub usize);

impl Deref for NPCDialogueStylesRef {
    type Target = NPCDialogueStylesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCDialogueStyles[self.0]
    }
}

impl NPCDialogueStylesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCDialogueStylesRow {
        &TABLE_NPCDialogueStyles[self.0]
    }
    pub fn get(&self) -> &'static NPCDialogueStylesRow {
        &TABLE_NPCDialogueStyles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCDialogueStyles
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCDialogueStylesRow)> {
        TABLE_NPCDialogueStyles
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
        for row in TABLE_NPCDialogueStyles.iter() {
            black_box(row);
        }
    }
}
