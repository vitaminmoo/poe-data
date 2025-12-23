#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Archetypes: LazyLock<Vec<ArchetypesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/archetypes.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| ArchetypesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#characters_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                CharactersRef::new(value as usize)
            },
            r#passive_skill_tree_url: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ascendancy_class_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(32..32 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(40..40 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ui_image_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#tutorial_video_bk_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown68: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown72: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#background_image_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(76..76 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_temporary: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(84).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown85: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(85).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#archetype_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(86..86 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown94: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(94).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown95: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(95).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ArchetypesRow {
    pub r#id: String,
    pub r#characters_key: CharactersRef,
    pub r#passive_skill_tree_url: String,
    pub r#ascendancy_class_name: String,
    pub r#description: String,
    pub r#ui_image_file: String,
    pub r#tutorial_video_bk_file: String,
    pub r#unknown64: i32,
    pub r#unknown68: f32,
    pub r#unknown72: f32,
    pub r#background_image_file: String,
    pub r#is_temporary: bool,
    pub r#unknown85: bool,
    pub r#archetype_image: String,
    pub r#unknown94: bool,
    pub r#unknown95: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ArchetypesRef(pub usize);

impl Deref for ArchetypesRef {
    type Target = ArchetypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Archetypes[self.0]
    }
}

impl ArchetypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ArchetypesRow {
        &TABLE_Archetypes[self.0]
    }
    pub fn get(&self) -> &'static ArchetypesRow {
        &TABLE_Archetypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Archetypes.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ArchetypesRow)> {
        TABLE_Archetypes.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Archetypes.iter() {
            black_box(row);
        }
    }
}
