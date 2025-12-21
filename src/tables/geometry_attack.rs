#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GeometryAttack: LazyLock<Vec<GeometryAttackRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/geometryattack.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GeometryAttackRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
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
            r#unknown20: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
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
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(57..57 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(65..65 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown69: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(69).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown70: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(70).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown71: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(71..71 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(95..95 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(99..99 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown103: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(103..103 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(107..107 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(111).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(112).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown113: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(113..113 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(121).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown122: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(122..122 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown126: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(126).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown127: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(127..127 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown143: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(143..143 + 16).unwrap();
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
            r#unknown159: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(159..159 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown163: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(163).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(164).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown165: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(165..165 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown181: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(181).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown182: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(182..182 + 16).unwrap();
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
            r#unknown198: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(198).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown199: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(199).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown200: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown216: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(216).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown217: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(217).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown218: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(218..218 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown234: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(234).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown235: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(235).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown236: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(236).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown237: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(237).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown238: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(238..238 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown242: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(242).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown243: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(243).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown244: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(244..244 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown248: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(248..248 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown252: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(252..252 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown256: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(256).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown257: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(257..257 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GeometryAttackRow {
    pub r#unknown0: i32,
    pub r#unknown4: Vec<i64>,
    pub r#unknown20: Vec<i64>,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: bool,
    pub r#unknown57: i32,
    pub r#unknown61: i32,
    pub r#unknown65: i32,
    pub r#unknown69: bool,
    pub r#unknown70: bool,
    pub r#unknown71: (usize, usize),
    pub r#unknown87: i32,
    pub r#unknown91: i32,
    pub r#unknown95: i32,
    pub r#unknown99: i32,
    pub r#unknown103: i32,
    pub r#unknown107: i32,
    pub r#unknown111: bool,
    pub r#unknown112: bool,
    pub r#unknown113: i32,
    pub r#unknown117: i32,
    pub r#unknown121: bool,
    pub r#unknown122: i32,
    pub r#unknown126: bool,
    pub r#unknown127: i64,
    pub r#unknown143: Vec<i32>,
    pub r#unknown159: i32,
    pub r#unknown163: bool,
    pub r#unknown164: bool,
    pub r#unknown165: i64,
    pub r#unknown181: bool,
    pub r#unknown182: Vec<i32>,
    pub r#unknown198: bool,
    pub r#unknown199: bool,
    pub r#unknown200: i64,
    pub r#unknown216: bool,
    pub r#unknown217: bool,
    pub r#unknown218: (usize, usize),
    pub r#unknown234: bool,
    pub r#unknown235: bool,
    pub r#unknown236: bool,
    pub r#unknown237: bool,
    pub r#unknown238: i32,
    pub r#unknown242: bool,
    pub r#unknown243: bool,
    pub r#unknown244: i32,
    pub r#unknown248: i32,
    pub r#unknown252: i32,
    pub r#unknown256: bool,
    pub r#unknown257: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GeometryAttackRef(pub usize);

impl Deref for GeometryAttackRef {
    type Target = GeometryAttackRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GeometryAttack[self.0]
    }
}

impl GeometryAttackRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GeometryAttackRow {
        &TABLE_GeometryAttack[self.0]
    }
    pub fn get(&self) -> &'static GeometryAttackRow {
        &TABLE_GeometryAttack[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GeometryAttack
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GeometryAttackRow)> {
        TABLE_GeometryAttack
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
        for row in TABLE_GeometryAttack.iter() {
            black_box(row);
        }
    }
}
