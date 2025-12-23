#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternateSkillTargetingBehaviours: LazyLock<Vec<AlternateSkillTargetingBehavioursRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/alternateskilltargetingbehaviours.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AlternateSkillTargetingBehavioursRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#client_strings: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
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
        })
        .collect()
});

#[derive(Debug)]
pub struct AlternateSkillTargetingBehavioursRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#client_strings: ClientStringsRef,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternateSkillTargetingBehavioursRef(pub usize);

impl Deref for AlternateSkillTargetingBehavioursRef {
    type Target = AlternateSkillTargetingBehavioursRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
}

impl AlternateSkillTargetingBehavioursRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternateSkillTargetingBehavioursRow {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
    pub fn get(&self) -> &'static AlternateSkillTargetingBehavioursRow {
        &TABLE_AlternateSkillTargetingBehaviours[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternateSkillTargetingBehaviours.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternateSkillTargetingBehavioursRow)> {
        TABLE_AlternateSkillTargetingBehaviours.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AlternateSkillTargetingBehaviours.iter() {
            black_box(row);
        }
    }
}
