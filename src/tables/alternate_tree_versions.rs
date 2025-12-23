#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AlternateTreeVersions: LazyLock<Vec<AlternateTreeVersionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/alternatetreeversions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AlternateTreeVersionsRow {
            r#conqueror_type: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#small_attribute_replaced: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#small_normal_passive_replaced: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#small_attribute_passive_skill_additions: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(10..10 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#notable_additions: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#small_normal_passive_skill_additions: {
                // array_mutator column.array == false && column.type == 'interval'
                let mut cell_bytes = row.get(26..26 + 8).unwrap();
                let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
                value
            },
            r#notable_replacement_spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(34..34 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AlternateTreeVersionsRow {
    pub r#conqueror_type: String,
    pub r#small_attribute_replaced: bool,
    pub r#small_normal_passive_replaced: bool,
    pub r#small_attribute_passive_skill_additions: (i32, i32),
    pub r#notable_additions: (i32, i32),
    pub r#small_normal_passive_skill_additions: (i32, i32),
    pub r#notable_replacement_spawn_weight: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AlternateTreeVersionsRef(pub usize);

impl Deref for AlternateTreeVersionsRef {
    type Target = AlternateTreeVersionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AlternateTreeVersions[self.0]
    }
}

impl AlternateTreeVersionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AlternateTreeVersionsRow {
        &TABLE_AlternateTreeVersions[self.0]
    }
    pub fn get(&self) -> &'static AlternateTreeVersionsRow {
        &TABLE_AlternateTreeVersions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AlternateTreeVersions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AlternateTreeVersionsRow)> {
        TABLE_AlternateTreeVersions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AlternateTreeVersions.iter() {
            black_box(row);
        }
    }
}
