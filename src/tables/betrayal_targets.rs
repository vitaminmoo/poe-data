#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_BetrayalTargets: LazyLock<Vec<BetrayalTargetsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/betrayaltargets.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| BetrayalTargetsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#betrayal_ranks_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(8..8 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BetrayalRanksRef::new(value as usize)
            },
            r#monster_varieties_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(24..24 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#betrayal_jobs_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BetrayalJobsRef::new(value as usize)
            },
            r#art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown64: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(64).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#item_classes: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(65..65 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                ItemClassesRef::new(value as usize)
            },
            r#full_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(81..81 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#safehouse_arm_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(89..89 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#short_name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(97..97 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown105: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(105..105 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#safehouse_leader_acheivement_items_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(109..109 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#level3_achievement_items_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(125..125 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                AchievementItemsRef::new(value as usize)
            },
            r#unknown141: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(141..141 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown145: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(145..145 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown149: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(149..149 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown153: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(153..153 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#script_argument: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(169..169 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct BetrayalTargetsRow {
    pub r#id: String,
    pub r#betrayal_ranks_key: BetrayalRanksRef,
    pub r#monster_varieties_key: MonsterVarietiesRef,
    pub r#betrayal_jobs_key: BetrayalJobsRef,
    pub r#art: String,
    pub r#unknown64: bool,
    pub r#item_classes: ItemClassesRef,
    pub r#full_name: String,
    pub r#safehouse_arm_file: String,
    pub r#short_name: String,
    pub r#unknown105: i32,
    pub r#safehouse_leader_acheivement_items_key: AchievementItemsRef,
    pub r#level3_achievement_items_key: AchievementItemsRef,
    pub r#unknown141: i32,
    pub r#unknown145: i32,
    pub r#unknown149: i32,
    pub r#unknown153: i64,
    pub r#script_argument: String,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct BetrayalTargetsRef(pub usize);

impl Deref for BetrayalTargetsRef {
    type Target = BetrayalTargetsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_BetrayalTargets[self.0]
    }
}

impl BetrayalTargetsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static BetrayalTargetsRow {
        &TABLE_BetrayalTargets[self.0]
    }
    pub fn get(&self) -> &'static BetrayalTargetsRow {
        &TABLE_BetrayalTargets[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_BetrayalTargets
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static BetrayalTargetsRow)> {
        TABLE_BetrayalTargets
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
        for row in TABLE_BetrayalTargets.iter() {
            black_box(row);
        }
    }
}
