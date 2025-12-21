#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GraphicalItemReceptacleSlot: LazyLock<Vec<GraphicalItemReceptacleSlotRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/graphicalitemreceptacleslot.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| GraphicalItemReceptacleSlotRow {
                r#graphical_item_receptacle: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    GraphicalItemReceptacleRef::new(value as usize)
                },
                r#key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
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
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(56..56 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#sound_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(72..72 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SoundEffectsRef::new(value as usize)
                },
                r#unknown88: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(88..88 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct GraphicalItemReceptacleSlotRow {
    pub r#graphical_item_receptacle: GraphicalItemReceptacleRef,
    pub r#key: i32,
    pub r#unknown20: i64,
    pub r#unknown36: i32,
    pub r#unknown40: String,
    pub r#unknown48: String,
    pub r#base_item_type: BaseItemTypesRef,
    pub r#sound_effect: SoundEffectsRef,
    pub r#unknown88: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GraphicalItemReceptacleSlotRef(pub usize);

impl Deref for GraphicalItemReceptacleSlotRef {
    type Target = GraphicalItemReceptacleSlotRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GraphicalItemReceptacleSlot[self.0]
    }
}

impl GraphicalItemReceptacleSlotRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GraphicalItemReceptacleSlotRow {
        &TABLE_GraphicalItemReceptacleSlot[self.0]
    }
    pub fn get(&self) -> &'static GraphicalItemReceptacleSlotRow {
        &TABLE_GraphicalItemReceptacleSlot[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GraphicalItemReceptacleSlot
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GraphicalItemReceptacleSlotRow)>
    {
        TABLE_GraphicalItemReceptacleSlot
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
        for row in TABLE_GraphicalItemReceptacleSlot.iter() {
            black_box(row);
        }
    }
}
