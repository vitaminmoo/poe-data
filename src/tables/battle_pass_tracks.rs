#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BattlePassTracks: LazyLock<Vec<BattlePassTracksRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/battlepasstracks.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BattlePassTracksRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BattlePassTracksRow {
    pub r#id: String,
    pub r#description: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BattlePassTracksRef(pub usize);

impl Deref for BattlePassTracksRef {
    type Target = BattlePassTracksRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BattlePassTracks[self.0]
    }
}

impl BattlePassTracksRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BattlePassTracksRow {
        &TABLE_BattlePassTracks[self.0]
    }
    pub fn get(&self) -> &'static BattlePassTracksRow {
        &TABLE_BattlePassTracks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BattlePassTracks
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BattlePassTracksRow)> {
        TABLE_BattlePassTracks
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
        for row in TABLE_BattlePassTracks.iter() {
            black_box(row);
        }
    }
}
