#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_WorldAreas: LazyLock<Vec<WorldAreasRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/worldareas.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| WorldAreasRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#act: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_town: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(20).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#has_waypoint: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(21).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#connections: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(22..22 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 8)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| WorldAreasRef::new(value as usize)).collect()
            },
            r#area_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(38..38 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#hash16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(42..42 + 2).unwrap();
                let value = cell_bytes.get_i16_le();
                value
            },
            r#loading_screens: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#flag_on_entered: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#required_flags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(76..76 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#unknown92: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(92..92 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#topologies: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(96..96 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| TopologiesRef::new(value as usize)).collect()
            },
            r#parent_town: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(112..112 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                WorldAreasRef::new(value as usize)
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown124: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(124..124 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown140: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(140..140 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#bosses: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(156..156 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| MonsterVarietiesRef::new(value as usize)).collect()
            },
            r#unknown172: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(172..172 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown188: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(188..188 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown204: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(204..204 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#is_map_area: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(220).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown221: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(221..221 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#unknown237: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(237..237 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown253: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(253..253 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#area_mods: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(269..269 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| ModsRef::new(value as usize)).collect()
            },
            r#unknown285: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(285..285 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown289: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(289).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#max_level: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(290..290 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown294: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(294..294 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#is_hideout: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(298).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown299: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(299..299 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown303: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(303..303 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#tags: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(307..307 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| TagsRef::new(value as usize)).collect()
            },
            r#unknown323: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(323..323 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown339: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(339..339 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown355: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(355..355 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown359: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(359..359 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown363: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(363..363 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown379: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(379..379 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'string'
                let values = df.strings_from_offset(offset, count).unwrap();
                values
            },
            r#is_unique_map_area: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(395).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown396: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(396..396 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown412: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(412..412 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown428: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(428..428 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown432: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(432..432 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown436: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(436..436 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#environment: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(440..440 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                EnvironmentsRef::new(value as usize)
            },
            r#unknown456: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(456..456 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#terrain_plugins: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(460..460 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                TerrainPluginsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct WorldAreasRow {
    pub r#id: String,
    pub r#name: String,
    pub r#act: i32,
    pub r#is_town: bool,
    pub r#has_waypoint: bool,
    pub r#connections: Vec<WorldAreasRef>,
    pub r#area_level: i32,
    pub r#hash16: i16,
    pub r#loading_screens: Vec<String>,
    pub r#flag_on_entered: Vec<QuestFlagsRef>,
    pub r#required_flags: Vec<QuestFlagsRef>,
    pub r#unknown92: i32,
    pub r#topologies: Vec<TopologiesRef>,
    pub r#parent_town: WorldAreasRef,
    pub r#unknown120: i32,
    pub r#unknown124: i64,
    pub r#unknown140: i64,
    pub r#bosses: Vec<MonsterVarietiesRef>,
    pub r#unknown172: (usize, usize),
    pub r#unknown188: (usize, usize),
    pub r#unknown204: (usize, usize),
    pub r#is_map_area: bool,
    pub r#unknown221: (usize, usize),
    pub r#unknown237: i64,
    pub r#unknown253: i64,
    pub r#area_mods: Vec<ModsRef>,
    pub r#unknown285: i32,
    pub r#unknown289: bool,
    pub r#max_level: i32,
    pub r#unknown294: i32,
    pub r#is_hideout: bool,
    pub r#unknown299: i32,
    pub r#unknown303: i32,
    pub r#tags: Vec<TagsRef>,
    pub r#unknown323: i64,
    pub r#unknown339: i64,
    pub r#unknown355: i32,
    pub r#unknown359: i32,
    pub r#unknown363: i64,
    pub r#unknown379: Vec<String>,
    pub r#is_unique_map_area: bool,
    pub r#unknown396: i64,
    pub r#unknown412: i64,
    pub r#unknown428: i32,
    pub r#unknown432: i32,
    pub r#unknown436: i32,
    pub r#environment: EnvironmentsRef,
    pub r#unknown456: i32,
    pub r#terrain_plugins: TerrainPluginsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct WorldAreasRef(pub usize);

impl Deref for WorldAreasRef {
    type Target = WorldAreasRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_WorldAreas[self.0]
    }
}

impl WorldAreasRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static WorldAreasRow {
        &TABLE_WorldAreas[self.0]
    }
    pub fn get(&self) -> &'static WorldAreasRow {
        &TABLE_WorldAreas[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_WorldAreas.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static WorldAreasRow)> {
        TABLE_WorldAreas.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_WorldAreas.iter() {
            black_box(row);
        }
    }
}
