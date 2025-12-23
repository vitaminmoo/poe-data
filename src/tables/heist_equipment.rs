#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HeistEquipment: LazyLock<Vec<HeistEquipmentRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/heistequipment.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| HeistEquipmentRow {
            r#base_item_types_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#required_job_heist_jobs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HeistJobsRef::new(value as usize)
            },
            r#required_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HeistEquipmentRow {
    pub r#base_item_types_key: BaseItemTypesRef,
    pub r#required_job_heist_jobs_key: HeistJobsRef,
    pub r#required_level: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HeistEquipmentRef(pub usize);

impl Deref for HeistEquipmentRef {
    type Target = HeistEquipmentRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HeistEquipment[self.0]
    }
}

impl HeistEquipmentRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HeistEquipmentRow {
        &TABLE_HeistEquipment[self.0]
    }
    pub fn get(&self) -> &'static HeistEquipmentRow {
        &TABLE_HeistEquipment[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HeistEquipment.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HeistEquipmentRow)> {
        TABLE_HeistEquipment.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HeistEquipment.iter() {
            black_box(row);
        }
    }
}
