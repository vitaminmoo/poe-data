#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MicrotransactionSkillGemEffectSlotTypes: LazyLock<
    Vec<MicrotransactionSkillGemEffectSlotTypesRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/microtransactionskillgemeffectslottypes.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| MicrotransactionSkillGemEffectSlotTypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#type: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
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
pub struct MicrotransactionSkillGemEffectSlotTypesRow {
    pub r#id: String,
    pub r#type: String,
    pub r#unknown16: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MicrotransactionSkillGemEffectSlotTypesRef(pub usize);

impl Deref for MicrotransactionSkillGemEffectSlotTypesRef {
    type Target = MicrotransactionSkillGemEffectSlotTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
}

impl MicrotransactionSkillGemEffectSlotTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MicrotransactionSkillGemEffectSlotTypesRow {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
    pub fn get(&self) -> &'static MicrotransactionSkillGemEffectSlotTypesRow {
        &TABLE_MicrotransactionSkillGemEffectSlotTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MicrotransactionSkillGemEffectSlotTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static MicrotransactionSkillGemEffectSlotTypesRow)> {
        TABLE_MicrotransactionSkillGemEffectSlotTypes
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
        for row in TABLE_MicrotransactionSkillGemEffectSlotTypes.iter() {
            black_box(row);
        }
    }
}
