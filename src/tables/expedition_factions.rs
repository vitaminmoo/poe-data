#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ExpeditionFactions: LazyLock<Vec<ExpeditionFactionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/expeditionfactions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ExpeditionFactionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#faction_flag: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#faction_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_varieties: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#progress1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#progress2_vaal: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#progress3_final: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#tags: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TagsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ExpeditionFactionsRow {
    pub r#id: String,
    pub r#name: String,
    pub r#faction_flag: String,
    pub r#unknown24: i32,
    pub r#faction_icon: String,
    pub r#monster_varieties: MonsterVarietiesRef,
    pub r#progress1: NPCTextAudioRef,
    pub r#progress2_vaal: NPCTextAudioRef,
    pub r#progress3_final: NPCTextAudioRef,
    pub r#tags: TagsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ExpeditionFactionsRef(pub usize);

impl Deref for ExpeditionFactionsRef {
    type Target = ExpeditionFactionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ExpeditionFactions[self.0]
    }
}

impl ExpeditionFactionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ExpeditionFactionsRow {
        &TABLE_ExpeditionFactions[self.0]
    }
    pub fn get(&self) -> &'static ExpeditionFactionsRow {
        &TABLE_ExpeditionFactions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ExpeditionFactions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ExpeditionFactionsRow)> {
        TABLE_ExpeditionFactions
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
        for row in TABLE_ExpeditionFactions.iter() {
            black_box(row);
        }
    }
}
