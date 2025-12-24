#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BeyondFactions: LazyLock<Vec<BeyondFactionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/beyondfactions.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BeyondFactionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#boss: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BeyondFactionsRow {
    pub r#id: String,
    pub r#boss: MonsterVarietiesRef,
    pub r#unknown24: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BeyondFactionsRef(pub usize);

impl Deref for BeyondFactionsRef {
    type Target = BeyondFactionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BeyondFactions[self.0]
    }
}

impl BeyondFactionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BeyondFactionsRow {
        &TABLE_BeyondFactions[self.0]
    }
    pub fn get(&self) -> &'static BeyondFactionsRow {
        &TABLE_BeyondFactions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BeyondFactions.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BeyondFactionsRow)> {
        TABLE_BeyondFactions.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BeyondFactions.iter() {
            black_box(row);
        }
    }
}
