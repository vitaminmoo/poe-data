#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_LeagueProgressQuestFlags: LazyLock<Vec<LeagueProgressQuestFlagsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/leagueprogressquestflags.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| LeagueProgressQuestFlagsRow {
                r#quest_flag: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    QuestFlagsRef::new(value as usize)
                },
                r#completion_string: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ClientStringsRef::new(value as usize)
                },
                r#boss: {
                    // array_mutator column.array == false && column.type == 'string'
                    let mut cell_bytes = row.get(32..32 + 8).unwrap();
                    let offset = cell_bytes.get_i32_le() as usize;
                    let value = df.string_from_offset(offset).unwrap();
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
pub struct LeagueProgressQuestFlagsRow {
    pub r#quest_flag: QuestFlagsRef,
    pub r#completion_string: ClientStringsRef,
    pub r#boss: String,
    pub r#unknown40: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct LeagueProgressQuestFlagsRef(pub usize);

impl Deref for LeagueProgressQuestFlagsRef {
    type Target = LeagueProgressQuestFlagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_LeagueProgressQuestFlags[self.0]
    }
}

impl LeagueProgressQuestFlagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static LeagueProgressQuestFlagsRow {
        &TABLE_LeagueProgressQuestFlags[self.0]
    }
    pub fn get(&self) -> &'static LeagueProgressQuestFlagsRow {
        &TABLE_LeagueProgressQuestFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_LeagueProgressQuestFlags
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static LeagueProgressQuestFlagsRow)> {
        TABLE_LeagueProgressQuestFlags
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
        for row in TABLE_LeagueProgressQuestFlags.iter() {
            black_box(row);
        }
    }
}
