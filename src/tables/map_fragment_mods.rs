#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MapFragmentMods: LazyLock<Vec<MapFragmentModsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/mapfragmentmods.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MapFragmentModsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
            r#achievement_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
            r#map_fragment_families: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(52).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown53: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(53).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown54: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(54).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown55: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(55).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(57).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapFragmentModsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#mods: Vec<ModsRef>,
    pub r#achievement_items: Vec<AchievementItemsRef>,
    pub r#map_fragment_families: i32,
    pub r#unknown52: bool,
    pub r#unknown53: bool,
    pub r#unknown54: bool,
    pub r#unknown55: bool,
    pub r#unknown56: bool,
    pub r#unknown57: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapFragmentModsRef(pub usize);

impl Deref for MapFragmentModsRef {
    type Target = MapFragmentModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MapFragmentMods[self.0]
    }
}

impl MapFragmentModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapFragmentModsRow {
        &TABLE_MapFragmentMods[self.0]
    }
    pub fn get(&self) -> &'static MapFragmentModsRow {
        &TABLE_MapFragmentMods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MapFragmentMods.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapFragmentModsRow)> {
        TABLE_MapFragmentMods.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MapFragmentMods.iter() {
            black_box(row);
        }
    }
}
