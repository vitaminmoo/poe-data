#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HarvestCraftCostScalingByBasetype: LazyLock<
    Vec<HarvestCraftCostScalingByBasetypeRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/harvestcraftcostscalingbybasetype.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HarvestCraftCostScalingByBasetypeRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HarvestCraftCostScalingByBasetypeRow {
    pub r#unknown0: i64,
    pub r#unknown16: i64,
    pub r#unknown32: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HarvestCraftCostScalingByBasetypeRef(pub usize);

impl Deref for HarvestCraftCostScalingByBasetypeRef {
    type Target = HarvestCraftCostScalingByBasetypeRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HarvestCraftCostScalingByBasetype[self.0]
    }
}

impl HarvestCraftCostScalingByBasetypeRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HarvestCraftCostScalingByBasetypeRow {
        &TABLE_HarvestCraftCostScalingByBasetype[self.0]
    }
    pub fn get(&self) -> &'static HarvestCraftCostScalingByBasetypeRow {
        &TABLE_HarvestCraftCostScalingByBasetype[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HarvestCraftCostScalingByBasetype
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static HarvestCraftCostScalingByBasetypeRow)> {
        TABLE_HarvestCraftCostScalingByBasetype
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
        for row in TABLE_HarvestCraftCostScalingByBasetype.iter() {
            black_box(row);
        }
    }
}
