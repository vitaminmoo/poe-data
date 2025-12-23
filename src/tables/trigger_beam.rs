#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TriggerBeam: LazyLock<Vec<TriggerBeamRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/triggerbeam.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| TriggerBeamRow {
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
                values.into_iter().map(|value| MiscBeamsRef::new(value as usize)).collect()
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
                values.into_iter().map(|value| MiscBeamsRef::new(value as usize)).collect()
            },
            r#unknown36: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            r#unknown52: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(52).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(53..53 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
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
                // array_mutator column.array == true
                let mut cell_bytes = row.get(70..70 + 16).unwrap();
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
            r#unknown86: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(86).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown87: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(87..87 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(91).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TriggerBeamRow {
    pub r#unknown0: i32,
    pub r#unknown4: Vec<MiscBeamsRef>,
    pub r#unknown20: Vec<MiscBeamsRef>,
    pub r#unknown36: Vec<i32>,
    pub r#unknown52: bool,
    pub r#unknown53: i32,
    pub r#unknown57: i32,
    pub r#unknown61: i32,
    pub r#unknown65: i32,
    pub r#unknown69: bool,
    pub r#unknown70: Vec<i32>,
    pub r#unknown86: bool,
    pub r#unknown87: i32,
    pub r#unknown91: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TriggerBeamRef(pub usize);

impl Deref for TriggerBeamRef {
    type Target = TriggerBeamRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TriggerBeam[self.0]
    }
}

impl TriggerBeamRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TriggerBeamRow {
        &TABLE_TriggerBeam[self.0]
    }
    pub fn get(&self) -> &'static TriggerBeamRow {
        &TABLE_TriggerBeam[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TriggerBeam.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TriggerBeamRow)> {
        TABLE_TriggerBeam.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TriggerBeam.iter() {
            black_box(row);
        }
    }
}
