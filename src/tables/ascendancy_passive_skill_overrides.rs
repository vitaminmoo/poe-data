#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AscendancyPassiveSkillOverrides: LazyLock<
    Vec<AscendancyPassiveSkillOverridesRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/ascendancypassiveskilloverrides.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AscendancyPassiveSkillOverridesRow {
            r#ascendancy_to_override_for: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AscendancyRef::new(value as usize)
            },
            r#skill_to_override: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
            r#override: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AscendancyPassiveSkillOverridesRow {
    pub r#ascendancy_to_override_for: AscendancyRef,
    pub r#skill_to_override: PassiveSkillsRef,
    pub r#override: PassiveSkillsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AscendancyPassiveSkillOverridesRef(pub usize);

impl Deref for AscendancyPassiveSkillOverridesRef {
    type Target = AscendancyPassiveSkillOverridesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AscendancyPassiveSkillOverrides[self.0]
    }
}

impl AscendancyPassiveSkillOverridesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AscendancyPassiveSkillOverridesRow {
        &TABLE_AscendancyPassiveSkillOverrides[self.0]
    }
    pub fn get(&self) -> &'static AscendancyPassiveSkillOverridesRow {
        &TABLE_AscendancyPassiveSkillOverrides[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AscendancyPassiveSkillOverrides
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static AscendancyPassiveSkillOverridesRow)> {
        TABLE_AscendancyPassiveSkillOverrides
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
        for row in TABLE_AscendancyPassiveSkillOverrides.iter() {
            black_box(row);
        }
    }
}
