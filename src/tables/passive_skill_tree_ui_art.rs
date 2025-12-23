#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillTreeUIArt: LazyLock<Vec<PassiveSkillTreeUIArtRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passiveskilltreeuiart.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveSkillTreeUIArtRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#group_background_small: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#group_background_medium: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#group_background_large: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(32).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#group_background_small_blank: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(33..33 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#group_background_medium_blank: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(41..41 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#group_background_large_blank: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(49..49 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#connection: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeConnectionArtRef::new(value as usize)
            },
            r#passive_frame: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
            r#notable_frame: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
            r#keystone_frame: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(105..105 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
            r#jewel_frame: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
            r#glow: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(137..137 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ascendancy_start: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeNodeFrameArtRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillTreeUIArtRow {
    pub r#id: String,
    pub r#group_background_small: String,
    pub r#group_background_medium: String,
    pub r#group_background_large: String,
    pub r#unknown32: bool,
    pub r#group_background_small_blank: String,
    pub r#group_background_medium_blank: String,
    pub r#group_background_large_blank: String,
    pub r#connection: PassiveSkillTreeConnectionArtRef,
    pub r#passive_frame: PassiveSkillTreeNodeFrameArtRef,
    pub r#notable_frame: PassiveSkillTreeNodeFrameArtRef,
    pub r#keystone_frame: PassiveSkillTreeNodeFrameArtRef,
    pub r#jewel_frame: PassiveSkillTreeNodeFrameArtRef,
    pub r#glow: String,
    pub r#ascendancy_start: PassiveSkillTreeNodeFrameArtRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillTreeUIArtRef(pub usize);

impl Deref for PassiveSkillTreeUIArtRef {
    type Target = PassiveSkillTreeUIArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillTreeUIArt[self.0]
    }
}

impl PassiveSkillTreeUIArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillTreeUIArtRow {
        &TABLE_PassiveSkillTreeUIArt[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillTreeUIArtRow {
        &TABLE_PassiveSkillTreeUIArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillTreeUIArt.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillTreeUIArtRow)> {
        TABLE_PassiveSkillTreeUIArt.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkillTreeUIArt.iter() {
            black_box(row);
        }
    }
}
