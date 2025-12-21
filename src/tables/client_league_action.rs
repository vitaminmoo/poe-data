#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ClientLeagueAction: LazyLock<Vec<ClientLeagueActionRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/clientleagueaction.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ClientLeagueActionRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#gamepad_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ClientLeagueActionRow {
    pub r#id: String,
    pub r#unknown8: i64,
    pub r#unknown24: i32,
    pub r#gamepad_icon: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ClientLeagueActionRef(pub usize);

impl Deref for ClientLeagueActionRef {
    type Target = ClientLeagueActionRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ClientLeagueAction[self.0]
    }
}

impl ClientLeagueActionRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ClientLeagueActionRow {
        &TABLE_ClientLeagueAction[self.0]
    }
    pub fn get(&self) -> &'static ClientLeagueActionRow {
        &TABLE_ClientLeagueAction[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ClientLeagueAction
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ClientLeagueActionRow)> {
        TABLE_ClientLeagueAction
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
        for row in TABLE_ClientLeagueAction.iter() {
            black_box(row);
        }
    }
}
