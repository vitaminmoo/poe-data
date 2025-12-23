#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_UltimatumModifiers: LazyLock<Vec<UltimatumModifiersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/ultimatummodifiers.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| UltimatumModifiersRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#types: {
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
                values.into_iter().map(|value| UltimatumModifierTypesRef::new(value as usize)).collect()
            },
            r#extra_mods: {
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
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
            r#types_filtered: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| UltimatumModifierTypesRef::new(value as usize)).collect()
            },
            r#previous_tier: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| UltimatumModifiersRef::new(value as usize)).collect()
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(72).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(73..73 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(81..81 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 2).unwrap();
                let value = cell_bytes.get_u16_le();
                value
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#unknown93: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(93..93 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#buff_templates: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(109..109 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BuffTemplatesRef::new(value as usize)).collect()
            },
            r#tier: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(125..125 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(129..129 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown137: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(137..137 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#text_audio: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(153..153 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#unknown169: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(169..169 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#map_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(185..185 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| StatsRef::new(value as usize)).collect()
            },
            r#map_stat_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(201..201 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#map_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(217..217 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct UltimatumModifiersRow {
    pub r#id: String,
    pub r#types: Vec<UltimatumModifierTypesRef>,
    pub r#extra_mods: Vec<ModsRef>,
    pub r#types_filtered: Vec<UltimatumModifierTypesRef>,
    pub r#previous_tier: Vec<UltimatumModifiersRef>,
    pub r#unknown72: bool,
    pub r#name: String,
    pub r#icon: String,
    pub r#hash16: u16,
    pub r#unknown91: i16,
    pub r#unknown93: Vec<String>,
    pub r#buff_templates: Vec<BuffTemplatesRef>,
    pub r#tier: i32,
    pub r#description: String,
    pub r#unknown137: (usize, usize),
    pub r#text_audio: NPCTextAudioRef,
    pub r#unknown169: i64,
    pub r#map_stats: Vec<StatsRef>,
    pub r#map_stat_values: Vec<i32>,
    pub r#map_mods: Vec<ModsRef>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct UltimatumModifiersRef(pub usize);

impl Deref for UltimatumModifiersRef {
    type Target = UltimatumModifiersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_UltimatumModifiers[self.0]
    }
}

impl UltimatumModifiersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static UltimatumModifiersRow {
        &TABLE_UltimatumModifiers[self.0]
    }
    pub fn get(&self) -> &'static UltimatumModifiersRow {
        &TABLE_UltimatumModifiers[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_UltimatumModifiers.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static UltimatumModifiersRow)> {
        TABLE_UltimatumModifiers.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_UltimatumModifiers.iter() {
            black_box(row);
        }
    }
}
