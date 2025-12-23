#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_MavenDialog: LazyLock<Vec<MavenDialogRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/mavendialog.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| MavenDialogRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text_audio_t1: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#text_audio_t2: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#text_audio_t3: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#text_audio_t4: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#text_audio_t5: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#unknown88: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(88).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#text_audio_t6: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(89..89 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct MavenDialogRow {
    pub r#id: String,
    pub r#text_audio_t1: NPCTextAudioRef,
    pub r#text_audio_t2: NPCTextAudioRef,
    pub r#text_audio_t3: NPCTextAudioRef,
    pub r#text_audio_t4: NPCTextAudioRef,
    pub r#text_audio_t5: NPCTextAudioRef,
    pub r#unknown88: bool,
    pub r#text_audio_t6: NPCTextAudioRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct MavenDialogRef(pub usize);

impl Deref for MavenDialogRef {
    type Target = MavenDialogRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_MavenDialog[self.0]
    }
}

impl MavenDialogRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static MavenDialogRow {
        &TABLE_MavenDialog[self.0]
    }
    pub fn get(&self) -> &'static MavenDialogRow {
        &TABLE_MavenDialog[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_MavenDialog.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static MavenDialogRow)> {
        TABLE_MavenDialog.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_MavenDialog.iter() {
            black_box(row);
        }
    }
}
