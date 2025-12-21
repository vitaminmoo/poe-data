#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameCorruptionMods: LazyLock<Vec<EndgameCorruptionModsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/endgamecorruptionmods.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| EndgameCorruptionModsRow {
                r#corruption_mod: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ModsRef::new(value as usize)
                },
                r#spawn_weight: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
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
pub struct EndgameCorruptionModsRow {
    pub r#corruption_mod: ModsRef,
    pub r#spawn_weight: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameCorruptionModsRef(pub usize);

impl Deref for EndgameCorruptionModsRef {
    type Target = EndgameCorruptionModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameCorruptionMods[self.0]
    }
}

impl EndgameCorruptionModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameCorruptionModsRow {
        &TABLE_EndgameCorruptionMods[self.0]
    }
    pub fn get(&self) -> &'static EndgameCorruptionModsRow {
        &TABLE_EndgameCorruptionMods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameCorruptionMods
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameCorruptionModsRow)> {
        TABLE_EndgameCorruptionMods
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
        for row in TABLE_EndgameCorruptionMods.iter() {
            black_box(row);
        }
    }
}
