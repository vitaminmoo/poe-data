#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Incursion2RoomPerLevel: LazyLock<Vec<Incursion2RoomPerLevelRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/incursion2roomperlevel.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| Incursion2RoomPerLevelRow {
                r#room: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    Incursion2RoomsRef::new(value as usize)
                },
                r#level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(20..20 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#description: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(28..28 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#name: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(36..36 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#icon_dds_file: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(44..44 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#mod: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(52..52 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ModsRef::new(value as usize)
                },
                r#mod_values: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
                r#description2: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(84..84 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct Incursion2RoomPerLevelRow {
    pub r#room: Incursion2RoomsRef,
    pub r#level: i32,
    pub r#id: String,
    pub r#description: String,
    pub r#name: String,
    pub r#icon_dds_file: String,
    pub r#mod: ModsRef,
    pub r#mod_values: Vec<i32>,
    pub r#description2: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Incursion2RoomPerLevelRef(pub usize);

impl Deref for Incursion2RoomPerLevelRef {
    type Target = Incursion2RoomPerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Incursion2RoomPerLevel[self.0]
    }
}

impl Incursion2RoomPerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Incursion2RoomPerLevelRow {
        &TABLE_Incursion2RoomPerLevel[self.0]
    }
    pub fn get(&self) -> &'static Incursion2RoomPerLevelRow {
        &TABLE_Incursion2RoomPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Incursion2RoomPerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Incursion2RoomPerLevelRow)> {
        TABLE_Incursion2RoomPerLevel
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
        for row in TABLE_Incursion2RoomPerLevel.iter() {
            black_box(row);
        }
    }
}
