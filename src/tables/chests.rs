#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Chests: LazyLock<Vec<ChestsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/chests.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ChestsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(9..9 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(13..13 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(21..21 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown38: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown42: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(42).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown43: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(43).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(81..81 + 16).unwrap();
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
            r#unknown97: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
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
            r#unknown113: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown129: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(133..133 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(141..141 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(145..145 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown161: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(161..161 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown177: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(177..177 + 16).unwrap();
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
            r#unknown193: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(193..193 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown209: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(209..209 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown217: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(217).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown218: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(218..218 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown234: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(234..234 + 16).unwrap();
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
            r#unknown250: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(250..250 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown258: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(258..258 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown262: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(262..262 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown266: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(266).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown267: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(267..267 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown283: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(283..283 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown299: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(299).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown300: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(300).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown301: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(301..301 + 16).unwrap();
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
            r#unknown317: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(317).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown318: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(318..318 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown334: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(334).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown335: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(335).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown336: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(336).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown337: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(337..337 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown341: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(341..341 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown345: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(345..345 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown361: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(361).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ChestsRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#unknown9: i32,
    pub r#name: String,
    pub r#ao_files: Vec<String>,
    pub r#unknown37: bool,
    pub r#unknown38: i32,
    pub r#unknown42: bool,
    pub r#unknown43: bool,
    pub r#unknown44: i32,
    pub r#unknown48: Vec<i64>,
    pub r#unknown64: i64,
    pub r#unknown80: bool,
    pub r#unknown81: Vec<i64>,
    pub r#unknown97: Vec<i64>,
    pub r#unknown113: i64,
    pub r#unknown129: i32,
    pub r#unknown133: String,
    pub r#unknown141: i32,
    pub r#unknown145: i64,
    pub r#unknown161: i64,
    pub r#unknown177: Vec<i64>,
    pub r#unknown193: i64,
    pub r#unknown209: String,
    pub r#unknown217: bool,
    pub r#unknown218: i64,
    pub r#unknown234: Vec<i64>,
    pub r#unknown250: String,
    pub r#unknown258: i32,
    pub r#unknown262: i32,
    pub r#unknown266: bool,
    pub r#unknown267: i64,
    pub r#unknown283: i64,
    pub r#unknown299: bool,
    pub r#unknown300: bool,
    pub r#unknown301: Vec<i64>,
    pub r#unknown317: bool,
    pub r#unknown318: Vec<String>,
    pub r#unknown334: bool,
    pub r#unknown335: bool,
    pub r#unknown336: bool,
    pub r#unknown337: i32,
    pub r#unknown341: i32,
    pub r#unknown345: i64,
    pub r#unknown361: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChestsRef(pub usize);

impl Deref for ChestsRef {
    type Target = ChestsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Chests[self.0]
    }
}

impl ChestsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ChestsRow {
        &TABLE_Chests[self.0]
    }
    pub fn get(&self) -> &'static ChestsRow {
        &TABLE_Chests[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Chests.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ChestsRow)> {
        TABLE_Chests.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Chests.iter() {
            black_box(row);
        }
    }
}
