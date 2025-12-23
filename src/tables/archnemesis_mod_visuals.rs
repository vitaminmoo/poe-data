#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ArchnemesisModVisuals: LazyLock<Vec<ArchnemesisModVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/archnemesismodvisuals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ArchnemesisModVisualsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown56: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BuffVisualsRef::new(value as usize)).collect()
            },
            r#unknown72: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#unknown88: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(88..88 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MiscAnimatedRef::new(value as usize)).collect()
            },
            r#unknown104: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(104..104 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct ArchnemesisModVisualsRow {
    pub r#id: String,
    pub r#unknown8: MiscAnimatedRef,
    pub r#unknown24: i64,
    pub r#unknown40: i64,
    pub r#unknown56: Vec<BuffVisualsRef>,
    pub r#unknown72: Vec<MonsterVarietiesRef>,
    pub r#unknown88: Vec<MiscAnimatedRef>,
    pub r#unknown104: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ArchnemesisModVisualsRef(pub usize);

impl Deref for ArchnemesisModVisualsRef {
    type Target = ArchnemesisModVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ArchnemesisModVisuals[self.0]
    }
}

impl ArchnemesisModVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ArchnemesisModVisualsRow {
        &TABLE_ArchnemesisModVisuals[self.0]
    }
    pub fn get(&self) -> &'static ArchnemesisModVisualsRow {
        &TABLE_ArchnemesisModVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ArchnemesisModVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ArchnemesisModVisualsRow)> {
        TABLE_ArchnemesisModVisuals.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ArchnemesisModVisuals.iter() {
            black_box(row);
        }
    }
}
