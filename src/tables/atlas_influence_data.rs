#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AtlasInfluenceData: LazyLock<Vec<AtlasInfluenceDataRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/atlasinfluencedata.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| AtlasInfluenceDataRow {
            r#influence_pack: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AtlasInfluenceOutcomesRef::new(value as usize)
            },
            r#monster_packs: {
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
                values
                    .into_iter()
                    .map(|value| MonsterPacksRef::new(value as usize))
                    .collect()
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown48: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown76: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown104: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(104..104 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown128: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown132: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown136: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(136).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown137: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(137).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown138: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(138).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown139: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(139).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AtlasInfluenceDataRow {
    pub r#influence_pack: AtlasInfluenceOutcomesRef,
    pub r#monster_packs: Vec<MonsterPacksRef>,
    pub r#unknown32: i64,
    pub r#unknown48: Vec<StatsRef>,
    pub r#unknown64: i32,
    pub r#unknown68: i32,
    pub r#unknown72: i32,
    pub r#unknown76: i32,
    pub r#unknown80: (usize, usize),
    pub r#unknown96: i32,
    pub r#unknown100: i32,
    pub r#unknown104: i64,
    pub r#unknown120: i32,
    pub r#unknown124: i32,
    pub r#unknown128: i32,
    pub r#unknown132: i32,
    pub r#unknown136: bool,
    pub r#unknown137: bool,
    pub r#unknown138: bool,
    pub r#unknown139: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AtlasInfluenceDataRef(pub usize);

impl Deref for AtlasInfluenceDataRef {
    type Target = AtlasInfluenceDataRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AtlasInfluenceData[self.0]
    }
}

impl AtlasInfluenceDataRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AtlasInfluenceDataRow {
        &TABLE_AtlasInfluenceData[self.0]
    }
    pub fn get(&self) -> &'static AtlasInfluenceDataRow {
        &TABLE_AtlasInfluenceData[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AtlasInfluenceData
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AtlasInfluenceDataRow)> {
        TABLE_AtlasInfluenceData
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
        for row in TABLE_AtlasInfluenceData.iter() {
            black_box(row);
        }
    }
}
