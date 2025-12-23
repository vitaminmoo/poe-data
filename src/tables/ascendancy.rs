#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Ascendancy: LazyLock<Vec<AscendancyRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/ascendancy.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| AscendancyRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#class_no: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#character: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharactersRef::new(value as usize)
            },
            r#coordinate_rect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(28..28 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#rgb_flavour_text_colour: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(36..36 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(44..44 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#flavour_text: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(52..52 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ogg_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(60..60 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#passive_tree_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(68..68 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#tree_region_vector: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tree_region_angle: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(80..80 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#background_image: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(88..88 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#disabled: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(92).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#base_ascendancy: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(93..93 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                AscendancyRef::new(value as usize)
            },
            r#ui_art: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(101..101 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                PassiveSkillTreeUIArtRef::new(value as usize)
            },
            r#unknown117: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(117..117 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(133..133 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown149: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(149..149 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct AscendancyRow {
    pub r#id: String,
    pub r#class_no: i32,
    pub r#character: CharactersRef,
    pub r#coordinate_rect: String,
    pub r#rgb_flavour_text_colour: String,
    pub r#name: String,
    pub r#flavour_text: String,
    pub r#ogg_file: String,
    pub r#passive_tree_image: String,
    pub r#tree_region_vector: i32,
    pub r#tree_region_angle: i32,
    pub r#background_image: i32,
    pub r#unknown88: i32,
    pub r#disabled: bool,
    pub r#base_ascendancy: AscendancyRef,
    pub r#ui_art: PassiveSkillTreeUIArtRef,
    pub r#unknown117: i64,
    pub r#unknown133: i64,
    pub r#unknown149: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AscendancyRef(pub usize);

impl Deref for AscendancyRef {
    type Target = AscendancyRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Ascendancy[self.0]
    }
}

impl AscendancyRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AscendancyRow {
        &TABLE_Ascendancy[self.0]
    }
    pub fn get(&self) -> &'static AscendancyRow {
        &TABLE_Ascendancy[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Ascendancy.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AscendancyRow)> {
        TABLE_Ascendancy.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Ascendancy.iter() {
            black_box(row);
        }
    }
}
