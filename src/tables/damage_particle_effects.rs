#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_DamageParticleEffects: LazyLock<Vec<DamageParticleEffectsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/damageparticleeffects.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| DamageParticleEffectsRow {
            r#damage_particle_effect_types: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                DamageParticleEffectTypes::from_repr(value as usize)
            },
            r#variation: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(4..4 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown12: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown28: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(44).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct DamageParticleEffectsRow {
    pub r#damage_particle_effect_types: Option<DamageParticleEffectTypes>,
    pub r#variation: i32,
    pub r#unknown8: i32,
    pub r#unknown12: i64,
    pub r#unknown28: i64,
    pub r#unknown44: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct DamageParticleEffectsRef(pub usize);

impl Deref for DamageParticleEffectsRef {
    type Target = DamageParticleEffectsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_DamageParticleEffects[self.0]
    }
}

impl DamageParticleEffectsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static DamageParticleEffectsRow {
        &TABLE_DamageParticleEffects[self.0]
    }
    pub fn get(&self) -> &'static DamageParticleEffectsRow {
        &TABLE_DamageParticleEffects[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_DamageParticleEffects.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static DamageParticleEffectsRow)> {
        TABLE_DamageParticleEffects.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_DamageParticleEffects.iter() {
            black_box(row);
        }
    }
}
