#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionConditionalApparitionEvents: LazyLock<Vec<MicrotransactionConditionalApparitionEventsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionconditionalapparitionevents.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionConditionalApparitionEventsRow {
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
            r#unknown20: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(20..20 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(40).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown41: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(41..41 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown45: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(45..45 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown49: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(49..49 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(65..65 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(81..81 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MicrotransactionConditionalApparitionEventsRow {
    pub r#unknown0: i64,
    pub r#unknown16: i32,
    pub r#unknown20: i32,
    pub r#unknown24: i64,
    pub r#unknown40: bool,
    pub r#unknown41: i32,
    pub r#unknown45: i32,
    pub r#unknown49: i64,
    pub r#unknown65: i64,
    pub r#unknown81: i32,
    pub r#unknown85: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionConditionalApparitionEventsRef(pub usize);

impl Deref for MicrotransactionConditionalApparitionEventsRef {
    type Target = MicrotransactionConditionalApparitionEventsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionConditionalApparitionEvents[self.0]
    }
}

impl MicrotransactionConditionalApparitionEventsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionConditionalApparitionEventsRow {
        &TABLE_MicrotransactionConditionalApparitionEvents[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionConditionalApparitionEventsRow {
        &TABLE_MicrotransactionConditionalApparitionEvents[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionConditionalApparitionEvents.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MicrotransactionConditionalApparitionEventsRow)> {
        TABLE_MicrotransactionConditionalApparitionEvents.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MicrotransactionConditionalApparitionEvents.iter() {
            black_box(row);
        }
    }
}
