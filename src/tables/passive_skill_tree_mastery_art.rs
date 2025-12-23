#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillTreeMasteryArt: LazyLock<Vec<PassiveSkillTreeMasteryArtRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passiveskilltreemasteryart.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveSkillTreeMasteryArtRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#inactive_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active_effect_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillTreeMasteryArtRow {
    pub r#id: String,
    pub r#inactive_icon: String,
    pub r#active_icon: String,
    pub r#active_effect_image: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillTreeMasteryArtRef(pub usize);

impl Deref for PassiveSkillTreeMasteryArtRef {
    type Target = PassiveSkillTreeMasteryArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillTreeMasteryArt[self.0]
    }
}

impl PassiveSkillTreeMasteryArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillTreeMasteryArtRow {
        &TABLE_PassiveSkillTreeMasteryArt[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillTreeMasteryArtRow {
        &TABLE_PassiveSkillTreeMasteryArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillTreeMasteryArt.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillTreeMasteryArtRow)> {
        TABLE_PassiveSkillTreeMasteryArt.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkillTreeMasteryArt.iter() {
            black_box(row);
        }
    }
}
