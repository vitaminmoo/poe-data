local config = import 'config.libsonnet';
local schema = import 'schema.min.json';
local types = import 'types.libsonnet';
local util = import 'util.libsonnet';

local tables = [
  config.kvSchema[table.name]
  for table in config.tables
];

local kvEnum = {
  [enumeration.name]: enumeration
  for enumeration in schema.enumerations
  if enumeration.validFor == 2 || enumeration.validFor == 3
};

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
        for enumeration in std.objectValues(kvEnum)
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
              .get_table("data/%(tableName)s.datc64")
              .unwrap()
              .clone();

          df.rows_iter()
              .map(|row| %(tableName)sRow {
                %(field_values)s
               /*
                  r#id: df
                      .string_from_offset(row.get(0..8).unwrap().get_i32_le() as usize)
                      .unwrap(),
                  r#ui_title: df
                      .string_from_offset(row.get(8..16).unwrap().get_i32_le() as usize)
                      .unwrap(),
                  r#act_number: row.get(16..20).unwrap().get_i32_le(),
                  r#is_end_game: row.get(40).unwrap().to_le() != 0,
                  //r#unknown_int: row.get(41..43).unwrap().get_i16_le(),

                  //r#unknown_foreign_array: df
                  //    .array_from_offset(
                  //        row.get(53..59).unwrap().get_i32_le() as usize,
                  //        row.get(45..51).unwrap().get_i32_le() as usize,
                  //        16,
                  //    )
                  //    .unwrap()
                  //    .iter()
                  //    .map(|x| x.clone().get_i32_le())
                  //    .collect(),
                  r#description: df
                      .string_from_offset(row.get(125..131).unwrap().get_i32_le() as usize)
                      .unwrap(),
                      */
              })
              .collect()
      });

      #[derive(Debug)]
      pub struct %(tableName)sRow {
          %(field_types)s
          //pub r#id: String,
          //pub r#ui_title: String,
          //pub r#act_number: i32,
          //pub r#is_end_game: bool,
          //pub r#unknown_int: i16,
          //pub r#unknown_foreign_array: Vec<i32>,
          //pub r#description: String,
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
    ||| % {
      local columns = types.columns_from_table(table),
      tableName: table.name,
      field_types: std.join('\n', columns.field_types),
      field_values: std.join('\n', columns.field_values),
    }
  for table in tables
} + {
  'src/tables.rs'+: std.join(
    '\n',
    [
      'pub use enums::*;',
    ] + [
      'pub use %(table.name)s::*;' % util.case.snake(table.name)
      for table in tables
    ] + [
      'pub mod enums;',
    ]
    + [
      'pub mod %(table.name)s;' % util.case.snake(table.name)
      for table in tables
    ]
  ),
}
