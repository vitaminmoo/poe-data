local config = import 'config.libsonnet';
local util = import 'util.libsonnet';

local rename = '#[serde(rename = "%(column_name)s")]';
local not_null = function(x) x != null;
local any = function(x) true;

{
  array_mutator(column):: (
    column + if column.array
    then {
      cell_read+: (
        |||
          // array_mutator column.array == true
          let mut cell_bytes = row.get(%(offset)s..%(offset)s + %(cell_bytes)s).unwrap();
          let count = cell_bytes.get_u64_le() as usize;
          let offset = cell_bytes.get_u64_le() as usize;
        ||| + (
          if column.type == 'string' then |||
            // array_mutator column.array == true && column.type == 'string'
            let values = df.strings_from_offset(offset, count).unwrap();
            %(value)s
          ||| else if column.type == 'array' then |||
            // array_mutator column.array == true && column.type == 'array'
            let values = (count, offset);
            %(value)s
          ||| else |||
            // array_mutator column.array == true && column.type else
            let values = df.array_from_offset(offset, count, %(value_bytes)s).unwrap().iter().map(|x| x.clone().get_%(value_type)s_le()).collect::<Vec<%(value_type)s>>();
            %(value)s
          |||
        )
      ) % column,
      cell_type: '(i32, i32)',
      value_type: if column.type == 'array' then column.cell_type else 'Vec<%s>' % column.cell_type,
      local return_type = std.get(column, 'return_type', column.value_type),
      return_type: if column.type == 'array' then return_type else 'Vec<%s>' % return_type,
    } else {
      cell_read+:
        (
          if column.type == 'string' then |||
            // array_mutator column.array == false && column.type == 'string'
            let mut cell_bytes = row.get(%(offset)s..%(offset)s + %(cell_bytes)s).unwrap();
            let offset = cell_bytes.get_i32_le() as usize;
            let value = df.string_from_offset(offset).unwrap();
            %(value)s
          |||
          else if column.type == 'bool' then |||
            // array_mutator column.array == false && column.type == 'bool'
            let cell_bytes = row.get(%(offset)s).unwrap();
            let value = cell_bytes.to_le() != 0;
            %(value)s
          |||
          else if column.type == 'interval' then |||
            // array_mutator column.array == false && column.type == 'interval'
            let mut cell_bytes = row.get(%(offset)s..%(offset)s + %(cell_bytes)s).unwrap();
            let value = (cell_bytes.get_i32_le(), cell_bytes.get_i32_le());
            %(value)s
          |||
          else |||
            // array_mutator column.array == false && column.type != 'string|bool'
            let mut cell_bytes = row.get(%(offset)s..%(offset)s + %(cell_bytes)s).unwrap();
            let value = cell_bytes.get_%(cell_type)s_le();
            %(value)s
          |||
        ) % column,
      return_type: std.get(column, 'return_type', column.value_type),
    }
  ),
  type_mutator(column):: (
    local type_to_cell_type = {
      array: '(usize, usize)',  // count, offset - for unknown arrays
      bool: 'i8',
      enumrow: 'i32',  // index in enum table
      f32: 'f32',
      f64: 'f64',
      foreignrow: 'i64',  // index in foreign table
      i16: 'i16',
      i32: 'i32',
      i64: 'i64',
      interval: '(i32, i32)',
      row: 'i64',  // index in current table
      string: 'i32',  // offset in vdata
      u16: 'u16',
      u32: 'u32',
      u64: 'u64',
    };
    local type_to_value_type = type_to_cell_type {
      bool: 'bool',
      string: 'String',  // offset in vdata
    };
    local type_to_return_type = type_to_value_type {
      enumrow: 'Option<%s>' % column.references.table,  // index in enum table
      foreignrow: '%sRef' % column.references.table,  // index in foreign table
      row: '%sRef' % column.references.table,  // index in current table
    };
    column {
      local this = self,
      value: if column.array then 'values' else 'value',
      // the type in the actual bytes of the column or array
      cell_type: std.get(type_to_cell_type, column.type, error 'unknown schema type %(type)s in column %(name)s' % column),
      // the type that exists once we either read the cell, read the string, or read the array
      value_type: std.get(type_to_value_type, column.type, error 'unknown schema type %(type)s in column %(name)s' % column),
    } + if column.references != null && std.objectHas(column.references, 'table') && !std.objectHas(column.references, 'column') then
      ({
         // the type that we actually return after dereferencing table references
         return_type: std.get(type_to_return_type, column.type, error 'unknown schema type %(type)s in column %(name)s' % column),
         value: if column.type == 'enumrow' then (
           '%s::from_repr(value as usize)' % column.references.table
         ) else if std.contains(['row', 'foreignrow'], column.type) then (
           '%sRef::new(value as usize)' % column.references.table
         ) else super.value,
       } + {
         value:
           if column.array then
             'values.into_iter().map(|value| %s).collect()' % [super.value]
           else super.value,
       }) else {}
  ),
  schema_type_to_array(t):: (
    local tm = {
      array: '(i32, i32)',
      string: 'Vec<String>',
      default: 'Vec<' + t + '>',
    };
    std.get(tm, t, tm.default)
  ),
  sizeof_map:: {
    i16: 2,
    u16: 2,
    i32: 4,
    u32: 4,
    u64: 8,
    string: 8,
    bool: 1,
    f32: 4,
    row: 8,
    foreignrow: 16,
    enumrow: 4,
    array: 16,
    interval: 8,
  },
  sizeof(column)::
    std.get($.sizeof_map, column.type, error 'unknown size for type %s' % column.type),
  sizeof_cell(column)::
    if column.array then 16 else $.sizeof(column),
  sizeof_value(column)::
    if column.type == 'string' then 0 else $.sizeof(column),
  scanl(func, arr, init)::
    if std.length(arr) == 0 then [init] else (
      local result = func(arr[0], init);
      [init] + $.scanl(func, arr[1:], result)
    ),
  mutate_columns(columns):: (
    local valid_columns = std.filter(function(c) !std.objectHas(c, 'errors') || std.length(c.errors) == 0, columns);
    local initial = { offset: 0, column: [] };
    [x.column for x in $.scanl(
      function(column, acc) (
        local current_offset = acc.offset;
        {
          offset: current_offset + self.column.cell_bytes,
          column: $.array_mutator($.type_mutator(column {
            name: if column.name != null then column.name else 'Unknown%d' % current_offset,
            name_snake: util.case.snake(self.name),
            name_field: 'r#' + self.name_snake,
            type:
              if super.interval
              then 'interval'
              //else if self.type == 'row' || self.type == 'foreignrow'
              //then 'reference'
              //else if self.file != null
              //then 'file'
              //else if self.files != null
              //then 'directory'
              else super.type,
            references:
              if super.references != null &&
                 std.objectHas(super.references, 'table') &&
                 std.contains(config.validReferences, super.references.table)
              then super.references
              else null,
            //cell_bytes: $.sizeof_cell(self),
            value_bytes: $.sizeof_value(self),
            //offset: current_offset,
            // hide shit I don't care about
            description:: super.description,
            files:: super.files,
            file:: super.file,
            interval:: super.interval,
            localized:: super.localized,
            unique:: super.unique,
            until:: super.until,
          })),
        }
      ),
      valid_columns,
      initial
    )[1:]]
  ),
}
