#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_TalkingPetNPCAudio: LazyLock<Vec<TalkingPetNPCAudioRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/talkingpetnpcaudio.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| TalkingPetNPCAudioRow {
            r#unknown0: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TalkingPetAudioEventsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TalkingPetsRef::new(value as usize)
            },
            r#unknown32: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(32..32 + 16).unwrap();
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
            r#unknown48: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(48..48 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown56: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(56).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct TalkingPetNPCAudioRow {
    pub r#unknown0: TalkingPetAudioEventsRef,
    pub r#unknown16: TalkingPetsRef,
    pub r#unknown32: Vec<i32>,
    pub r#unknown48: String,
    pub r#unknown56: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct TalkingPetNPCAudioRef(pub usize);

impl Deref for TalkingPetNPCAudioRef {
    type Target = TalkingPetNPCAudioRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_TalkingPetNPCAudio[self.0]
    }
}

impl TalkingPetNPCAudioRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static TalkingPetNPCAudioRow {
        &TABLE_TalkingPetNPCAudio[self.0]
    }
    pub fn get(&self) -> &'static TalkingPetNPCAudioRow {
        &TABLE_TalkingPetNPCAudio[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_TalkingPetNPCAudio.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static TalkingPetNPCAudioRow)> {
        TABLE_TalkingPetNPCAudio.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_TalkingPetNPCAudio.iter() {
            black_box(row);
        }
    }
}
