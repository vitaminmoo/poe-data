#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LegionRewardTypeVisuals: LazyLock<Vec<LegionRewardTypeVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/legionrewardtypevisuals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| LegionRewardTypeVisualsRow {
            r#int_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#minimap_icons_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MinimapIconsRef::new(value as usize)
            },
            r#unknown20: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct LegionRewardTypeVisualsRow {
    pub r#int_id: i32,
    pub r#minimap_icons_key: MinimapIconsRef,
    pub r#unknown20: String,
    pub r#misc_animated_key: MiscAnimatedRef,
    pub r#unknown44: f32,
    pub r#id: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LegionRewardTypeVisualsRef(pub usize);

impl Deref for LegionRewardTypeVisualsRef {
    type Target = LegionRewardTypeVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LegionRewardTypeVisuals[self.0]
    }
}

impl LegionRewardTypeVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LegionRewardTypeVisualsRow {
        &TABLE_LegionRewardTypeVisuals[self.0]
    }
    pub fn get(&self) -> &'static LegionRewardTypeVisualsRow {
        &TABLE_LegionRewardTypeVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LegionRewardTypeVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LegionRewardTypeVisualsRow)> {
        TABLE_LegionRewardTypeVisuals.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_LegionRewardTypeVisuals.iter() {
            black_box(row);
        }
    }
}
