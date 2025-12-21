#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_SkillTotemVariations: LazyLock<Vec<SkillTotemVariationsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/skilltotemvariations.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| SkillTotemVariationsRow {
                r#skill_totems_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    SkillTotems::from_repr(value as usize)
                },
                r#totem_skin_id: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(4..4 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#monster_varieties_key: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(8..8 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    MonsterVarietiesRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct SkillTotemVariationsRow {
    pub r#skill_totems_key: Option<SkillTotems>,
    pub r#totem_skin_id: i32,
    pub r#monster_varieties_key: MonsterVarietiesRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct SkillTotemVariationsRef(pub usize);

impl Deref for SkillTotemVariationsRef {
    type Target = SkillTotemVariationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_SkillTotemVariations[self.0]
    }
}

impl SkillTotemVariationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static SkillTotemVariationsRow {
        &TABLE_SkillTotemVariations[self.0]
    }
    pub fn get(&self) -> &'static SkillTotemVariationsRow {
        &TABLE_SkillTotemVariations[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_SkillTotemVariations
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static SkillTotemVariationsRow)> {
        TABLE_SkillTotemVariations
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
        for row in TABLE_SkillTotemVariations.iter() {
            black_box(row);
        }
    }
}
