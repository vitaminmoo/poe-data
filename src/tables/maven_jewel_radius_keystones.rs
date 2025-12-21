#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MavenJewelRadiusKeystones: LazyLock<Vec<MavenJewelRadiusKeystonesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/mavenjewelradiuskeystones.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| MavenJewelRadiusKeystonesRow {
                r#keystone: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    PassiveSkillsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct MavenJewelRadiusKeystonesRow {
    pub r#keystone: PassiveSkillsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MavenJewelRadiusKeystonesRef(pub usize);

impl Deref for MavenJewelRadiusKeystonesRef {
    type Target = MavenJewelRadiusKeystonesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MavenJewelRadiusKeystones[self.0]
    }
}

impl MavenJewelRadiusKeystonesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MavenJewelRadiusKeystonesRow {
        &TABLE_MavenJewelRadiusKeystones[self.0]
    }
    pub fn get(&self) -> &'static MavenJewelRadiusKeystonesRow {
        &TABLE_MavenJewelRadiusKeystones[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MavenJewelRadiusKeystones
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MavenJewelRadiusKeystonesRow)> {
        TABLE_MavenJewelRadiusKeystones
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
        for row in TABLE_MavenJewelRadiusKeystones.iter() {
            black_box(row);
        }
    }
}
