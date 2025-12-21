#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MinionType: LazyLock<Vec<MinionTypeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/miniontype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MinionTypeRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#limit_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#active_count_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(41).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(42..42 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown46: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(46).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown47: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(47).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(48).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(49).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown50: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(50).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown51: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(51).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(52).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(53).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown54: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(54).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown55: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(55).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown58: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(58).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown59: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(59..59 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(75).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MinionTypeRow {
    pub r#id: String,
    pub r#limit_stat: StatsRef,
    pub r#active_count_stat: StatsRef,
    pub r#unknown40: bool,
    pub r#unknown41: bool,
    pub r#unknown42: i32,
    pub r#unknown46: bool,
    pub r#unknown47: bool,
    pub r#unknown48: bool,
    pub r#unknown49: bool,
    pub r#unknown50: bool,
    pub r#unknown51: bool,
    pub r#unknown52: bool,
    pub r#unknown53: bool,
    pub r#unknown54: bool,
    pub r#unknown55: bool,
    pub r#unknown56: bool,
    pub r#unknown57: bool,
    pub r#unknown58: bool,
    pub r#unknown59: i64,
    pub r#unknown75: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MinionTypeRef(pub usize);

impl Deref for MinionTypeRef {
    type Target = MinionTypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MinionType[self.0]
    }
}

impl MinionTypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MinionTypeRow {
        &TABLE_MinionType[self.0]
    }
    pub fn get(&self) -> &'static MinionTypeRow {
        &TABLE_MinionType[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MinionType.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MinionTypeRow)> {
        TABLE_MinionType
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
        for row in TABLE_MinionType.iter() {
            println!("{:?}", row);
        }
    }
}
