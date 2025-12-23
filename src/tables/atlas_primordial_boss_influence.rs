#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasPrimordialBossInfluence: LazyLock<Vec<AtlasPrimordialBossInfluenceRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/atlasprimordialbossinfluence.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AtlasPrimordialBossInfluenceRow {
            r#boss: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AtlasPrimordialBossesRef::new(value as usize)
            },
            r#progress: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_map_tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown52: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AtlasPrimordialBossInfluenceRow {
    pub r#boss: AtlasPrimordialBossesRef,
    pub r#progress: i32,
    pub r#min_map_tier: i32,
    pub r#unknown24: i32,
    pub r#unknown28: i32,
    pub r#unknown32: i64,
    pub r#unknown48: f32,
    pub r#unknown52: QuestFlagsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasPrimordialBossInfluenceRef(pub usize);

impl Deref for AtlasPrimordialBossInfluenceRef {
    type Target = AtlasPrimordialBossInfluenceRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasPrimordialBossInfluence[self.0]
    }
}

impl AtlasPrimordialBossInfluenceRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasPrimordialBossInfluenceRow {
        &TABLE_AtlasPrimordialBossInfluence[self.0]
    }
    pub fn get(&self) -> &'static AtlasPrimordialBossInfluenceRow {
        &TABLE_AtlasPrimordialBossInfluence[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasPrimordialBossInfluence.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasPrimordialBossInfluenceRow)> {
        TABLE_AtlasPrimordialBossInfluence.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_AtlasPrimordialBossInfluence.iter() {
            black_box(row);
        }
    }
}
