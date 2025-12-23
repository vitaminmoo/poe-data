#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistBalancePerLevel: LazyLock<Vec<HeistBalancePerLevelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/heistbalanceperlevel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HeistBalancePerLevelRow {
            r#level: {
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
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#heist_value_scaling_key1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#heist_value_scaling_key2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#heist_value_scaling_key3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#heist_value_scaling_key4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#heist_value_scaling_key5: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(108..108 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown112: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown116: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#heist_value_scaling_key6: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#heist_value_scaling_key7: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(140..140 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistValueScalingRef::new(value as usize)
            },
            r#unknown156: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(156..156 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown160: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(160..160 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown164: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(164..164 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistBalancePerLevelRow {
    pub r#level: i32,
    pub r#unknown4: f32,
    pub r#unknown8: f32,
    pub r#unknown12: i32,
    pub r#unknown16: i32,
    pub r#unknown20: f32,
    pub r#unknown24: f32,
    pub r#heist_value_scaling_key1: HeistValueScalingRef,
    pub r#heist_value_scaling_key2: HeistValueScalingRef,
    pub r#heist_value_scaling_key3: HeistValueScalingRef,
    pub r#heist_value_scaling_key4: HeistValueScalingRef,
    pub r#heist_value_scaling_key5: HeistValueScalingRef,
    pub r#unknown108: f32,
    pub r#unknown112: f32,
    pub r#unknown116: f32,
    pub r#unknown120: f32,
    pub r#heist_value_scaling_key6: HeistValueScalingRef,
    pub r#heist_value_scaling_key7: HeistValueScalingRef,
    pub r#unknown156: f32,
    pub r#unknown160: f32,
    pub r#unknown164: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistBalancePerLevelRef(pub usize);

impl Deref for HeistBalancePerLevelRef {
    type Target = HeistBalancePerLevelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistBalancePerLevel[self.0]
    }
}

impl HeistBalancePerLevelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistBalancePerLevelRow {
        &TABLE_HeistBalancePerLevel[self.0]
    }
    pub fn get(&self) -> &'static HeistBalancePerLevelRow {
        &TABLE_HeistBalancePerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistBalancePerLevel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistBalancePerLevelRow)> {
        TABLE_HeistBalancePerLevel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistBalancePerLevel.iter() {
            black_box(row);
        }
    }
}
