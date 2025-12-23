#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MinimapIcons: LazyLock<Vec<MinimapIconsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/minimapicons.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MinimapIconsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#minimap_icon_radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#largemap_icon_radius: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown17: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#minimap_icon_pointer_max_distance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(18..18 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown22: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(22..22 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown26: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MinimapIconsRow {
    pub r#id: String,
    pub r#minimap_icon_radius: i32,
    pub r#largemap_icon_radius: i32,
    pub r#unknown16: bool,
    pub r#unknown17: bool,
    pub r#minimap_icon_pointer_max_distance: i32,
    pub r#unknown22: i32,
    pub r#unknown26: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MinimapIconsRef(pub usize);

impl Deref for MinimapIconsRef {
    type Target = MinimapIconsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MinimapIcons[self.0]
    }
}

impl MinimapIconsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MinimapIconsRow {
        &TABLE_MinimapIcons[self.0]
    }
    pub fn get(&self) -> &'static MinimapIconsRow {
        &TABLE_MinimapIcons[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MinimapIcons.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MinimapIconsRow)> {
        TABLE_MinimapIcons.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MinimapIcons.iter() {
            black_box(row);
        }
    }
}
