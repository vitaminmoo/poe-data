#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HudLifeVisuals: LazyLock<Vec<HudLifeVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hudlifevisuals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HudLifeVisualsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown24: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown32: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown40: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(64..64 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(72..72 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(80..80 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(88..88 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(96..96 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown100: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(100..100 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HudLifeVisualsRow {
    pub r#id: String,
    pub r#unknown8: String,
    pub r#unknown16: String,
    pub r#unknown24: String,
    pub r#unknown32: String,
    pub r#unknown40: String,
    pub r#unknown48: String,
    pub r#unknown56: String,
    pub r#unknown64: String,
    pub r#unknown72: String,
    pub r#unknown80: String,
    pub r#unknown88: String,
    pub r#unknown96: i32,
    pub r#unknown100: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HudLifeVisualsRef(pub usize);

impl Deref for HudLifeVisualsRef {
    type Target = HudLifeVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HudLifeVisuals[self.0]
    }
}

impl HudLifeVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HudLifeVisualsRow {
        &TABLE_HudLifeVisuals[self.0]
    }
    pub fn get(&self) -> &'static HudLifeVisualsRow {
        &TABLE_HudLifeVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HudLifeVisuals
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HudLifeVisualsRow)> {
        TABLE_HudLifeVisuals
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
        for row in TABLE_HudLifeVisuals.iter() {
            black_box(row);
        }
    }
}
