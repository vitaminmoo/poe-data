#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Acts: LazyLock<Vec<ActsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/acts.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ActsRow {
            r#id: df
                .string_from_offset(row.get(0..0 + 8).unwrap().get_i32_le() as usize)
                .unwrap(), // string
            r#ui_title: df
                .string_from_offset(row.get(8..8 + 8).unwrap().get_i32_le() as usize)
                .unwrap(), // string
            r#act_number: row.get(16..16 + 4).unwrap().get_i32_le(), // basic
            r#unknown20: df
                .string_from_offset(row.get(20..20 + 8).unwrap().get_i32_le() as usize)
                .unwrap(), // string
            r#unknown28: df
                .string_from_offset(row.get(28..28 + 8).unwrap().get_i32_le() as usize)
                .unwrap(), // string
            r#unknown36: row.get(36..36 + 4).unwrap().get_i32_le(),  // basic
            r#is_end_game: row.get(40).unwrap().to_le() != 0,        // basic
            r#unknown41: row.get(41..41 + 4).unwrap().get_i32_le(),  // basic
            r#unknown45: df
                .array_from_offset(
                    row.get(45 + 8..45 + 16).unwrap().get_u64_le() as usize,
                    row.get(45..45 + 8).unwrap().get_u64_le() as usize,
                    16,
                )
                .unwrap()
                .iter()
                .map(|x| x.clone().get_u64_le())
                .collect(), // foreignrow_unknown_array'],

            r#unknown61: df
                .array_from_offset(
                    row.get(61 + 8..61 + 16).unwrap().get_u64_le() as usize,
                    row.get(61..61 + 8).unwrap().get_u64_le() as usize,
                    16,
                )
                .unwrap()
                .iter()
                .map(|x| x.clone().get_u64_le())
                .collect(), // foreignrow_unknown_array'],

            r#unknown77: df
                .array_from_offset(
                    row.get(77 + 8..77 + 16).unwrap().get_u64_le() as usize,
                    row.get(77..77 + 8).unwrap().get_u64_le() as usize,
                    16,
                )
                .unwrap()
                .iter()
                .map(|x| x.clone().get_u64_le())
                .collect(), // foreignrow_unknown_array'],

            r#unknown93: {
                let x = row.get(93..93 + 16).unwrap().get_u64_le();
                match x {
                    0xFEFEFEFEFEFEFEFE => None,
                    _ => Some(x),
                }
            }, // foreignrow_unknown

            r#unknown109: df
                .array_from_offset(
                    row.get(109 + 8..109 + 16).unwrap().get_u64_le() as usize,
                    row.get(109..109 + 8).unwrap().get_u64_le() as usize,
                    16,
                )
                .unwrap()
                .iter()
                .map(|x| x.clone().get_u64_le())
                .collect(), // foreignrow_unknown_array'],

            r#description: df
                .string_from_offset(row.get(125..125 + 8).unwrap().get_i32_le() as usize)
                .unwrap(), // string
        })
        .collect()
});

#[derive(Debug)]
pub struct ActsRow {
    pub r#id: String,             // string
    pub r#ui_title: String,       // string
    pub r#act_number: i32,        // basic
    pub r#unknown20: String,      // string
    pub r#unknown28: String,      // string
    pub r#unknown36: i32,         // basic
    pub r#is_end_game: bool,      // basic
    pub r#unknown41: i32,         // basic
    pub r#unknown45: Vec<u64>,    // foreignrow_unknown_array
    pub r#unknown61: Vec<u64>,    // foreignrow_unknown_array
    pub r#unknown77: Vec<u64>,    // foreignrow_unknown_array
    pub r#unknown93: Option<u64>, // foreignrow_unknown
    pub r#unknown109: Vec<u64>,   // foreignrow_unknown_array
    pub r#description: String,    // string
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ActsRef(pub usize);

impl Deref for ActsRef {
    type Target = ActsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Acts[self.0]
    }
}

impl ActsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn get(&self) -> &'static ActsRow {
        &TABLE_Acts[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Acts.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ActsRow)> {
        TABLE_Acts.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_Acts.iter() {
            println!("{:?}", row);
        }
    }
}
