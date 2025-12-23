#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MonsterDeathAchievements: LazyLock<Vec<MonsterDeathAchievementsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/monsterdeathachievements.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MonsterDeathAchievementsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#monster_varieties_keys: {
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
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#achievement_items_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| AchievementItemsRef::new(value as usize)).collect()
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#player_conditions_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(41..41 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| PlayerConditionsRef::new(value as usize)).collect()
            },
            r#monster_death_conditions_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterDeathConditionsRef::new(value as usize)).collect()
            },
            r#unknown73: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
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
            r#unknown89: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown93: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown97: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(97).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown98: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(98).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown99: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(99..99 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(115..115 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown131: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(131..131 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown147: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(147..147 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown163: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(163..163 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown179: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(179..179 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown183: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(183..183 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown199: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(199).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown200: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown216: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(216).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown217: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(217..217 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MonsterDeathAchievementsRow {
    pub r#id: String,
    pub r#monster_varieties_keys: Vec<MonsterVarietiesRef>,
    pub r#achievement_items_keys: Vec<AchievementItemsRef>,
    pub r#unknown40: bool,
    pub r#player_conditions_keys: Vec<PlayerConditionsRef>,
    pub r#monster_death_conditions_keys: Vec<MonsterDeathConditionsRef>,
    pub r#unknown73: Vec<i64>,
    pub r#unknown89: i32,
    pub r#unknown93: i32,
    pub r#unknown97: bool,
    pub r#unknown98: bool,
    pub r#unknown99: i64,
    pub r#unknown115: (usize, usize),
    pub r#unknown131: (usize, usize),
    pub r#unknown147: (usize, usize),
    pub r#unknown163: i64,
    pub r#unknown179: i32,
    pub r#unknown183: (usize, usize),
    pub r#unknown199: bool,
    pub r#unknown200: (usize, usize),
    pub r#unknown216: bool,
    pub r#unknown217: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MonsterDeathAchievementsRef(pub usize);

impl Deref for MonsterDeathAchievementsRef {
    type Target = MonsterDeathAchievementsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MonsterDeathAchievements[self.0]
    }
}

impl MonsterDeathAchievementsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MonsterDeathAchievementsRow {
        &TABLE_MonsterDeathAchievements[self.0]
    }
    pub fn get(&self) -> &'static MonsterDeathAchievementsRow {
        &TABLE_MonsterDeathAchievements[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MonsterDeathAchievements.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MonsterDeathAchievementsRow)> {
        TABLE_MonsterDeathAchievements.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MonsterDeathAchievements.iter() {
            black_box(row);
        }
    }
}
