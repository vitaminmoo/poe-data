#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_GameObjectTasksFromStats: LazyLock<Vec<GameObjectTasksFromStatsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/gameobjecttasksfromstats.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| GameObjectTasksFromStatsRow {
            r#stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#game_object_task: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                GameObjectTasksRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown36: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(36).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown37: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(37).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown38: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(38).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown39: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(39).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct GameObjectTasksFromStatsRow {
    pub r#stat: StatsRef,
    pub r#game_object_task: GameObjectTasksRef,
    pub r#unknown32: i32,
    pub r#unknown36: bool,
    pub r#unknown37: bool,
    pub r#unknown38: bool,
    pub r#unknown39: bool,
    pub r#unknown40: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GameObjectTasksFromStatsRef(pub usize);

impl Deref for GameObjectTasksFromStatsRef {
    type Target = GameObjectTasksFromStatsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_GameObjectTasksFromStats[self.0]
    }
}

impl GameObjectTasksFromStatsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GameObjectTasksFromStatsRow {
        &TABLE_GameObjectTasksFromStats[self.0]
    }
    pub fn get(&self) -> &'static GameObjectTasksFromStatsRow {
        &TABLE_GameObjectTasksFromStats[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GameObjectTasksFromStats.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GameObjectTasksFromStatsRow)> {
        TABLE_GameObjectTasksFromStats.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_GameObjectTasksFromStats.iter() {
            black_box(row);
        }
    }
}
