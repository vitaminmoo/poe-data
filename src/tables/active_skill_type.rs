#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ActiveSkillType: LazyLock<Vec<ActiveSkillTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/activeskilltype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActiveSkillTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#flag_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ActiveSkillTypeRow {
    pub r#id: String,
    pub r#flag_stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActiveSkillTypeRef(pub usize);

impl Deref for ActiveSkillTypeRef {
    type Target = ActiveSkillTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActiveSkillType[self.0]
    }
}

impl ActiveSkillTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActiveSkillTypeRow {
        &TABLE_ActiveSkillType[self.0]
    }
    pub fn get(&self) -> &'static ActiveSkillTypeRow {
        &TABLE_ActiveSkillType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActiveSkillType
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActiveSkillTypeRow)> {
        TABLE_ActiveSkillType
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_ActiveSkillType.iter() {
            println!("{:?}", row);
        }
    }
}
