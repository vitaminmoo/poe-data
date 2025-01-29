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
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: Option<usize>,'],
      field_type+: ['pub r#%(snake_column_name)s: Option<%(target_table_name)sRow>,'],
      field_value+: ['r#%(snake_column_name)s: x.r#%(snake_column_name)s.map(%(target_table_name)sRow),'],
    }],
    // row_ref_array
    [{
      type: ['row', 'foreignrow'],
      array: true,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'row_ref_array',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: Vec<usize>,'],
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(target_table_name)sRow>,'],
      field_value+: ['r#%(snake_column_name)s: x.r#%(snake_column_name)s.iter().copied().map(%(target_table_name)sRow).collect(),'],
    }],
    // column_ref
    [{
      type: ['i32', 'u32', 'string', 'f32'],
      array: false,
      table_reference: not_null,
      column_reference: not_null,
    }, {
      type: 'column_ref',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: %(type)s,'],
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
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: Vec<%(type)s>,'],
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
      raw_field_type+: [],
      field_type+: [],
      field_value+: [],
    }],
    // enum_ref
    [{
      type: 'enumrow',
      array: false,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'enum_ref',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: usize,'],
      field_type+: ['pub r#%(snake_column_name)s: MaybeVariant<%(target_table_name)s>,'],
      field_value+: ['r#%(snake_column_name)s: %(target_table_name)s::from_repr(x.r#%(snake_column_name)s).map_or(MaybeVariant::NotVariant(x.r#%(snake_column_name)s), MaybeVariant::Variant),'],
    }],
    // enum_ref_array
    [{
      type: 'enumrow',
      array: true,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'enum_ref_array',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: Vec<usize>,'],
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(target_table_name)s>,'],
      field_value+: ['r#%(snake_column_name)s: x.r#%(snake_column_name)s.iter().map(|y| %(target_table_name)s::from_repr(*y).unwrap()).collect(),'],
    }],
    // basic
    [{
      type: ['i16', 'f32', 'i32', 'u32', 'bool', 'string'],
      array: false,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'basic',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: %(type)s,'],
      field_type+: ['pub r#%(snake_column_name)s: %(type)s,'],
      field_value+: ['r#%(snake_column_name)s: x.r#%(snake_column_name)s.clone(),'],
    }],
    // basic_array
    [{
      type: ['i16', 'f32', 'i32', 'u32', 'bool', 'string'],
      array: true,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'basic_array',
      raw_field_type+: [rename, 'pub r#%(snake_column_name)s: Vec<%(type)s>,'],
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(type)s>,'],
      field_value+: ['r#%(snake_column_name)s: x.r#%(snake_column_name)s.clone(),'],
    }],
  ];

{
  type_map(t):: (
    local tm = {
      i16: 'i16',
      i32: 'i32',
      u32: 'u32',
      string: 'String',
      bool: 'bool',
      f32: 'f32',
    };
    if std.objectHas(tm, t) then tm[t] else error 'unknown type %s' % t
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
    local parts = [
      $.column(table.name, column)
      for column in table.columns
      if column.name != null
    ];
    std.foldl(
      function(x, y)
        x + if y != null then
          {
            raw_field_types+: if y.raw_field_type != null then y.raw_field_type else [],
            field_types+: if y.field_type != null then y.field_type else [],
            field_values+: if y.field_value != null then y.field_value else [],
          }
        else
          {},
      parts,
      {}
    )
  ),
  column(table, column):: (
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
