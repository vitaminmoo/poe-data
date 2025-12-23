#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Incursion2TempleLevelPerCharacterLevel: LazyLock<Vec<Incursion2TempleLevelPerCharacterLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/incursion2templelevelpercharacterlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| Incursion2TempleLevelPerCharacterLevelRow {
            r#character_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct Incursion2TempleLevelPerCharacterLevelRow {
    pub r#character_level: i32,
    pub r#area_level: i32,
    pub r#unknown8: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Incursion2TempleLevelPerCharacterLevelRef(pub usize);

impl Deref for Incursion2TempleLevelPerCharacterLevelRef {
    type Target = Incursion2TempleLevelPerCharacterLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Incursion2TempleLevelPerCharacterLevel[self.0]
    }
}

impl Incursion2TempleLevelPerCharacterLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Incursion2TempleLevelPerCharacterLevelRow {
        &TABLE_Incursion2TempleLevelPerCharacterLevel[self.0]
    }
    pub fn get(&self) -> &'static Incursion2TempleLevelPerCharacterLevelRow {
        &TABLE_Incursion2TempleLevelPerCharacterLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Incursion2TempleLevelPerCharacterLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Incursion2TempleLevelPerCharacterLevelRow)> {
        TABLE_Incursion2TempleLevelPerCharacterLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Incursion2TempleLevelPerCharacterLevel.iter() {
            black_box(row);
        }
    }
}
