#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ActiveSkillRequirements: LazyLock<Vec<ActiveSkillRequirementsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/activeskillrequirements.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActiveSkillRequirementsRow {
            r#granted_effect: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GrantedEffectsRef::new(value as usize)
            },
            r#buff_definition: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffDefinitionsRef::new(value as usize)
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
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
            r#unknown90: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(90..90 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown94: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(94).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ActiveSkillRequirementsRow {
    pub r#granted_effect: GrantedEffectsRef,
    pub r#buff_definition: BuffDefinitionsRef,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: i64,
    pub r#unknown56: i64,
    pub r#stat: StatsRef,
    pub r#unknown88: bool,
    pub r#unknown89: bool,
    pub r#unknown90: i32,
    pub r#unknown94: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActiveSkillRequirementsRef(pub usize);

impl Deref for ActiveSkillRequirementsRef {
    type Target = ActiveSkillRequirementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ActiveSkillRequirements[self.0]
    }
}

impl ActiveSkillRequirementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActiveSkillRequirementsRow {
        &TABLE_ActiveSkillRequirements[self.0]
    }
    pub fn get(&self) -> &'static ActiveSkillRequirementsRow {
        &TABLE_ActiveSkillRequirements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ActiveSkillRequirements.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActiveSkillRequirementsRow)> {
        TABLE_ActiveSkillRequirements.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ActiveSkillRequirements.iter() {
            black_box(row);
        }
    }
}
