#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffVisuals: LazyLock<Vec<BuffVisualsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/buffvisuals.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| BuffVisualsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#buff_dds_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#epk_files1: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#epk_files2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#preload_groups: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| PreloadGroupsRef::new(value as usize)).collect()
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#buff_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(65..65 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#misc_animated2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#buff_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(105..105 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#epk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(113..113 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#has_extra_art: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(121).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#extra_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(122..122 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown130: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(130..130 + 16).unwrap();
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
            r#epk_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(146..146 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#buff_visual_orbs: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(162..162 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| BuffVisualOrbsRef::new(value as usize)).collect()
            },
            r#misc_animated3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(178..178 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown194: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(194..194 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(210..210 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#unknown218: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(218..218 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown234: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(234..234 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown238: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(238..238 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown242: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(242).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown243: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(243).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown244: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(244..244 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown260: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(260).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown261: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(261).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown262: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(262).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown263: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(263..263 + 16).unwrap();
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
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffVisualsRow {
    pub r#id: String,
    pub r#buff_dds_file: String,
    pub r#epk_files1: Vec<String>,
    pub r#epk_files2: Vec<String>,
    pub r#preload_groups: Vec<PreloadGroupsRef>,
    pub r#unknown64: bool,
    pub r#buff_name: String,
    pub r#misc_animated1: MiscAnimatedRef,
    pub r#misc_animated2: MiscAnimatedRef,
    pub r#buff_description: String,
    pub r#epk_file: String,
    pub r#has_extra_art: bool,
    pub r#extra_art: String,
    pub r#unknown130: Vec<i32>,
    pub r#epk_files: Vec<String>,
    pub r#buff_visual_orbs: Vec<BuffVisualOrbsRef>,
    pub r#misc_animated3: MiscAnimatedRef,
    pub r#unknown194: i64,
    pub r#unknown210: BuffVisualsRef,
    pub r#unknown218: i64,
    pub r#unknown234: f32,
    pub r#unknown238: f32,
    pub r#unknown242: bool,
    pub r#unknown243: bool,
    pub r#unknown244: (usize, usize),
    pub r#unknown260: bool,
    pub r#unknown261: bool,
    pub r#unknown262: bool,
    pub r#unknown263: Vec<i32>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffVisualsRef(pub usize);

impl Deref for BuffVisualsRef {
    type Target = BuffVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffVisuals[self.0]
    }
}

impl BuffVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffVisualsRow {
        &TABLE_BuffVisuals[self.0]
    }
    pub fn get(&self) -> &'static BuffVisualsRow {
        &TABLE_BuffVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffVisuals.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffVisualsRow)> {
        TABLE_BuffVisuals.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_BuffVisuals.iter() {
            black_box(row);
        }
    }
}
