#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_CharacterShapeShiftBasicSkills: LazyLock<Vec<CharacterShapeShiftBasicSkillsRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/charactershapeshiftbasicskills.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| CharacterShapeShiftBasicSkillsRow {
                r#shape_shift_form: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ShapeShiftFormsRef::new(value as usize)
                },
                r#skill: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    SkillGemsRef::new(value as usize)
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct CharacterShapeShiftBasicSkillsRow {
    pub r#shape_shift_form: ShapeShiftFormsRef,
    pub r#skill: SkillGemsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharacterShapeShiftBasicSkillsRef(pub usize);

impl Deref for CharacterShapeShiftBasicSkillsRef {
    type Target = CharacterShapeShiftBasicSkillsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_CharacterShapeShiftBasicSkills[self.0]
    }
}

impl CharacterShapeShiftBasicSkillsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharacterShapeShiftBasicSkillsRow {
        &TABLE_CharacterShapeShiftBasicSkills[self.0]
    }
    pub fn get(&self) -> &'static CharacterShapeShiftBasicSkillsRow {
        &TABLE_CharacterShapeShiftBasicSkills[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_CharacterShapeShiftBasicSkills
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static CharacterShapeShiftBasicSkillsRow)> {
        TABLE_CharacterShapeShiftBasicSkills
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
        for row in TABLE_CharacterShapeShiftBasicSkills.iter() {
            black_box(row);
        }
    }
}
