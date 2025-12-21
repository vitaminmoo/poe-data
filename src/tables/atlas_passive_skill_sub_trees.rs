#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasPassiveSkillSubTrees: LazyLock<Vec<AtlasPassiveSkillSubTreesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/atlaspassiveskillsubtrees.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AtlasPassiveSkillSubTreesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#ui_image: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(8..8 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#ui_background: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(16..16 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#illustration_x: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#illustration_y: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#counter_x: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#counter_y: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown40: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#unknown72: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AtlasPassiveSkillSubTreesRow {
    pub r#id: String,
    pub r#ui_image: String,
    pub r#ui_background: String,
    pub r#illustration_x: i32,
    pub r#illustration_y: i32,
    pub r#counter_x: i32,
    pub r#counter_y: i32,
    pub r#unknown40: ClientStringsRef,
    pub r#unknown56: ClientStringsRef,
    pub r#unknown72: ClientStringsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasPassiveSkillSubTreesRef(pub usize);

impl Deref for AtlasPassiveSkillSubTreesRef {
    type Target = AtlasPassiveSkillSubTreesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasPassiveSkillSubTrees[self.0]
    }
}

impl AtlasPassiveSkillSubTreesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasPassiveSkillSubTreesRow {
        &TABLE_AtlasPassiveSkillSubTrees[self.0]
    }
    pub fn get(&self) -> &'static AtlasPassiveSkillSubTreesRow {
        &TABLE_AtlasPassiveSkillSubTrees[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasPassiveSkillSubTrees
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasPassiveSkillSubTreesRow)> {
        TABLE_AtlasPassiveSkillSubTrees
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
        for row in TABLE_AtlasPassiveSkillSubTrees.iter() {
            black_box(row);
        }
    }
}
