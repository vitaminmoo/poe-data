local config = import 'config.libsonnet';
local types = import 'types.libsonnet';
local util = import 'util.libsonnet';

{
  'src/tables/enums.rs'+:
    std.join(
      '\n',
      ['#![allow(clippy::all)]']
      + ['use serde::{Deserialize, Serialize};']
      + ['#[allow(unused_imports)]']
      + ['use std::ops::Deref;']
      + ['use strum::{EnumIter, FromRepr, Display};']
      + [|||
        #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
        pub enum MaybeVariant<T> {
            Variant(T),
            NotVariant(usize),
        }

        impl<T> MaybeVariant<T> {
            pub fn unwrap(self) -> T {
                match self {
                    MaybeVariant::Variant(x) => x,
                    MaybeVariant::NotVariant(x) => panic!("Not an enum variant {}", x),
                }
            }
        }
      |||]
      + [
        |||
          #[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display)]
          pub enum %(enumname)s {
          %(enumerators)s
          }
        ||| % {
          enumname: enumeration.name,
          enumerators: std.join('\n', [
            util.enumeratorToVariant(enumeration.indexing, idx, enumeration.enumerators[idx])
            for idx in std.range(0, std.length(enumeration.enumerators) - 1)
            if enumeration.enumerators[idx] != null
          ]),
        }
        for enumeration in config.enums
        if enumeration.name != null
      ],
    ),
} + {
  ['src/tables/%s.rs' % util.case.snake(table.name)]+:
    |||
      #![allow(clippy::all)]
      use bytes::Buf;

      use crate::dat_parser::DAT_LOADER;

      #[allow(unused)]
      use super::*;
      use std::{ops::Deref, sync::LazyLock};

      #[allow(non_upper_case_globals)]
      pub static TABLE_%(tableName)s: LazyLock<Vec<%(tableName)sRow>> = LazyLock::new(|| {
          let df = DAT_LOADER
              .write()
              .unwrap()
              .get_table("%(datPath)s/%(tableNameLC)s.datc64")
              .unwrap()
              .clone();
          df.rows_iter()
              .map(|row| %(tableName)sRow {
                %(field_values)s
              })
              .collect()
      });

      #[derive(Debug)]
      pub struct %(tableName)sRow {
          %(field_types)s
      }

      #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
      pub struct %(tableName)sRef(pub usize);

      impl Deref for %(tableName)sRef {
          type Target = %(tableName)sRow;
          fn deref(&self) -> &'static Self::Target {
              &TABLE_%(tableName)s[self.0]
          }
      }

      impl %(tableName)sRef {
          pub fn new(index: usize) -> Self {
              Self(index)
          }
          pub fn as_static_ref(&self) -> &'static %(tableName)sRow {
              &TABLE_%(tableName)s[self.0]
          }
          pub fn get(&self) -> &'static %(tableName)sRow {
              &TABLE_%(tableName)s[self.0]
          }
          pub fn iter() -> impl Iterator<Item = Self> {
              TABLE_%(tableName)s.iter().enumerate().map(|(i, _)| Self(i))
          }
          pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static %(tableName)sRow)> {
              TABLE_%(tableName)s.iter().enumerate().map(|(i, x)| (Self(i), x))
          }
      }

      #[cfg(test)]
      mod test {
          use super::*;
          use std::hint::black_box;
          #[test]
          fn get_all_rows() {
              for row in TABLE_%(tableName)s.iter() {
                  black_box(row);
              }
          }
      }
    ||| % {
      tableName: table.name,
      tableNameLC: std.asciiLower(table.name),
      field_types: std.join('\n', ['pub %(name_field)s: %(return_type)s,' % column for column in table.columns]),
      field_values: std.join('\n', ['%(name_field)s: { %(cell_read)s },' % column for column in table.columns]),
      datPath: config.datPath,
    }
  for table in config.tables
} + {
  'src/tables.rs'+: std.join(
    '\n',
    [
      'pub use enums::*;',
    ] + [
      'pub use %(table.name)s::*;' % util.case.snake(table.name)
      for table in config.tables
    ] + [
      'pub mod enums;',
    ]
    + [
      'pub mod %(table.name)s;' % util.case.snake(table.name)
      for table in config.tables
    ]
  ),
}
