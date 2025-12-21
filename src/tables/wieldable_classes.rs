#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WieldableClasses: LazyLock<Vec<WieldableClassesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/wieldableclasses.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| WieldableClassesRow {
            r#item_class: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown18: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(18..18 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown34: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(50..50 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(98..98 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown114: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(114..114 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown130: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(130..130 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown162: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(162..162 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown178: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(178..178 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown194: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(194..194 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(210..210 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown226: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(226..226 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown242: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(242..242 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown258: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(258..258 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown274: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(274..274 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown290: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(290..290 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown306: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(306..306 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown322: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(322..322 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown338: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(338..338 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown354: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(354..354 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown370: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(370..370 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown386: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(386..386 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown402: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(402..402 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WieldableClassesRow {
    pub r#item_class: ItemClassesRef,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#unknown18: i64,
    pub r#unknown34: i64,
    pub r#unknown50: i64,
    pub r#unknown66: i64,
    pub r#unknown82: i64,
    pub r#unknown98: i64,
    pub r#unknown114: i64,
    pub r#unknown130: i64,
    pub r#unknown146: i64,
    pub r#unknown162: i64,
    pub r#unknown178: i64,
    pub r#unknown194: i64,
    pub r#unknown210: i64,
    pub r#unknown226: i64,
    pub r#unknown242: i64,
    pub r#unknown258: i64,
    pub r#unknown274: i64,
    pub r#unknown290: i64,
    pub r#unknown306: i64,
    pub r#unknown322: i64,
    pub r#unknown338: i64,
    pub r#unknown354: i64,
    pub r#unknown370: i64,
    pub r#unknown386: i64,
    pub r#unknown402: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WieldableClassesRef(pub usize);

impl Deref for WieldableClassesRef {
    type Target = WieldableClassesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WieldableClasses[self.0]
    }
}

impl WieldableClassesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WieldableClassesRow {
        &TABLE_WieldableClasses[self.0]
    }
    pub fn get(&self) -> &'static WieldableClassesRow {
        &TABLE_WieldableClasses[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WieldableClasses
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WieldableClassesRow)> {
        TABLE_WieldableClasses
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_WieldableClasses.iter() {
            println!("{:?}", row);
        }
    }
}
