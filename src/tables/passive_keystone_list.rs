#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PassiveKeystoneList: LazyLock<Vec<PassiveKeystoneListRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/passivekeystonelist.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PassiveKeystoneListRow {
            r#passive: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillsRef::new(value as usize)
            },
            r#display_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PassiveKeystoneListRow {
    pub r#passive: PassiveSkillsRef,
    pub r#display_text: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PassiveKeystoneListRef(pub usize);

impl Deref for PassiveKeystoneListRef {
    type Target = PassiveKeystoneListRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PassiveKeystoneList[self.0]
    }
}

impl PassiveKeystoneListRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PassiveKeystoneListRow {
        &TABLE_PassiveKeystoneList[self.0]
    }
    pub fn get(&self) -> &'static PassiveKeystoneListRow {
        &TABLE_PassiveKeystoneList[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PassiveKeystoneList.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PassiveKeystoneListRow)> {
        TABLE_PassiveKeystoneList.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PassiveKeystoneList.iter() {
            black_box(row);
        }
    }
}
