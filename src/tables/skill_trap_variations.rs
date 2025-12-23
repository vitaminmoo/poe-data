#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillTrapVariations: LazyLock<Vec<SkillTrapVariationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/skilltrapvariations.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SkillTrapVariationsRow {
            r#id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#metadata: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(4..4 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SkillTrapVariationsRow {
    pub r#id: i32,
    pub r#metadata: String,
    pub r#misc_animated: MiscAnimatedRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillTrapVariationsRef(pub usize);

impl Deref for SkillTrapVariationsRef {
    type Target = SkillTrapVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillTrapVariations[self.0]
    }
}

impl SkillTrapVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillTrapVariationsRow {
        &TABLE_SkillTrapVariations[self.0]
    }
    pub fn get(&self) -> &'static SkillTrapVariationsRow {
        &TABLE_SkillTrapVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillTrapVariations.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillTrapVariationsRow)> {
        TABLE_SkillTrapVariations.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_SkillTrapVariations.iter() {
            black_box(row);
        }
    }
}
