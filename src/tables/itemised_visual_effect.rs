#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_ItemisedVisualEffect: LazyLock<Vec<ItemisedVisualEffectRow>> =
    LazyLock::new(|| {
        let df = DAT_LOADER
            .write()
            .unwrap()
            .get_table("data/balance/itemisedvisualeffect.datc64")
            .unwrap()
            .clone();
        df.rows_iter()
            .map(|row| ItemisedVisualEffectRow {
                r#effect_base_type: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(0..0 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    BaseItemTypesRef::new(value as usize)
                },
                r#visual_effect: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(16..16 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ItemVisualEffectRef::new(value as usize)
                },
                r#visual_identity: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(32..32 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    ItemVisualIdentityRef::new(value as usize)
                },
                r#stats: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(48..48 + 16).unwrap();
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
                r#item_classes: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(64..64 + 16).unwrap();
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
                r#unknown80: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
                r#unknown96: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(96).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown97: {
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
                r#unknown113: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(113..113 + 16).unwrap();
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
                r#unknown129: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(129).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown130: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(130..130 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown146: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(146).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown147: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(147..147 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown163: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(163..163 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown179: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(179..179 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown195: {
                    // array_mutator column.array == true
                    let mut cell_bytes = row.get(195..195 + 16).unwrap();
                    let count = cell_bytes.get_u64_le() as usize;
                    let offset = cell_bytes.get_u64_le() as usize;
                    // array_mutator column.array == true && column.type == 'array'
                    let values = (count, offset);
                    values
                },
                r#unknown211: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(211..211 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown227: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(227).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown228: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(228..228 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown244: {
                    // array_mutator column.array == false && column.type != 'string|bool'
                    let mut cell_bytes = row.get(244..244 + 16).unwrap();
                    let value = cell_bytes.get_i64_le();
                    value
                },
                r#unknown260: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(260).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
                r#unknown261: {
                    // array_mutator column.array == false && column.type == 'bool'
                    let cell_bytes = row.get(261).unwrap();
                    let value = cell_bytes.to_le() != 0;
                    value
                },
            })
            .collect()
    });

#[derive(Debug)]
pub struct ItemisedVisualEffectRow {
    pub r#effect_base_type: BaseItemTypesRef,
    pub r#visual_effect: ItemVisualEffectRef,
    pub r#visual_identity: ItemVisualIdentityRef,
    pub r#stats: Vec<StatsRef>,
    pub r#item_classes: Vec<ItemClassesRef>,
    pub r#unknown80: Vec<i64>,
    pub r#unknown96: bool,
    pub r#unknown97: Vec<i32>,
    pub r#unknown113: Vec<i32>,
    pub r#unknown129: bool,
    pub r#unknown130: (usize, usize),
    pub r#unknown146: bool,
    pub r#unknown147: (usize, usize),
    pub r#unknown163: (usize, usize),
    pub r#unknown179: (usize, usize),
    pub r#unknown195: (usize, usize),
    pub r#unknown211: i64,
    pub r#unknown227: bool,
    pub r#unknown228: i64,
    pub r#unknown244: i64,
    pub r#unknown260: bool,
    pub r#unknown261: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct ItemisedVisualEffectRef(pub usize);

impl Deref for ItemisedVisualEffectRef {
    type Target = ItemisedVisualEffectRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_ItemisedVisualEffect[self.0]
    }
}

impl ItemisedVisualEffectRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static ItemisedVisualEffectRow {
        &TABLE_ItemisedVisualEffect[self.0]
    }
    pub fn get(&self) -> &'static ItemisedVisualEffectRow {
        &TABLE_ItemisedVisualEffect[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_ItemisedVisualEffect
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static ItemisedVisualEffectRow)> {
        TABLE_ItemisedVisualEffect
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
        for row in TABLE_ItemisedVisualEffect.iter() {
            black_box(row);
        }
    }
}
