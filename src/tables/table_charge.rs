#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TableCharge: LazyLock<Vec<TableChargeRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/tablecharge.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| TableChargeRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(12).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown13: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(13..13 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown29: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(29).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown30: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(30..30 + 16).unwrap();
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
            r#unknown46: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(46..46 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown62: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(62..62 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(66..66 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown70: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(70..70 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown74: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(74..74 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(78).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(79).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(116).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown125: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(125..125 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown129: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(133..133 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown137: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(137..137 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(141..141 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(145).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(146).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown147: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(147..147 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TableChargeRow {
    pub r#unknown0: i32,
    pub r#unknown4: f32,
    pub r#unknown8: f32,
    pub r#unknown12: bool,
    pub r#unknown13: i64,
    pub r#unknown29: bool,
    pub r#unknown30: Vec<i64>,
    pub r#unknown46: i64,
    pub r#unknown62: i32,
    pub r#unknown66: i32,
    pub r#unknown70: i32,
    pub r#unknown74: i32,
    pub r#unknown78: bool,
    pub r#unknown79: bool,
    pub r#unknown80: i64,
    pub r#unknown96: i64,
    pub r#unknown112: i32,
    pub r#unknown116: bool,
    pub r#unknown117: i32,
    pub r#unknown121: i32,
    pub r#unknown125: i32,
    pub r#unknown129: i32,
    pub r#unknown133: i32,
    pub r#unknown137: i32,
    pub r#unknown141: i32,
    pub r#unknown145: bool,
    pub r#unknown146: bool,
    pub r#unknown147: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TableChargeRef(pub usize);

impl Deref for TableChargeRef {
    type Target = TableChargeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TableCharge[self.0]
    }
}

impl TableChargeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TableChargeRow {
        &TABLE_TableCharge[self.0]
    }
    pub fn get(&self) -> &'static TableChargeRow {
        &TABLE_TableCharge[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TableCharge.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TableChargeRow)> {
        TABLE_TableCharge.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TableCharge.iter() {
            black_box(row);
        }
    }
}
