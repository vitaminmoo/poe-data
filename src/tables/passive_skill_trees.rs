#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillTrees: LazyLock<Vec<PassiveSkillTreesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/passiveskilltrees.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| PassiveSkillTreesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#passive_skill_graph: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown33: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(33).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(34).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown35: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(35).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(36).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown38: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(38).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown39: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(39).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(41).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(42).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(43..43 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ClientStringsRef::new(value as usize)
            },
            r#ui_art: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(59..59 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeUIArtRef::new(value as usize)
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(111..111 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(115..115 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillTreesRow {
    pub r#id: String,
    pub r#passive_skill_graph: String,
    pub r#unknown16: i32,
    pub r#unknown20: f32,
    pub r#unknown24: f32,
    pub r#unknown28: f32,
    pub r#unknown32: bool,
    pub r#unknown33: bool,
    pub r#unknown34: bool,
    pub r#unknown35: bool,
    pub r#unknown36: bool,
    pub r#unknown37: bool,
    pub r#unknown38: bool,
    pub r#unknown39: bool,
    pub r#unknown40: bool,
    pub r#unknown41: bool,
    pub r#unknown42: bool,
    pub r#name: ClientStringsRef,
    pub r#ui_art: PassiveSkillTreeUIArtRef,
    pub r#unknown75: i64,
    pub r#unknown91: i64,
    pub r#unknown107: f32,
    pub r#unknown111: f32,
    pub r#unknown115: f32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillTreesRef(pub usize);

impl Deref for PassiveSkillTreesRef {
    type Target = PassiveSkillTreesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillTrees[self.0]
    }
}

impl PassiveSkillTreesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillTreesRow {
        &TABLE_PassiveSkillTrees[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillTreesRow {
        &TABLE_PassiveSkillTrees[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillTrees.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillTreesRow)> {
        TABLE_PassiveSkillTrees.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkillTrees.iter() {
            black_box(row);
        }
    }
}
