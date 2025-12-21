#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SupportGems: LazyLock<Vec<SupportGemsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/supportgems.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SupportGemsRow {
            r#skill_gem: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SkillGemsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#family: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
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
                    .map(|value| SupportGemFamilyRef::new(value as usize))
                    .collect()
            },
            r#is_lineage: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                FlavourTextRef::new(value as usize)
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(77..77 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                SupportGemSocketedVisualIdentityRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SupportGemsRow {
    pub r#skill_gem: SkillGemsRef,
    pub r#unknown16: i32,
    pub r#icon: String,
    pub r#family: Vec<SupportGemFamilyRef>,
    pub r#is_lineage: bool,
    pub r#flavour_text: FlavourTextRef,
    pub r#unknown61: i64,
    pub r#visual_identity: SupportGemSocketedVisualIdentityRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SupportGemsRef(pub usize);

impl Deref for SupportGemsRef {
    type Target = SupportGemsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SupportGems[self.0]
    }
}

impl SupportGemsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SupportGemsRow {
        &TABLE_SupportGems[self.0]
    }
    pub fn get(&self) -> &'static SupportGemsRow {
        &TABLE_SupportGems[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SupportGems.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SupportGemsRow)> {
        TABLE_SupportGems
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
        for row in TABLE_SupportGems.iter() {
            black_box(row);
        }
    }
}
