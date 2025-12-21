#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutSalvageBenchInterfaceVisuals: LazyLock<
    Vec<HideoutSalvageBenchInterfaceVisualsRow>,
> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hideoutsalvagebenchinterfacevisuals.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HideoutSalvageBenchInterfaceVisualsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#background_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#hammer_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#button_art: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
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
            r#unknown40: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown44: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutSalvageBenchInterfaceVisualsRow {
    pub r#id: String,
    pub r#background_art: String,
    pub r#hammer_art: String,
    pub r#button_art: String,
    pub r#unknown32: i32,
    pub r#unknown36: i32,
    pub r#unknown40: i32,
    pub r#unknown44: i32,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutSalvageBenchInterfaceVisualsRef(pub usize);

impl Deref for HideoutSalvageBenchInterfaceVisualsRef {
    type Target = HideoutSalvageBenchInterfaceVisualsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutSalvageBenchInterfaceVisuals[self.0]
    }
}

impl HideoutSalvageBenchInterfaceVisualsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutSalvageBenchInterfaceVisualsRow {
        &TABLE_HideoutSalvageBenchInterfaceVisuals[self.0]
    }
    pub fn get(&self) -> &'static HideoutSalvageBenchInterfaceVisualsRow {
        &TABLE_HideoutSalvageBenchInterfaceVisuals[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutSalvageBenchInterfaceVisuals
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs(
    ) -> impl Iterator<Item = (Self, &'static HideoutSalvageBenchInterfaceVisualsRow)> {
        TABLE_HideoutSalvageBenchInterfaceVisuals
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
        for row in TABLE_HideoutSalvageBenchInterfaceVisuals.iter() {
            black_box(row);
        }
    }
}
