#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterPacks: LazyLock<Vec<MonsterPacksRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monsterpacks.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterPacksRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#world_areas: {
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
                values
                    .into_iter()
                    .map(|value| WorldAreasRef::new(value as usize))
                    .collect()
            },
            r#min_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#boss_monster_spawn_chance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#boss_count: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#boss_monsters: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
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
                    .map(|value| MonsterVarietiesRef::new(value as usize))
                    .collect()
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown57: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
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
            r#grounds: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#tags: {
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
                    .map(|value| TagsRef::new(value as usize))
                    .collect()
            },
            r#min_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(105..105 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(109..109 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown113: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#formation: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PackFormationRef::new(value as usize)
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(145..145 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown149: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(149).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown150: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(150..150 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown166: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(166).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown167: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(167).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown168: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(168..168 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown176: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(176).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown177: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(177).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#additional_monsters: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(178..178 + 16).unwrap();
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
                    .map(|value| MonsterVarietiesRef::new(value as usize))
                    .collect()
            },
            r#additional_counts: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(194..194 + 16).unwrap();
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
pub struct MonsterPacksRow {
    pub r#id: String,
    pub r#world_areas: Vec<WorldAreasRef>,
    pub r#min_count: i32,
    pub r#max_count: i32,
    pub r#boss_monster_spawn_chance: i32,
    pub r#boss_count: i32,
    pub r#boss_monsters: Vec<MonsterVarietiesRef>,
    pub r#unknown56: bool,
    pub r#unknown57: Vec<i32>,
    pub r#grounds: Vec<String>,
    pub r#tags: Vec<TagsRef>,
    pub r#min_level: i32,
    pub r#max_level: i32,
    pub r#unknown113: (usize, usize),
    pub r#formation: PackFormationRef,
    pub r#unknown145: i32,
    pub r#unknown149: bool,
    pub r#unknown150: Vec<String>,
    pub r#unknown166: bool,
    pub r#unknown167: bool,
    pub r#unknown168: String,
    pub r#unknown176: bool,
    pub r#unknown177: bool,
    pub r#additional_monsters: Vec<MonsterVarietiesRef>,
    pub r#additional_counts: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterPacksRef(pub usize);

impl Deref for MonsterPacksRef {
    type Target = MonsterPacksRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterPacks[self.0]
    }
}

impl MonsterPacksRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterPacksRow {
        &TABLE_MonsterPacks[self.0]
    }
    pub fn get(&self) -> &'static MonsterPacksRow {
        &TABLE_MonsterPacks[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterPacks.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterPacksRow)> {
        TABLE_MonsterPacks
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
        for row in TABLE_MonsterPacks.iter() {
            black_box(row);
        }
    }
}
