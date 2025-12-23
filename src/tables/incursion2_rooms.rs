#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Incursion2Rooms: LazyLock<Vec<Incursion2RoomsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/incursion2rooms.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| Incursion2RoomsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_pathway: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(9..9 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| Incursion2RoomsRef::new(value as usize)).collect()
            },
            r#unknown25: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(25..25 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| Incursion2RoomsRef::new(value as usize)).collect()
            },
            r#unknown41: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(41..41 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| Incursion2RoomsRef::new(value as usize)).collect()
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(61).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(62).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(63..63 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(71..71 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct Incursion2RoomsRow {
    pub r#id: String,
    pub r#is_pathway: bool,
    pub r#unknown9: Vec<Incursion2RoomsRef>,
    pub r#unknown25: Vec<Incursion2RoomsRef>,
    pub r#unknown41: Vec<Incursion2RoomsRef>,
    pub r#unknown57: i32,
    pub r#unknown61: bool,
    pub r#unknown62: bool,
    pub r#name: String,
    pub r#icon_dds_file: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct Incursion2RoomsRef(pub usize);

impl Deref for Incursion2RoomsRef {
    type Target = Incursion2RoomsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Incursion2Rooms[self.0]
    }
}

impl Incursion2RoomsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Incursion2RoomsRow {
        &TABLE_Incursion2Rooms[self.0]
    }
    pub fn get(&self) -> &'static Incursion2RoomsRow {
        &TABLE_Incursion2Rooms[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Incursion2Rooms.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Incursion2RoomsRow)> {
        TABLE_Incursion2Rooms.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Incursion2Rooms.iter() {
            black_box(row);
        }
    }
}
