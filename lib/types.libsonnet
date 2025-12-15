local util = import 'util.libsonnet';

local rename = '#[serde(rename = "%(column_name)s")]';
local not_null = function(x) x != null;
local any = function(x) true;

local types =
  [
    // row_ref
    [{
      type: ['row', 'foreignrow'],
      array: false,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'row_ref',
      field_type+: ['pub r#%(snake_column_name)s: Option<%(target_table_name)sRow>,'],
      field_value+: ['// row_ref %(snake_column_name)s'],
    }],
    // row_ref_array
    [{
      type: ['row', 'foreignrow'],
      array: true,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'row_ref_array',
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(target_table_name)sRow>,'],
      field_value+: ['// row_ref_array %(snake_column_name)s'],
    }],
    // column_ref
    [{
      type: ['i32', 'u32', 'string', 'f32'],
      array: false,
      table_reference: not_null,
      column_reference: not_null,
    }, {
      type: 'column_ref',
      field_type+: ['// column_ref %(snake_column_name)s'],
      field_value+: ['// column_ref %(snake_column_name)s'],
    }],
    // column_ref_array
    [{
      type: ['i32', 'u32', 'string', 'f32'],
      array: true,
      table_reference: not_null,
      column_reference: not_null,
    }, {
      type: 'column_ref_array',
      field_type+: ['// column_ref_array %(snake_column_name)s'],
      field_value+: ['// column_ref_array %(snake_column_name)s'],
    }],
    // row_ref_unknown
    [{
      type: ['foreignrow'],
      array: any,
      table_reference: null,
      column_reference: any,
    }, {
      type: 'row_ref_unknown',
      field_type+: ['// row_ref_unknown %(snake_column_name)s'],
      field_value+: ['// row_ref_unknown %(snake_column_name)s'],
    }],
    // enum_ref
    [{
      type: 'enumrow',
      array: false,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'enum_ref',
      field_type+: ['pub r#%(snake_column_name)s: MaybeVariant<%(target_table_name)s>,'],
      field_value+: ['// enum_ref %(snake_column_name)s'],
    }],
    // enum_ref_array
    [{
      type: 'enumrow',
      array: true,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'enum_ref_array',
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(target_table_name)s>,'],
      field_value+: ['// enum_ref_array%(snake_column_name)s'],
    }],
    // basic
    [{
      type: ['i16', 'u16', 'f32', 'i32', 'u32', 'bool'],
      array: false,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'basic',
      field_type+: ['pub r#%(snake_column_name)s: %(type)s,'],
      field_value+: ['r#%(snake_column_name)s: %(getter)s, // basic'],
    }],
    // basic_array
    [{
      type: ['i16', 'u16', 'f32', 'i32', 'u32', 'bool'],
      array: true,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'basic_array',
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(type)s>,'],
      field_value+: ['// basic_array %(snake_column_name)s'],
    }],
    // string
    [{
      type: ['string'],
      array: false,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'string',
      field_type+: ['pub r#%(snake_column_name)s: %(type)s,'],
      field_value+: ['r#%(snake_column_name)s: %(getter)s, // string'],
    }],
    // string_array
    [{
      type: ['string'],
      array: true,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'string',
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(type)s>,'],
      field_value+: ['// string %(snake_column_name)s'],
    }],
  ];

{
  type_map(t):: (
    local tm = {
      i16: 'i16',
      u16: 'u16',
      i32: 'i32',
      u32: 'u32',
      string: 'String',
      bool: 'bool',
      f32: 'f32',
    };
    if std.objectHas(tm, t) then tm[t] else error 'unknown type %s' % t
  ),
  getter_map(t):: (
    local tm = {
      i16: 'row.get(%(offset)d..%(offset)d + 2).unwrap().get_i16_le()',
      u16: 'row.get(%(offset)d..%(offset)d + 2).unwrap().get_u16_le()',
      i32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_i32_le()',
      u32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_u32_le()',
      string: 'df.string_from_offset(row.get(%(offset)d..%(offset)d + 8).unwrap().get_i32_le() as usize).unwrap()',
      bool: 'row.get(%(offset)d).unwrap().to_le() != 0',
      f32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_f32_le()',
    };
    if std.objectHas(tm, t) then tm[t] else error 'unknown type %s' % t
  ),
  sizeof_map:: {
    i16: 2,
    u16: 2,
    i32: 4,
    u32: 4,
    string: 8,
    bool: 1,
    f32: 4,
    row: 8,
    foreignrow: 8,
    enumrow: 4,
  },
  sizeof(column):: (
    if column.array then
      8
    else if std.objectHas($.sizeof_map, column.type) then
      $.sizeof_map[column.type]
    else
      error 'unknown size for type %s' % column.type
  ),
  match(item, matcher):: (
    local type = std.type(matcher);
    if type == 'string'
       || type == 'boolean'
       || type == 'number'
       || type == 'null'
       || (type == 'array' && matcher == [])
    then item == matcher  // literal match
    else if type == 'array'
    then std.setMember(item, std.set(matcher))  // any of the items in the set
    else if type == 'function'
    then matcher(item)  // function matcher
    else error 'invalid matcher type %s' % type
  ),
  column_match(column, filter):: (
    $.match(column.type, filter.type)
    && $.match(column.array, filter.array)
    && $.match(
      if column.references != null && std.objectHas(column.references, 'table')
      then column.references.table
      else null,
      filter.table_reference,
    )
    && $.match(
      if column.references != null && std.objectHas(column.references, 'column')
      then column.references.column
      else null,
      filter.column_reference,
    )
  ),
  columns_from_table(table):: (
    local columns = [c for c in table.columns];
    local initial = { offset: 0, fields: { field_types: [], field_values: [] } };
    local result = std.foldl(
      function(acc, column) (
        local current_offset = acc.offset;
        local new_fields = if column.name != null then (
          local processed_column = $.column(table.name, column, current_offset);
          if processed_column != null then
            {
              field_types+: if processed_column.field_type != null then processed_column.field_type else [],
              field_values+: if processed_column.field_value != null then processed_column.field_value else [],
            }
          else
            {}
        ) else {};

        local type_size = $.sizeof(column);
        local next_offset = current_offset + type_size;
        {
          offset: next_offset,
          fields: acc.fields + new_fields,
        }
      ),
      columns,
      initial
    );
    result.fields
  ),
  column(table, column, offset):: (
    local type = column.type;
    local target_table =
      if column.references != null && std.objectHas(column.references, 'table')
      then column.references.table else null;
    local dat = {
      column_name: column.name,
      snake_column_name: util.case.snake(column.name),
      table_name: table,
      snake_table_name: util.case.snake(table),
      target_table_name: target_table,
      snake_target_table_name: if target_table != null then util.case.snake(target_table) else null,
      type: $.type_map(column.type),
      offset: offset,
      getter: $.getter_map(column.type) % self,
    };
    local matches = [
      x[1]
      for x in types
      if $.column_match(column, x[0])
    ];
    if std.length(matches) == 0
    then error 'unknown column type %s' % [column]
    else if std.length(matches) > 1
    then error 'ambiguous column type, matches: %s\n%s' % [[m.type for m in matches], column]
    else std.mapWithKey(function(k, v) [x % dat for x in v], matches[0])
  ),
}
