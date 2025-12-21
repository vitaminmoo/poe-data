#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TormentSpirits: LazyLock<Vec<TormentSpiritsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/tormentspirits.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| TormentSpiritsRow {
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#spirit_mods_keys: {
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#touched_mods_keys: {
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#possessed_mods_keys: {
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#min_zone_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_zone_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#spawn_weight: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#summoned_monster_monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#mods_keys0: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#mods_keys1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(112..112 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#stats_keys1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(128..128 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown144: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#type: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#mods_keys3: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(152..152 + 16).unwrap();
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
                    .map(|value| ModsRef::new(value as usize))
                    .collect()
            },
            r#unknown168: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
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
            },
            r#stats_keys2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(184..184 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown200: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
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
            },
            r#unknown216: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(216..216 + 16).unwrap();
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
            r#unknown232: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(232..232 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#mods_keys4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(248..248 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ModsRef::new(value as usize)
            },
            r#drops_base_items: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(264..264 + 16).unwrap();
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
                    .map(|value| BaseItemTypesRef::new(value as usize))
                    .collect()
            },
            r#unknown280: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(280..280 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown284: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(284..284 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown288: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(288..288 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct TormentSpiritsRow {
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#spirit_mods_keys: Vec<ModsRef>,
    pub r#touched_mods_keys: Vec<ModsRef>,
    pub r#possessed_mods_keys: Vec<ModsRef>,
    pub r#min_zone_level: i32,
    pub r#max_zone_level: i32,
    pub r#spawn_weight: i32,
    pub r#summoned_monster_monster_varieties_key: MonsterVarietiesRef,
    pub r#unknown92: i32,
    pub r#mods_keys0: Vec<ModsRef>,
    pub r#mods_keys1: Vec<ModsRef>,
    pub r#stats_keys1: StatsRef,
    pub r#unknown144: i32,
    pub r#type: i32,
    pub r#mods_keys3: Vec<ModsRef>,
    pub r#unknown168: Vec<i64>,
    pub r#stats_keys2: StatsRef,
    pub r#unknown200: Vec<i64>,
    pub r#unknown216: Vec<i32>,
    pub r#unknown232: i64,
    pub r#mods_keys4: ModsRef,
    pub r#drops_base_items: Vec<BaseItemTypesRef>,
    pub r#unknown280: i32,
    pub r#unknown284: i32,
    pub r#unknown288: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TormentSpiritsRef(pub usize);

impl Deref for TormentSpiritsRef {
    type Target = TormentSpiritsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TormentSpirits[self.0]
    }
}

impl TormentSpiritsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TormentSpiritsRow {
        &TABLE_TormentSpirits[self.0]
    }
    pub fn get(&self) -> &'static TormentSpiritsRow {
        &TABLE_TormentSpirits[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TormentSpirits
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TormentSpiritsRow)> {
        TABLE_TormentSpirits
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
        for row in TABLE_TormentSpirits.iter() {
            black_box(row);
        }
    }
}
