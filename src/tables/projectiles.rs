#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Projectiles: LazyLock<Vec<ProjectilesRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/projectiles.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| ProjectilesRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_files: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#loop_animation_ids: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#impact_animation_ids: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#projectile_speed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(56..56 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown60: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(60).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown61: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(61..61 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown65: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(65).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown66: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(66).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#inherits_from: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(67..67 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#misc_animated: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(75..75 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MiscAnimatedRef::new(value as usize)
            },
            r#unknown91: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(91..91 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown95: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(95).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown96: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(96).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#stuck_ao_file: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(97..97 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#bounce_ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(113..113 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown121: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(121..121 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown125: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(125..125 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown129: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(129..129 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown133: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(133..133 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown137: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(137..137 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown153: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(153..153 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown169: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(169..169 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown173: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(173..173 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown177: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(177..177 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown181: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(181..181 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown185: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(185..185 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown189: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(189..189 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown205: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(205).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown206: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(206..206 + 16).unwrap();
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
            r#unknown222: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(222..222 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown230: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(230..230 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown246: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(246..246 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown262: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(262..262 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown278: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(278).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown279: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(279..279 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown287: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(287..287 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown303: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(303..303 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown311: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(311..311 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown327: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(327..327 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown335: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(335..335 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown351: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(351..351 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#unknown367: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(367).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct ProjectilesRow {
    pub r#id: String,
    pub r#ao_files: Vec<String>,
    pub r#loop_animation_ids: Vec<String>,
    pub r#impact_animation_ids: Vec<String>,
    pub r#projectile_speed: i32,
    pub r#unknown60: bool,
    pub r#unknown61: i32,
    pub r#unknown65: bool,
    pub r#unknown66: bool,
    pub r#inherits_from: String,
    pub r#misc_animated: MiscAnimatedRef,
    pub r#unknown91: i32,
    pub r#unknown95: bool,
    pub r#unknown96: bool,
    pub r#stuck_ao_file: Vec<String>,
    pub r#bounce_ao_file: String,
    pub r#unknown121: i32,
    pub r#unknown125: i32,
    pub r#unknown129: i32,
    pub r#unknown133: i32,
    pub r#unknown137: i64,
    pub r#unknown153: i64,
    pub r#unknown169: i32,
    pub r#unknown173: i32,
    pub r#unknown177: i32,
    pub r#unknown181: i32,
    pub r#unknown185: i32,
    pub r#unknown189: Vec<String>,
    pub r#unknown205: bool,
    pub r#unknown206: Vec<i32>,
    pub r#unknown222: String,
    pub r#unknown230: i64,
    pub r#unknown246: i64,
    pub r#unknown262: i64,
    pub r#unknown278: bool,
    pub r#unknown279: String,
    pub r#unknown287: i64,
    pub r#unknown303: String,
    pub r#unknown311: i64,
    pub r#unknown327: String,
    pub r#unknown335: Vec<String>,
    pub r#unknown351: Vec<String>,
    pub r#unknown367: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ProjectilesRef(pub usize);

impl Deref for ProjectilesRef {
    type Target = ProjectilesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Projectiles[self.0]
    }
}

impl ProjectilesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ProjectilesRow {
        &TABLE_Projectiles[self.0]
    }
    pub fn get(&self) -> &'static ProjectilesRow {
        &TABLE_Projectiles[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Projectiles.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ProjectilesRow)> {
        TABLE_Projectiles
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Projectiles.iter() {
            black_box(row);
        }
    }
}
