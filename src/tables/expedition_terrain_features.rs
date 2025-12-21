#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionTerrainFeatures: LazyLock<Vec<ExpeditionTerrainFeaturesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/expeditionterrainfeatures.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ExpeditionTerrainFeaturesRow {
                r#id: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(0..0 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
                    value
                },
                r#extra_feature: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ExtraTerrainFeaturesRef::new(value as usize)
                },
                r#expedition_faction: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(24..24 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ExpeditionFactionsRef::new(value as usize)
                },
                r#min_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#max_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown48: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#area: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(52..52 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    WorldAreasRef::new(value as usize)
                },
                r#expedition_areas: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(68..68 + 16).unwrap();
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
                        .map(|value| ExpeditionAreasRef::new(value as usize))
                        .collect()
                },
                r#unknown84: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(84..84 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown88: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(88).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unearth_achievements: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(89..89 + 16).unwrap();
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
                        .map(|value| AchievementItemsRef::new(value as usize))
                        .collect()
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ExpeditionTerrainFeaturesRow {
    pub r#id: String,
    pub r#extra_feature: ExtraTerrainFeaturesRef,
    pub r#expedition_faction: ExpeditionFactionsRef,
    pub r#min_level: i32,
    pub r#max_level: i32,
    pub r#unknown48: i32,
    pub r#area: WorldAreasRef,
    pub r#expedition_areas: Vec<ExpeditionAreasRef>,
    pub r#unknown84: i32,
    pub r#unknown88: bool,
    pub r#unearth_achievements: Vec<AchievementItemsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionTerrainFeaturesRef(pub usize);

impl Deref for ExpeditionTerrainFeaturesRef {
    type Target = ExpeditionTerrainFeaturesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionTerrainFeatures[self.0]
    }
}

impl ExpeditionTerrainFeaturesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionTerrainFeaturesRow {
        &TABLE_ExpeditionTerrainFeatures[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionTerrainFeaturesRow {
        &TABLE_ExpeditionTerrainFeatures[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionTerrainFeatures
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionTerrainFeaturesRow)> {
        TABLE_ExpeditionTerrainFeatures
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
        for row in TABLE_ExpeditionTerrainFeatures.iter() {
            black_box(row);
        }
    }
}
