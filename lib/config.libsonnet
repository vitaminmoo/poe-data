local schema = import 'schema.min.json';

local explicitTables = ['Acts', 'Shrines'];
// good simple case for testing
//local explicitTables = ['DamageParticleEffects'];
{
  tableBlacklist:: {
    //ActiveSkills: true,
  },
  kvSchema:: {
    [table.name]: table
    for table in schema.tables
    if table.validFor == 2 || table.validFor == 3
  },
  tableRefs(table)::
    if std.objectHas($.kvSchema, table)
    then std.set([
      $.columnRefs(column)
      for column in $.kvSchema[table].columns
      if column.name != null && column.type != 'enumrow' && $.columnRefs(column) != null
    ])
    else [],
  columnRefs(column)::
    if (column.references != null && std.length(column.references) > 0)
    then column.references.table
    else null,
  getTables5(baseTables, collectedTables)::
    if baseTables == []
    then []
    else
      std.setUnion(
        baseTables,
        $.getTables5(
          std.set(std.flattenArrays([
            if std.member(collectedTables, table2)
            then []  // error 'loops: %s -> %s' % [table, table2]
            else [table2]
            for table in baseTables
            for table2 in $.tableRefs(table)
            if !std.objectHas($.tableBlacklist, table2)
          ]))
          , std.setUnion(baseTables, collectedTables)
        )
      ),
  allTables:: $.getTables5(explicitTables, []),
  tables: [
    {
      name: table,
      columns: [
        column.name
        for column in $.kvSchema[table].columns
        if column.name != null
      ],
    }
    for table in $.allTables
  ],
}
