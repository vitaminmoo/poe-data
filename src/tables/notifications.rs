#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Notifications: LazyLock<Vec<NotificationsRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/notifications.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| NotificationsRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown8: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(8).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown9: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(9).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#message: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(10..10 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown18: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(18..18 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown26: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(26..26 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown30: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(30).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown31: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(31).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NotificationsRow {
    pub r#id: String,
    pub r#unknown8: bool,
    pub r#unknown9: bool,
    pub r#message: String,
    pub r#unknown18: String,
    pub r#unknown26: i32,
    pub r#unknown30: bool,
    pub r#unknown31: bool,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NotificationsRef(pub usize);

impl Deref for NotificationsRef {
    type Target = NotificationsRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Notifications[self.0]
    }
}

impl NotificationsRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NotificationsRow {
        &TABLE_Notifications[self.0]
    }
    pub fn get(&self) -> &'static NotificationsRow {
        &TABLE_Notifications[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Notifications.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NotificationsRow)> {
        TABLE_Notifications.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_Notifications.iter() {
            black_box(row);
        }
    }
}
