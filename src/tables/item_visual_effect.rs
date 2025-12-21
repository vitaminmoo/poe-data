#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemVisualEffect: LazyLock<Vec<ItemVisualEffectRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/itemvisualeffect.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ItemVisualEffectRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#dagger_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#bow_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#one_handed_mace_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#one_handed_sword_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#two_handed_sword_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#two_handed_staff_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown67: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(67).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#two_handed_mace_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(68..68 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#one_handed_axe_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(76..76 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#two_handed_axe_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(84..84 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#claw_epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(92..92 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#pet_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(100..100 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#shield: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(108..108 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#on_hit_effect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(116..116 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(124..124 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(132..132 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown140: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(140..140 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ItemVisualEffectRow {
    pub r#id: String,
    pub r#dagger_epk_file: String,
    pub r#bow_epk_file: String,
    pub r#one_handed_mace_epk_file: String,
    pub r#one_handed_sword_epk_file: String,
    pub r#unknown40: String,
    pub r#two_handed_sword_epk_file: String,
    pub r#two_handed_staff_epk_file: String,
    pub r#hash16: i16,
    pub r#unknown66: bool,
    pub r#unknown67: bool,
    pub r#two_handed_mace_epk_file: String,
    pub r#one_handed_axe_epk_file: String,
    pub r#two_handed_axe_epk_file: String,
    pub r#claw_epk_file: String,
    pub r#pet_file: String,
    pub r#shield: String,
    pub r#on_hit_effect: String,
    pub r#unknown124: String,
    pub r#unknown132: String,
    pub r#unknown140: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemVisualEffectRef(pub usize);

impl Deref for ItemVisualEffectRef {
    type Target = ItemVisualEffectRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemVisualEffect[self.0]
    }
}

impl ItemVisualEffectRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemVisualEffectRow {
        &TABLE_ItemVisualEffect[self.0]
    }
    pub fn get(&self) -> &'static ItemVisualEffectRow {
        &TABLE_ItemVisualEffect[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemVisualEffect
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemVisualEffectRow)> {
        TABLE_ItemVisualEffect
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
        for row in TABLE_ItemVisualEffect.iter() {
            black_box(row);
        }
    }
}
