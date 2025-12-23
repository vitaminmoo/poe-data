#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveSkillTreeConnectionArt: LazyLock<Vec<PassiveSkillTreeConnectionArtRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passiveskilltreeconnectionart.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveSkillTreeConnectionArtRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#normal: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#intermediate2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#intermediate: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#active: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mask: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ornament1: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ornament2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveSkillTreeConnectionArtRow {
    pub r#id: String,
    pub r#normal: String,
    pub r#intermediate2: String,
    pub r#intermediate: String,
    pub r#active: String,
    pub r#mask: String,
    pub r#ornament1: String,
    pub r#ornament2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveSkillTreeConnectionArtRef(pub usize);

impl Deref for PassiveSkillTreeConnectionArtRef {
    type Target = PassiveSkillTreeConnectionArtRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveSkillTreeConnectionArt[self.0]
    }
}

impl PassiveSkillTreeConnectionArtRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveSkillTreeConnectionArtRow {
        &TABLE_PassiveSkillTreeConnectionArt[self.0]
    }
    pub fn get(&self) -> &'static PassiveSkillTreeConnectionArtRow {
        &TABLE_PassiveSkillTreeConnectionArt[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveSkillTreeConnectionArt.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveSkillTreeConnectionArtRow)> {
        TABLE_PassiveSkillTreeConnectionArt.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveSkillTreeConnectionArt.iter() {
            black_box(row);
        }
    }
}
