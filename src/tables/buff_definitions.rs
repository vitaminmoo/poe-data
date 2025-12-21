#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BuffDefinitions: LazyLock<Vec<BuffDefinitionsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/buffdefinitions.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BuffDefinitionsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#invisible: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(16).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#removable: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(17).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(26..26 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#cancel_on_death: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(42).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#merge_mode: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(43..43 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                BuffMergeModes::from_repr(value as usize)
            },
            r#show_count: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(47).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#maximum_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#current_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown80: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(80).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown81: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(81..81 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#buff_visual: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(85..85 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BuffVisualsRef::new(value as usize)
            },
            r#unknown101: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(101).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown102: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(102).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#buff_category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(103..103 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown107: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(107).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown108: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(108).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown109: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(109).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown110: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(110).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#buff_limit: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(111..111 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown115: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(115).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#implementation: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(116..116 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#is_recovery: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(124).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown125: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(125).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#min_stat: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(126..126 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown142: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(142).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#ui_stack_mode: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(143..143 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown147: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(147).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#is_skill_buff: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(148).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown149: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(149..149 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown153: {
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
            r#unknown169: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(169).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown170: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(170).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown171: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(171..171 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
            r#unknown187: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(187).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stats2: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(188..188 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#granted_flags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(204..204 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#granted_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(220..220 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#condition_stats: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(236..236 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#unknown252: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(252).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown253: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(253).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown254: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(254).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown255: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(255).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown256: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(256..256 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                StatsRef::new(value as usize)
            },
            r#unknown272: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(272..272 + 16).unwrap();
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
            r#unknown288: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(288..288 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown296: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(296..296 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
                    .into_iter()
                    .map(|value| StatsRef::new(value as usize))
                    .collect()
            },
            r#unknown312: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(312..312 + 16).unwrap();
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
            r#unknown328: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(328..328 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown332: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(332..332 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
            r#unknown348: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(348).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown349: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(349).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BuffDefinitionsRow {
    pub r#id: String,
    pub r#description: String,
    pub r#invisible: bool,
    pub r#removable: bool,
    pub r#name: String,
    pub r#stats: Vec<StatsRef>,
    pub r#cancel_on_death: bool,
    pub r#merge_mode: Option<BuffMergeModes>,
    pub r#show_count: bool,
    pub r#maximum_stat: StatsRef,
    pub r#current_stat: StatsRef,
    pub r#unknown80: bool,
    pub r#unknown81: i32,
    pub r#buff_visual: BuffVisualsRef,
    pub r#unknown101: bool,
    pub r#unknown102: bool,
    pub r#buff_category: i32,
    pub r#unknown107: bool,
    pub r#unknown108: bool,
    pub r#unknown109: bool,
    pub r#unknown110: bool,
    pub r#buff_limit: i32,
    pub r#unknown115: bool,
    pub r#implementation: String,
    pub r#is_recovery: bool,
    pub r#unknown125: bool,
    pub r#min_stat: StatsRef,
    pub r#unknown142: bool,
    pub r#ui_stack_mode: i32,
    pub r#unknown147: bool,
    pub r#is_skill_buff: bool,
    pub r#unknown149: i32,
    pub r#unknown153: Vec<i32>,
    pub r#unknown169: bool,
    pub r#unknown170: bool,
    pub r#unknown171: Vec<i64>,
    pub r#unknown187: bool,
    pub r#stats2: Vec<StatsRef>,
    pub r#granted_flags: Vec<StatsRef>,
    pub r#granted_stats: Vec<StatsRef>,
    pub r#condition_stats: Vec<StatsRef>,
    pub r#unknown252: bool,
    pub r#unknown253: bool,
    pub r#unknown254: bool,
    pub r#unknown255: bool,
    pub r#unknown256: StatsRef,
    pub r#unknown272: Vec<i32>,
    pub r#unknown288: String,
    pub r#unknown296: Vec<StatsRef>,
    pub r#unknown312: Vec<i32>,
    pub r#unknown328: i32,
    pub r#unknown332: Vec<i64>,
    pub r#unknown348: bool,
    pub r#unknown349: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BuffDefinitionsRef(pub usize);

impl Deref for BuffDefinitionsRef {
    type Target = BuffDefinitionsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BuffDefinitions[self.0]
    }
}

impl BuffDefinitionsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BuffDefinitionsRow {
        &TABLE_BuffDefinitions[self.0]
    }
    pub fn get(&self) -> &'static BuffDefinitionsRow {
        &TABLE_BuffDefinitions[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BuffDefinitions
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BuffDefinitionsRow)> {
        TABLE_BuffDefinitions
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_BuffDefinitions.iter() {
            println!("{:?}", row);
        }
    }
}
