#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SessionQuestFlags: LazyLock<Vec<SessionQuestFlagsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/sessionquestflags.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| SessionQuestFlagsRow {
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct SessionQuestFlagsRow {
    pub r#quest_flag: QuestFlagsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SessionQuestFlagsRef(pub usize);

impl Deref for SessionQuestFlagsRef {
    type Target = SessionQuestFlagsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SessionQuestFlags[self.0]
    }
}

impl SessionQuestFlagsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SessionQuestFlagsRow {
        &TABLE_SessionQuestFlags[self.0]
    }
    pub fn get(&self) -> &'static SessionQuestFlagsRow {
        &TABLE_SessionQuestFlags[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SessionQuestFlags
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SessionQuestFlagsRow)> {
        TABLE_SessionQuestFlags
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
        for row in TABLE_SessionQuestFlags.iter() {
            black_box(row);
        }
    }
}
