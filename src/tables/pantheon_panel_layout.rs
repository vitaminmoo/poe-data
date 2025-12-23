#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_PantheonPanelLayout: LazyLock<Vec<PantheonPanelLayoutRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/pantheonpanellayout.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| PantheonPanelLayoutRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#x: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#y: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(12..12 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_major_god: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#cover_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(17..17 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#god_name2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(25..25 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#selection_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(33..33 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#effect1_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(41..41 + 16).unwrap();
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
            r#effect1_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(57..57 + 16).unwrap();
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
            r#effect2_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(73..73 + 16).unwrap();
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
            r#god_name3: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(89..89 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#effect3_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
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
            r#effect3_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(113..113 + 16).unwrap();
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
            r#god_name4: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(129..129 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#effect4_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(137..137 + 16).unwrap();
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
            r#effect4_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(153..153 + 16).unwrap();
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
            r#god_name1: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(169..169 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#effect2_values: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(177..177 + 16).unwrap();
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
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(193..193 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#unknown209: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(209..209 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown225: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(225..225 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown241: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(241..241 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown257: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(257).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown258: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(258..258 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct PantheonPanelLayoutRow {
    pub r#id: String,
    pub r#x: i32,
    pub r#y: i32,
    pub r#is_major_god: bool,
    pub r#cover_image: String,
    pub r#god_name2: String,
    pub r#selection_image: String,
    pub r#effect1_stats: Vec<StatsRef>,
    pub r#effect1_values: Vec<i32>,
    pub r#effect2_stats: Vec<StatsRef>,
    pub r#god_name3: String,
    pub r#effect3_values: Vec<i32>,
    pub r#effect3_stats: Vec<StatsRef>,
    pub r#god_name4: String,
    pub r#effect4_stats: Vec<StatsRef>,
    pub r#effect4_values: Vec<i32>,
    pub r#god_name1: String,
    pub r#effect2_values: Vec<i32>,
    pub r#quest_flag: QuestFlagsRef,
    pub r#unknown209: i64,
    pub r#unknown225: i64,
    pub r#unknown241: i64,
    pub r#unknown257: bool,
    pub r#unknown258: (usize, usize),
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct PantheonPanelLayoutRef(pub usize);

impl Deref for PantheonPanelLayoutRef {
    type Target = PantheonPanelLayoutRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_PantheonPanelLayout[self.0]
    }
}

impl PantheonPanelLayoutRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static PantheonPanelLayoutRow {
        &TABLE_PantheonPanelLayout[self.0]
    }
    pub fn get(&self) -> &'static PantheonPanelLayoutRow {
        &TABLE_PantheonPanelLayout[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_PantheonPanelLayout.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static PantheonPanelLayoutRow)> {
        TABLE_PantheonPanelLayout.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_PantheonPanelLayout.iter() {
            black_box(row);
        }
    }
}
