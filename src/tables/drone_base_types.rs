#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DroneBaseTypes: LazyLock<Vec<DroneBaseTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/dronebasetypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DroneBaseTypesRow {
            r#base_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                DroneTypesRef::new(value as usize)
            },
            r#charges: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#duration: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#enemy_limit: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#visual: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#empowerment: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#use_achievement: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#created_via_power_core: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DroneBaseTypesRow {
    pub r#base_type: BaseItemTypesRef,
    pub r#type: DroneTypesRef,
    pub r#charges: i32,
    pub r#duration: i32,
    pub r#enemy_limit: i32,
    pub r#visual: BuffVisualsRef,
    pub r#empowerment: i32,
    pub r#use_achievement: AchievementItemsRef,
    pub r#created_via_power_core: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DroneBaseTypesRef(pub usize);

impl Deref for DroneBaseTypesRef {
    type Target = DroneBaseTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DroneBaseTypes[self.0]
    }
}

impl DroneBaseTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DroneBaseTypesRow {
        &TABLE_DroneBaseTypes[self.0]
    }
    pub fn get(&self) -> &'static DroneBaseTypesRow {
        &TABLE_DroneBaseTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DroneBaseTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DroneBaseTypesRow)> {
        TABLE_DroneBaseTypes
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
        for row in TABLE_DroneBaseTypes.iter() {
            black_box(row);
        }
    }
}
