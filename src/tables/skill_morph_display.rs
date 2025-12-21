#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillMorphDisplay: LazyLock<Vec<SkillMorphDisplayRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skillmorphdisplay.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillMorphDisplayRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
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
                values
            },
            r#dds_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
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
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_f32_le())
                    .collect::<Vec<f32>>();
                values
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown89: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(89).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillMorphDisplayRow {
    pub r#unknown0: i64,
    pub r#unknown16: Vec<i64>,
    pub r#dds_files: Vec<String>,
    pub r#unknown48: i32,
    pub r#unknown52: Vec<i32>,
    pub r#unknown68: i32,
    pub r#unknown72: Vec<f32>,
    pub r#unknown88: bool,
    pub r#unknown89: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillMorphDisplayRef(pub usize);

impl Deref for SkillMorphDisplayRef {
    type Target = SkillMorphDisplayRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillMorphDisplay[self.0]
    }
}

impl SkillMorphDisplayRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillMorphDisplayRow {
        &TABLE_SkillMorphDisplay[self.0]
    }
    pub fn get(&self) -> &'static SkillMorphDisplayRow {
        &TABLE_SkillMorphDisplay[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillMorphDisplay
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillMorphDisplayRow)> {
        TABLE_SkillMorphDisplay
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
        for row in TABLE_SkillMorphDisplay.iter() {
            black_box(row);
        }
    }
}
