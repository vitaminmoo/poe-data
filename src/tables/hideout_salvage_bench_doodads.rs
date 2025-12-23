#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_HideoutSalvageBenchDoodads: LazyLock<Vec<HideoutSalvageBenchDoodadsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/hideoutsalvagebenchdoodads.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| HideoutSalvageBenchDoodadsRow {
            r#hideout_doodads: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HideoutDoodadsRef::new(value as usize)
            },
            r#interface_visuals: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                HideoutSalvageBenchInterfaceVisualsRef::new(value as usize)
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct HideoutSalvageBenchDoodadsRow {
    pub r#hideout_doodads: HideoutDoodadsRef,
    pub r#interface_visuals: HideoutSalvageBenchInterfaceVisualsRef,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct HideoutSalvageBenchDoodadsRef(pub usize);

impl Deref for HideoutSalvageBenchDoodadsRef {
    type Target = HideoutSalvageBenchDoodadsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_HideoutSalvageBenchDoodads[self.0]
    }
}

impl HideoutSalvageBenchDoodadsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static HideoutSalvageBenchDoodadsRow {
        &TABLE_HideoutSalvageBenchDoodads[self.0]
    }
    pub fn get(&self) -> &'static HideoutSalvageBenchDoodadsRow {
        &TABLE_HideoutSalvageBenchDoodads[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_HideoutSalvageBenchDoodads.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static HideoutSalvageBenchDoodadsRow)> {
        TABLE_HideoutSalvageBenchDoodads.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_HideoutSalvageBenchDoodads.iter() {
            black_box(row);
        }
    }
}
