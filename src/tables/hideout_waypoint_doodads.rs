#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutWaypointDoodads: LazyLock<Vec<HideoutWaypointDoodadsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/hideoutwaypointdoodads.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| HideoutWaypointDoodadsRow {
                r#unknown0: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown16: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct HideoutWaypointDoodadsRow {
    pub r#unknown0: i64,
    pub r#unknown16: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutWaypointDoodadsRef(pub usize);

impl Deref for HideoutWaypointDoodadsRef {
    type Target = HideoutWaypointDoodadsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutWaypointDoodads[self.0]
    }
}

impl HideoutWaypointDoodadsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutWaypointDoodadsRow {
        &TABLE_HideoutWaypointDoodads[self.0]
    }
    pub fn get(&self) -> &'static HideoutWaypointDoodadsRow {
        &TABLE_HideoutWaypointDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutWaypointDoodads
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutWaypointDoodadsRow)> {
        TABLE_HideoutWaypointDoodads
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
        for row in TABLE_HideoutWaypointDoodads.iter() {
            black_box(row);
        }
    }
}
