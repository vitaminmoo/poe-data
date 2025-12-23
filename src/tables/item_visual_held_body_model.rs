#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemVisualHeldBodyModel: LazyLock<Vec<ItemVisualHeldBodyModelRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemvisualheldbodymodel.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemVisualHeldBodyModelRow {
            r#item_visual_identity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemVisualIdentityRef::new(value as usize)
            },
            r#marauder_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ranger_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#witch_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#duelist_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#templar_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#shadow_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#scion_animated_object: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#marauder_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ranger_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#witch_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#duelist_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#templar_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(104..104 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#shadow_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(112..112 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#scion_bone: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(120..120 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemVisualHeldBodyModelRow {
    pub r#item_visual_identity: ItemVisualIdentityRef,
    pub r#marauder_animated_object: String,
    pub r#ranger_animated_object: String,
    pub r#witch_animated_object: String,
    pub r#duelist_animated_object: String,
    pub r#templar_animated_object: String,
    pub r#shadow_animated_object: String,
    pub r#scion_animated_object: String,
    pub r#marauder_bone: String,
    pub r#ranger_bone: String,
    pub r#witch_bone: String,
    pub r#duelist_bone: String,
    pub r#templar_bone: String,
    pub r#shadow_bone: String,
    pub r#scion_bone: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemVisualHeldBodyModelRef(pub usize);

impl Deref for ItemVisualHeldBodyModelRef {
    type Target = ItemVisualHeldBodyModelRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemVisualHeldBodyModel[self.0]
    }
}

impl ItemVisualHeldBodyModelRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemVisualHeldBodyModelRow {
        &TABLE_ItemVisualHeldBodyModel[self.0]
    }
    pub fn get(&self) -> &'static ItemVisualHeldBodyModelRow {
        &TABLE_ItemVisualHeldBodyModel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemVisualHeldBodyModel.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemVisualHeldBodyModelRow)> {
        TABLE_ItemVisualHeldBodyModel.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_ItemVisualHeldBodyModel.iter() {
            black_box(row);
        }
    }
}
