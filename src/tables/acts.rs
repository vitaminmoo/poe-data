#![allow(clippy::all)]
use anyhow::Result;
use bytes::{Buf, Bytes};

use crate::dat_parser::{DatFile, DAT_LOADER};

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
static RAW_TABLE_Acts: LazyLock<DatFile> = LazyLock::new(|| {
    DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/acts.datc64")
        .unwrap()
        .clone()
});

#[derive(Debug)]
pub struct ActsRowRaw {
    pub r#id: i32,
    pub r#part: i32,
    pub r#act_number: i32,
    pub r#is_end_game: bool,
}

fn acts_raw(row: &mut Bytes) -> Result<ActsRowRaw> {
    Ok(ActsRowRaw {
        r#id: row.get(0..8).unwrap().get_i32_le(),
        r#part: row.get(8..16).unwrap().get_i32_le(),
        r#act_number: row.get(16..20).unwrap().get_i32_le(),
        r#is_end_game: row.get(40).unwrap().to_le() != 0,
    })
}

impl ActsRowRaw {
    fn row(&self) -> Acts {
        Acts {
            r#id: self.id().unwrap(),
            r#part: self.part,
            r#act_number: self.act_number,
            r#is_end_game: self.is_end_game,
        }
    }
    fn id(&self) -> Result<String> {
        RAW_TABLE_Acts.string_from_offset(self.id as usize)
    }
}

#[allow(non_upper_case_globals)]
pub static TABLE_Acts: LazyLock<Vec<Acts>> = LazyLock::new(|| {
    RAW_TABLE_Acts
        .rows_iter()
        .map(|r| {
            let mut row = r.clone();
            acts_raw(&mut row).unwrap().row()
        })
        .collect()
});

#[derive(Debug)]
pub struct Acts {
    pub r#id: String,
    pub r#part: i32,
    pub r#act_number: i32,
    pub r#is_end_game: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActsRow(pub usize);

impl Deref for ActsRow {
    type Target = Acts;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Acts[self.0]
    }
}

impl ActsRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Acts {
        &TABLE_Acts[self.0]
    }
    pub fn get(&self) -> &'static Acts {
        &TABLE_Acts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Acts.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Acts)> {
        TABLE_Acts.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_table_acts() {
        for (i, act) in TABLE_Acts.iter().enumerate() {
            println!("Row {}: {:?}", i, act);
        }
    }
    #[test]
    fn test_acts_row_deref() {
        let row = ActsRow::new(0);
        let acts: &Acts = row.deref();
        assert_eq!(acts.id, TABLE_Acts[0].id);
        assert_eq!(acts.part, TABLE_Acts[0].part);
        assert_eq!(acts.act_number, TABLE_Acts[0].act_number);
        assert_eq!(acts.is_end_game, TABLE_Acts[0].is_end_game);
    }
}
