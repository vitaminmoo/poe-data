#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionRecycleOutcomes: LazyLock<
    Vec<MicrotransactionRecycleOutcomesRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionrecycleoutcomes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionRecycleOutcomesRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionRecycleOutcomesRow {
    pub r#unknown0: i64,
    pub r#unknown16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionRecycleOutcomesRef(pub usize);

impl Deref for MicrotransactionRecycleOutcomesRef {
    type Target = MicrotransactionRecycleOutcomesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionRecycleOutcomes[self.0]
    }
}

impl MicrotransactionRecycleOutcomesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionRecycleOutcomesRow {
        &TABLE_MicrotransactionRecycleOutcomes[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionRecycleOutcomesRow {
        &TABLE_MicrotransactionRecycleOutcomes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionRecycleOutcomes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionRecycleOutcomesRow)> {
        TABLE_MicrotransactionRecycleOutcomes
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
        for row in TABLE_MicrotransactionRecycleOutcomes.iter() {
            black_box(row);
        }
    }
}
