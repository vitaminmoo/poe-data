#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GeometryTrigger: LazyLock<Vec<GeometryTriggerRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/geometrytrigger.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GeometryTriggerRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
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
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown84: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown97: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(97..97 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown101: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(101).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown102: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(102..102 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown106: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(106..106 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown110: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(110).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown111: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(111..111 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(115..115 + 16).unwrap();
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
            r#unknown131: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(131..131 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown135: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(135).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown136: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(136..136 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown140: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(140).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(141).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(142..142 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown146: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(146).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown147: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(147).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(164).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown165: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(165).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown166: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(166..166 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown170: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(170..170 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown174: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(174..174 + 16).unwrap();
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
            r#unknown190: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(190..190 + 16).unwrap();
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
            r#unknown206: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(206..206 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(210..210 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown214: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(214..214 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown218: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(218).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown219: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(219..219 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown223: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(223..223 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown227: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(227..227 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown231: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(231).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown232: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(232).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GeometryTriggerRow {
    pub r#unknown0: i32,
    pub r#unknown4: i64,
    pub r#unknown20: i64,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
    pub r#unknown48: i32,
    pub r#unknown52: i32,
    pub r#unknown56: i32,
    pub r#unknown60: i32,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#unknown80: i32,
    pub r#unknown84: i32,
    pub r#unknown88: i32,
    pub r#unknown92: i32,
    pub r#unknown96: bool,
    pub r#unknown97: i32,
    pub r#unknown101: bool,
    pub r#unknown102: i32,
    pub r#unknown106: i32,
    pub r#unknown110: bool,
    pub r#unknown111: i32,
    pub r#unknown115: Vec<i64>,
    pub r#unknown131: i32,
    pub r#unknown135: bool,
    pub r#unknown136: i32,
    pub r#unknown140: bool,
    pub r#unknown141: bool,
    pub r#unknown142: i32,
    pub r#unknown146: bool,
    pub r#unknown147: bool,
    pub r#unknown148: i64,
    pub r#unknown164: bool,
    pub r#unknown165: bool,
    pub r#unknown166: i32,
    pub r#unknown170: i32,
    pub r#unknown174: Vec<i64>,
    pub r#unknown190: Vec<i32>,
    pub r#unknown206: i32,
    pub r#unknown210: f32,
    pub r#unknown214: i32,
    pub r#unknown218: bool,
    pub r#unknown219: i32,
    pub r#unknown223: i32,
    pub r#unknown227: i32,
    pub r#unknown231: bool,
    pub r#unknown232: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GeometryTriggerRef(pub usize);

impl Deref for GeometryTriggerRef {
    type Target = GeometryTriggerRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GeometryTrigger[self.0]
    }
}

impl GeometryTriggerRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GeometryTriggerRow {
        &TABLE_GeometryTrigger[self.0]
    }
    pub fn get(&self) -> &'static GeometryTriggerRow {
        &TABLE_GeometryTrigger[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GeometryTrigger
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GeometryTriggerRow)> {
        TABLE_GeometryTrigger
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
        for row in TABLE_GeometryTrigger.iter() {
            black_box(row);
        }
    }
}
