#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ArmourTypes: LazyLock<Vec<ArmourTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/armourtypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ArmourTypesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#armour: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#evasion: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#energy_shield: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#increased_movement_speed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#ward: {
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
        })
        .collect()
});

#[derive(Debug)]
pub struct ArmourTypesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#armour: i32,
    pub r#evasion: i32,
    pub r#energy_shield: i32,
    pub r#increased_movement_speed: i32,
    pub r#ward: i32,
    pub r#unknown36: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ArmourTypesRef(pub usize);

impl Deref for ArmourTypesRef {
    type Target = ArmourTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ArmourTypes[self.0]
    }
}

impl ArmourTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ArmourTypesRow {
        &TABLE_ArmourTypes[self.0]
    }
    pub fn get(&self) -> &'static ArmourTypesRow {
        &TABLE_ArmourTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ArmourTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ArmourTypesRow)> {
        TABLE_ArmourTypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ArmourTypes.iter() {
            black_box(row);
        }
    }
}
