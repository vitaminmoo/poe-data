local util = import 'util.libsonnet';

local rename = '#[serde(rename = "%(column_name)s")]';
local not_null = function(x) x != null;
local any = function(x) true;

local types =
  [
    // row
    [{
      type: ['row'],
      array: any,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'row',
      field_type+: ['pub r#%(snake_column_name)s: Option<%(target_table_name)sRef>, // row'],
      field_value+: ['// row_ref %(snake_column_name)s // row_ref'],
    }],
    // row_by_column
    [{
      type: ['row'],
      array: any,
      table_reference: null,
      column_reference: not_null,
    }, {
      type: 'row_by_column',
      field_type+: ['// row_by_column %(snake_column_name)s'],
      field_value+: ['// ro_by_column %(snake_column_name)s'],
    }],
    // foreignrow
    [{
      type: ['foreignrow'],
      array: false,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'foreignrow',
      field_type+: ['pub r#%(snake_column_name)s: Option<%(target_table_name)sRef>, // foreignrow'],
      field_value+: [|||
        r#%(snake_column_name)s: {
          let x = row.get(%(offset)d..%(offset)d + 16).unwrap().get_u64_le() as usize;
          match x {
            0xFEFEFEFEFEFEFEFE => None,
            _ => Some(%(target_table_name)sRef(x)),
          }
        }, // foreignrow
      |||],
    }],
    // foreignrow_array
    [{
      type: ['foreignrow'],
      array: true,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'foreignrow_array',
      field_type+: ['pub r#%(snake_column_name)s: Vec<%(target_table_name)sRef>, // foreignrow_array'],
      field_value+: [|||
        r#%(snake_column_name)s: {
          let x = row.get(%(offset)d..%(offset)d + 16).unwrap().get_u64_le() as usize;
          match x {
            0xFEFEFEFEFEFEFEFE => None,
            _ => Some(%(target_table_name)sRef(x)),
          }
        }, // foreignrow_array
      |||],
    }],
    // foreignrow_by_column
    [{
      type: ['i32', 'string'],
      array: any,
      table_reference: not_null,
      column_reference: not_null,
    }, {
      type: 'foreignrow_by_column',
      field_type+: ['// foreignrow_by_column %(snake_column_name)s'],
      field_value+: ['// foreignrow_by_column %(snake_column_name)s'],
    }],
    // foreignrow_unknown
    [{
      type: ['foreignrow'],
      array: false,
      table_reference: null,
      column_reference: any,
    }, {
      type: 'foreignrow_unknown',
      field_type+: ['pub r#%(snake_column_name)s: Option<u64>, // foreignrow_unknown'],
      field_value+: [|||
        r#%(snake_column_name)s: {
          let x = row.get(%(offset)d..%(offset)d + 16).unwrap().get_u64_le();
          match x {
            0xFEFEFEFEFEFEFEFE => None,
            _ => Some(x),
          }
        }, // foreignrow_unknown
      |||],
    }],
    // foreignrow_unknown_array
    [{
      type: ['foreignrow'],
      array: true,
      table_reference: null,
      column_reference: any,
    }, {
      type: 'foreignrow_unknown_array',
      field_type+: ['pub r#%(snake_column_name)s: Vec<u64>, // foreignrow_unknown_array'],
      field_value+: [|||
        r#%(snake_column_name)s: df.array_from_offset(
          row.get(%(offset)d+8..%(offset)d + 16).unwrap().get_u64_le() as usize,
          row.get(%(offset)d..%(offset)d + 8).unwrap().get_u64_le() as usize,
          16,
        ).unwrap().iter().map(|x| x.clone().get_u64_le()).collect(), // foreignrow_unknown_array'],
      |||],
    }],
    // enumrow
    [{
      type: 'enumrow',
      array: any,
      table_reference: not_null,
      column_reference: null,
    }, {
      type: 'enumrow',
      field_type+: ['pub r#%(snake_column_name)s: MaybeVariant<%(target_table_name)s>, // enumrow'],
      field_value+: [|||
        r#%(snake_column_name)s: {
          let x = row.get(%(offset)d..%(offset)d + 8).unwrap().get_u32_le() as usize;
          %(target_table_name)s::from_repr(x)
               .map_or(MaybeVariant::NotVariant(x), MaybeVariant::Variant)
        }, // enumrow
      |||],
    }],
    // basic
    [{
      type: ['i16', 'u16', 'f32', 'i32', 'u32', 'bool'],
      array: any,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'basic',
      field_type+: ['pub r#%(snake_column_name)s: %(rust_type)s, // basic'],
      field_value+: ['r#%(snake_column_name)s: %(getter)s, // basic'],
    }],
    // string
    [{
      type: ['string'],
      array: any,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'string',
      field_type+: ['pub r#%(snake_column_name)s: %(rust_type)s, // string'],
      field_value+: ['r#%(snake_column_name)s: %(getter)s, // string'],
    }],
    // array_unknown
    [{
      type: ['array'],
      array: true,
      table_reference: null,
      column_reference: null,
    }, {
      type: 'array_unknown',
      field_type+: ['pub r#%(snake_column_name)s: %(rust_type)s, // array_unknown'],
      field_value+: ['r#%(snake_column_name)s: %(getter)s, // array_unknown'],
    }],
  ];

