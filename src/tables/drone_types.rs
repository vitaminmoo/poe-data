#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DroneTypes: LazyLock<Vec<DroneTypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/dronetypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DroneTypesRow {
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
                QuestFlagsRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#deploy_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unlocked_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#socketable_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#not_powered_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DroneTypesRow {
    pub r#id: String,
    pub r#unknown8: QuestFlagsRef,
    pub r#unknown24: MonsterVarietiesRef,
    pub r#deploy_text: String,
    pub r#unknown48: String,
    pub r#unlocked_stat: StatsRef,
    pub r#socketable_text: String,
    pub r#not_powered_text: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DroneTypesRef(pub usize);

impl Deref for DroneTypesRef {
    type Target = DroneTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DroneTypes[self.0]
    }
}

impl DroneTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DroneTypesRow {
        &TABLE_DroneTypes[self.0]
    }
    pub fn get(&self) -> &'static DroneTypesRow {
        &TABLE_DroneTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DroneTypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DroneTypesRow)> {
        TABLE_DroneTypes
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
        for row in TABLE_DroneTypes.iter() {
            black_box(row);
        }
    }
}
