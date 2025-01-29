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
};

{
  'src/tables/enums.rs'+:
    std.join(
      '\n',
      ['#![allow(clippy::all)]']
      + ['use serde::{Deserialize, Serialize};']
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
      use serde::{Deserialize, Serialize};
      use std::sync::LazyLock;
      use std::ops::Deref;
      #[allow(unused_imports)]
      use super::*;
      #[derive(Debug)]
      pub struct %(tableName)s {
      %(field_types)s
      }

      #[allow(non_upper_case_globals)]
      pub static TABLE_%(tableName)s: LazyLock<Vec<%(tableName)s>> =
          LazyLock::new(|| {
              RAW_TABLE_%(tableName)s
                  .iter()
                  .map(|x| {%(tableName)s {
                      %(field_values)s
                  }})
                  .collect()
          });

      #[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
      pub struct %(tableName)sRow(pub usize);

      impl Deref for %(tableName)sRow {
          type Target = %(tableName)s;

          fn deref(&self) -> &'static Self::Target {
              &TABLE_%(tableName)s[self.0]
          }
      }

      impl %(tableName)sRow {
          pub fn new(index: usize) -> Self {
              Self(index)
          }
          pub fn as_static_ref(&self) -> &'static %(tableName)s {
              &TABLE_%(tableName)s[self.0]
          }
          pub fn get(&self) -> &'static %(tableName)s {
              &TABLE_%(tableName)s[self.0]
          }
          pub fn iter() -> impl Iterator<Item = Self> {
              TABLE_%(tableName)s.iter().enumerate().map(|(i, _)| Self(i))
          }
          pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static %(tableName)s)> {
              TABLE_%(tableName)s.iter().enumerate().map(|(i, x)| (Self(i), x))
          }
      }

      #[allow(non_upper_case_globals)]
      static RAW_TABLE_%(tableName)s: LazyLock<
          Vec<%(tableName)sRaw>,
      > = LazyLock::new(|| {
          const DATA: &str = include_str!("../../data/tables/English/%(tableName)s.json");
          serde_json::from_str(DATA).unwrap()
      });

      #[derive(Debug, Deserialize, Serialize)]
      struct %(tableName)sRaw {
      %(raw_field_types)s
      }

    ||| % {
      local columns = types.columns_from_table(table),
      tableName: table.name,
      field_types: std.join('\n', columns.field_types),
      raw_field_types: std.join('\n', columns.raw_field_types),
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
