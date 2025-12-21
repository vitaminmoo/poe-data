#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_AbyssBenchTicketTypes: LazyLock<Vec<AbyssBenchTicketTypesRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/abyssbenchtickettypes.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| AbyssBenchTicketTypesRow {
                r#base_item_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#usable_on_item_classes: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
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
                        .map(|value| ItemClassesRef::new(value as usize))
                        .collect()
                },
                r#unknown32: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown36: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(36..36 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#maximum_item_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(40..40 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#minimum_mod_level: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(44..44 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown48: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(48..48 + 4).unwrap();
                    let value = cell_bytes.get_i32_le();
                    value
                },
                r#unknown52: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(52).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct AbyssBenchTicketTypesRow {
    pub r#base_item_type: BaseItemTypesRef,
    pub r#usable_on_item_classes: Vec<ItemClassesRef>,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#maximum_item_level: i32,
    pub r#minimum_mod_level: i32,
    pub r#unknown48: i32,
    pub r#unknown52: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct AbyssBenchTicketTypesRef(pub usize);

impl Deref for AbyssBenchTicketTypesRef {
    type Target = AbyssBenchTicketTypesRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_AbyssBenchTicketTypes[self.0]
    }
}

impl AbyssBenchTicketTypesRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static AbyssBenchTicketTypesRow {
        &TABLE_AbyssBenchTicketTypes[self.0]
    }
    pub fn get(&self) -> &'static AbyssBenchTicketTypesRow {
        &TABLE_AbyssBenchTicketTypes[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_AbyssBenchTicketTypes
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static AbyssBenchTicketTypesRow)> {
        TABLE_AbyssBenchTicketTypes
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
        for row in TABLE_AbyssBenchTicketTypes.iter() {
            black_box(row);
        }
    }
}
