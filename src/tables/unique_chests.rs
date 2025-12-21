#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UniqueChests: LazyLock<Vec<UniqueChestsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/uniquechests.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| UniqueChestsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#words_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WordsRef::new(value as usize)
            },
            r#flavour_text_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                FlavourTextRef::new(value as usize)
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#mods_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown89: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
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
            r#appearance_chests_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(105..105 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestsRef::new(value as usize)
            },
            r#chests_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ChestsRef::new(value as usize)
            },
            r#unknown137: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(137..137 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UniqueChestsRow {
    pub r#id: String,
    pub r#words_key: WordsRef,
    pub r#flavour_text_key: FlavourTextRef,
    pub r#min_level: i32,
    pub r#mods_keys: Vec<ModsRef>,
    pub r#spawn_weight: i32,
    pub r#unknown64: Vec<i32>,
    pub r#ao_file: String,
    pub r#unknown88: bool,
    pub r#unknown89: Vec<i32>,
    pub r#appearance_chests_key: ChestsRef,
    pub r#chests_key: ChestsRef,
    pub r#unknown137: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UniqueChestsRef(pub usize);

impl Deref for UniqueChestsRef {
    type Target = UniqueChestsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UniqueChests[self.0]
    }
}

impl UniqueChestsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UniqueChestsRow {
        &TABLE_UniqueChests[self.0]
    }
    pub fn get(&self) -> &'static UniqueChestsRow {
        &TABLE_UniqueChests[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UniqueChests.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UniqueChestsRow)> {
        TABLE_UniqueChests
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
        for row in TABLE_UniqueChests.iter() {
            black_box(row);
        }
    }
}
