#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EclipseMods: LazyLock<Vec<EclipseModsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/eclipsemods.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| EclipseModsRow {
            r#key: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#spawn_weight_tags_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| TagsRef::new(value as usize)).collect()
            },
            r#spawn_weight_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
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
            r#mods_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(60..60 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_prefix: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EclipseModsRow {
    pub r#key: String,
    pub r#spawn_weight_tags_keys: Vec<TagsRef>,
    pub r#spawn_weight_values: Vec<i32>,
    pub r#mods_key: ModsRef,
    pub r#min_level: i32,
    pub r#max_level: i32,
    pub r#is_prefix: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EclipseModsRef(pub usize);

impl Deref for EclipseModsRef {
    type Target = EclipseModsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EclipseMods[self.0]
    }
}

impl EclipseModsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EclipseModsRow {
        &TABLE_EclipseMods[self.0]
    }
    pub fn get(&self) -> &'static EclipseModsRow {
        &TABLE_EclipseMods[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EclipseMods.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EclipseModsRow)> {
        TABLE_EclipseMods.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_EclipseMods.iter() {
            black_box(row);
        }
    }
}
