#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasUpgradesInventoryLayout: LazyLock<Vec<AtlasUpgradesInventoryLayoutRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/atlasupgradesinventorylayout.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AtlasUpgradesInventoryLayoutRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#unknown8: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#voidstone: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(12..12 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#unknown28: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(28..28 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#objective: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#grant_atlas_upgrade: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    QuestFlagsRef::new(value as usize)
                },
                r#unknown56: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AtlasUpgradesInventoryLayoutRow {
    pub r#id: String,
    pub r#unknown8: i32,
    pub r#voidstone: BaseItemTypesRef,
    pub r#unknown28: i32,
    pub r#objective: String,
    pub r#grant_atlas_upgrade: QuestFlagsRef,
    pub r#unknown56: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasUpgradesInventoryLayoutRef(pub usize);

impl Deref for AtlasUpgradesInventoryLayoutRef {
    type Target = AtlasUpgradesInventoryLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasUpgradesInventoryLayout[self.0]
    }
}

impl AtlasUpgradesInventoryLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasUpgradesInventoryLayoutRow {
        &TABLE_AtlasUpgradesInventoryLayout[self.0]
    }
    pub fn get(&self) -> &'static AtlasUpgradesInventoryLayoutRow {
        &TABLE_AtlasUpgradesInventoryLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasUpgradesInventoryLayout
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasUpgradesInventoryLayoutRow)>
    {
        TABLE_AtlasUpgradesInventoryLayout
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
        for row in TABLE_AtlasUpgradesInventoryLayout.iter() {
            black_box(row);
        }
    }
}
