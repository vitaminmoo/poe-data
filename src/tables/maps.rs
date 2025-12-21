#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Maps: LazyLock<Vec<MapsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/maps.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MapsRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#regular_world_area: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#monster_packs: {
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
                values
                    .into_iter()
                    .map(|value| MonsterPacksRef::new(value as usize))
                    .collect()
            },
            r#achievement_item: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#map_series: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(81).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown82: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(82).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MapsRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#regular_world_area: WorldAreasRef,
    pub r#monster_packs: Vec<MonsterPacksRef>,
    pub r#achievement_item: AchievementItemsRef,
    pub r#unknown64: String,
    pub r#tier: i32,
    pub r#map_series: i32,
    pub r#unknown80: bool,
    pub r#unknown81: bool,
    pub r#unknown82: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MapsRef(pub usize);

impl Deref for MapsRef {
    type Target = MapsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Maps[self.0]
    }
}

impl MapsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MapsRow {
        &TABLE_Maps[self.0]
    }
    pub fn get(&self) -> &'static MapsRow {
        &TABLE_Maps[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Maps.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MapsRow)> {
        TABLE_Maps.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Maps.iter() {
            black_box(row);
        }
    }
}
