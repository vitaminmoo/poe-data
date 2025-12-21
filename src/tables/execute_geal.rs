#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExecuteGEAL: LazyLock<Vec<ExecuteGEALRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/executegeal.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExecuteGEALRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
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
                    .map(|value| MiscAnimatedRef::new(value as usize))
                    .collect()
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown58: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(58..58 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown74: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(74).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown75: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown79: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(79..79 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown83: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(83..83 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown87: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(87..87 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown95: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(95).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(104).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(105..105 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown109: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(109).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown110: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(110..110 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown114: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(114..114 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown118: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(118).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown119: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(119..119 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown123: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(123..123 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown127: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(127..127 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown131: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(131..131 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown135: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(135..135 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown139: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(139..139 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown147: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(147..147 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown155: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(155..155 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown163: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(163..163 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown171: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(171..171 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown179: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(179..179 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown183: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(183).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown184: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(184).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown185: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(185..185 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown201: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(201..201 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown205: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(205).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown206: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(206..206 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(210).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown211: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(211..211 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown215: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(215..215 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown219: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(219..219 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExecuteGEALRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#misc_animated: Vec<MiscAnimatedRef>,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: bool,
    pub r#unknown45: i32,
    pub r#unknown49: i32,
    pub r#unknown53: i32,
    pub r#unknown57: bool,
    pub r#unknown58: (usize, usize),
    pub r#unknown74: bool,
    pub r#unknown75: i32,
    pub r#unknown79: i32,
    pub r#unknown83: i32,
    pub r#unknown87: i32,
    pub r#unknown91: i32,
    pub r#unknown95: bool,
    pub r#script: String,
    pub r#unknown104: bool,
    pub r#unknown105: i32,
    pub r#unknown109: bool,
    pub r#unknown110: i32,
    pub r#unknown114: i32,
    pub r#unknown118: bool,
    pub r#unknown119: i32,
    pub r#unknown123: i32,
    pub r#unknown127: i32,
    pub r#unknown131: i32,
    pub r#unknown135: i32,
    pub r#unknown139: String,
    pub r#unknown147: String,
    pub r#unknown155: String,
    pub r#unknown163: String,
    pub r#unknown171: String,
    pub r#unknown179: i32,
    pub r#unknown183: bool,
    pub r#unknown184: bool,
    pub r#unknown185: (usize, usize),
    pub r#unknown201: i32,
    pub r#unknown205: bool,
    pub r#unknown206: i32,
    pub r#unknown210: bool,
    pub r#unknown211: i32,
    pub r#unknown215: i32,
    pub r#unknown219: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExecuteGEALRef(pub usize);

impl Deref for ExecuteGEALRef {
    type Target = ExecuteGEALRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExecuteGEAL[self.0]
    }
}

impl ExecuteGEALRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExecuteGEALRow {
        &TABLE_ExecuteGEAL[self.0]
    }
    pub fn get(&self) -> &'static ExecuteGEALRow {
        &TABLE_ExecuteGEAL[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExecuteGEAL.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExecuteGEALRow)> {
        TABLE_ExecuteGEAL
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
        for row in TABLE_ExecuteGEAL.iter() {
            black_box(row);
        }
    }
}
