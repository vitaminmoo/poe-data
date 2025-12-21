#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_EndgameMapLocation: LazyLock<Vec<EndgameMapLocationRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/endgamemaplocation.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| EndgameMapLocationRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#biomes: {
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
                values
                    .into_iter()
                    .map(|value| EndgameMapBiomesRef::new(value as usize))
                    .collect()
            },
            r#adjacent_biomes: {
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
                values
                    .into_iter()
                    .map(|value| EndgameMapBiomesRef::new(value as usize))
                    .collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct EndgameMapLocationRow {
    pub r#id: String,
    pub r#biomes: Vec<EndgameMapBiomesRef>,
    pub r#adjacent_biomes: Vec<EndgameMapBiomesRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EndgameMapLocationRef(pub usize);

impl Deref for EndgameMapLocationRef {
    type Target = EndgameMapLocationRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_EndgameMapLocation[self.0]
    }
}

impl EndgameMapLocationRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static EndgameMapLocationRow {
        &TABLE_EndgameMapLocation[self.0]
    }
    pub fn get(&self) -> &'static EndgameMapLocationRow {
        &TABLE_EndgameMapLocation[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_EndgameMapLocation
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static EndgameMapLocationRow)> {
        TABLE_EndgameMapLocation
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
        for row in TABLE_EndgameMapLocation.iter() {
            black_box(row);
        }
    }
}
