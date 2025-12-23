#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Essences: LazyLock<Vec<EssencesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/essences.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EssencesRow {
            r#base_item_type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#hash32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_u32_le();
                value
            },
            r#unknown20: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(20..20 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown36: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
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
            r#unknown52: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#monster_mod: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#perfect: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(84).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#upgrade_result: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                EssencesRef::new(value as usize)
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#map_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EssencesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#hash32: u32,
    pub r#unknown20: Vec<i32>,
    pub r#unknown36: Vec<ModsRef>,
    pub r#unknown52: Vec<i32>,
    pub r#monster_mod: ModsRef,
    pub r#perfect: bool,
    pub r#upgrade_result: EssencesRef,
    pub r#tier: i32,
    pub r#map_stat: StatsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EssencesRef(pub usize);

impl Deref for EssencesRef {
    type Target = EssencesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Essences[self.0]
    }
}

impl EssencesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EssencesRow {
        &TABLE_Essences[self.0]
    }
    pub fn get(&self) -> &'static EssencesRow {
        &TABLE_Essences[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Essences.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EssencesRow)> {
        TABLE_Essences.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Essences.iter() {
            black_box(row);
        }
    }
}