{
  type_map(t):: (
    local tm = {
      // field type: rust type
      bool: 'bool',
      i16: 'i16',
      u16: 'u16',
      i32: 'i32',
      u32: 'u32',
      i64: 'i64',
      u64: 'u64',
      f32: 'f32',
      string: 'String',
      row: '%(target_table_name)sRef',  // references its own table
      foreignrow: '%(target_table_name)sRef',  // references another table
      enumrow: 'enumrowFIXME',
      array: 'i32',
    };
    if std.objectHas(tm, t) then tm[t] else error 'unknown upstream type %s' % t
  ),
  getter_map(t):: (
    local tm = {
      bool: 'row.get(%(offset)d).unwrap().to_le() != 0',
      i16: 'row.get(%(offset)d..%(offset)d + 2).unwrap().get_i16_le()',
      u16: 'row.get(%(offset)d..%(offset)d + 2).unwrap().get_u16_le()',
      i32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_i32_le()',
      u32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_u32_le()',
      i64: 'row.get(%(offset)d..%(offset)d + 8).unwrap().get_i64_le()',
      u64: 'row.get(%(offset)d..%(offset)d + 8).unwrap().get_u64_le()',
      f32: 'row.get(%(offset)d..%(offset)d + 4).unwrap().get_f32_le()',
      String: 'df.string_from_offset(row.get(%(offset)d..%(offset)d + 8).unwrap().get_i32_le() as usize).unwrap()',
      'Vec<%(target_table_name)s>': 'Vec::new()',
      '%(target_table_name)s': '%(target_table_name)s()',
      'Vec<i32>': 'Vec::new()',
      'Vec<String>': 'Vec::new()',
    };
    if std.objectHas(tm, t) then tm[t] else error 'unknown rust type %s' % t
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
  },
  sizeof(column):: (
    if column.array then
      16
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
        local processed_column = $.column(
          table.name,
          column { name: if column.name != null then column.name else 'unknown%d' % current_offset },
          current_offset
        );
        local new_fields =
          if processed_column != null then
            {
              field_types+: if processed_column.field_type != null then processed_column.field_type else [],
              field_values+: if processed_column.field_value != null then processed_column.field_value else [],
            }
          else
            {};

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
      target_table_name: if target_table != null then target_table else 'u64',
      rust_type_template: if column.array then
        'Vec<' + $.type_map(column.type) + '>' else
        $.type_map(column.type),
      rust_type: self.rust_type_template % self,
      offset: offset,
      getter: $.getter_map(self.rust_type_template) % self,
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
