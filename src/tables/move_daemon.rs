#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MoveDaemon: LazyLock<Vec<MoveDaemonRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/movedaemon.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MoveDaemonRow {
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
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(76).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown77: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(77).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown78: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(78..78 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(82..82 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown86: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(86..86 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown90: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(90..90 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown94: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(94..94 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(98..98 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown102: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(102..102 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(106).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(107..107 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown115: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(115..115 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown119: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(119).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(120).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown125: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(125).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown126: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(126..126 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown130: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(130..130 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown134: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(134).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MoveDaemonRow {
    pub r#unknown0: i32,
    pub r#unknown4: i32,
    pub r#unknown8: i32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
    pub r#unknown76: bool,
    pub r#unknown77: bool,
    pub r#unknown78: f32,
    pub r#unknown82: i32,
    pub r#unknown86: i32,
    pub r#unknown90: i32,
    pub r#unknown94: i32,
    pub r#unknown98: i32,
    pub r#unknown102: i32,
    pub r#unknown106: bool,
    pub r#unknown107: String,
    pub r#unknown115: i32,
    pub r#unknown119: bool,
    pub r#unknown120: bool,
    pub r#unknown121: i32,
    pub r#unknown125: bool,
    pub r#unknown126: f32,
    pub r#unknown130: i32,
    pub r#unknown134: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MoveDaemonRef(pub usize);

impl Deref for MoveDaemonRef {
    type Target = MoveDaemonRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MoveDaemon[self.0]
    }
}

impl MoveDaemonRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MoveDaemonRow {
        &TABLE_MoveDaemon[self.0]
    }
    pub fn get(&self) -> &'static MoveDaemonRow {
        &TABLE_MoveDaemon[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MoveDaemon.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MoveDaemonRow)> {
        TABLE_MoveDaemon.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MoveDaemon.iter() {
            black_box(row);
        }
    }
}
