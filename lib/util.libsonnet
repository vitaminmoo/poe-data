local isCaps(chr) = std.codepoint(chr) >= 65 && std.codepoint(chr) <= 90;
local isNum(chr) = std.codepoint(chr) >= 48 && std.codepoint(chr) <= 57;

local trace(obj) = std.trace('trace: %s' % [obj], obj);

{
  depsolver: {
    tableRefs(kvSchema, table)::
      if std.objectHas(kvSchema, table)
      then std.set([
        $.depsolver.columnRefs(column)
        for column in kvSchema[table].columns
        if column.type != 'enumrow' && $.depsolver.columnRefs(column) != null
      ])
      else [],
    columnRefs(column)::
      if (column.references != null && std.length(column.references) > 0)
      then column.references.table
      else null,
    getTables(kvSchema, baseTables, excluded, collectedTables)::
      if baseTables == []
      then []
      else
        std.setUnion(
          baseTables,
          $.depsolver.getTables(
            kvSchema,
            std.set(
              std.flattenArrays([
                if std.member(collectedTables, table2)
                then []  // error 'loops: %s -> %s' % [table, table2]
                else [table2]
                for table in baseTables
                for table2 in $.depsolver.tableRefs(kvSchema, table)
                if !std.objectHas(excluded, table2)
              ])
            ),
            excluded,
            std.setUnion(baseTables, collectedTables)
          )
        ),
    allTables(kvSchema, included, excluded):: $.depsolver.getTables(kvSchema, included, excluded, []),
  },
  case: {
    snake(str):
      std.lstripChars(
        std.asciiLower(
          std.join(
            '', std.mapWithIndex(
              function(i, chr)
                local chrs = std.stringChars(str);
                if i == 0 || chrs[i - 1] == '_'  // first char, chars after _ = just lowercase
                then chr
                else if isCaps(chr)
                        && !isCaps(chrs[i - 1])
                then '_' + chr
                else if isCaps(chr)
                        && std.length(str) > i + 1
                        && !isCaps(chrs[i + 1])  // don't add _ inside acronyms
                        && !isNum(chrs[i + 1])  // don't add _ between words and numbers
                        && chrs[i + 1] != '_'  // avoid UI_Image -> u_i_image
                then '_' + chr
                else chr
              , std.stringChars(str)
            )
          )
        ), '_'
      ),
    ucfirst(s):
      std.asciiUpper(
        std.substr(s, 0, 1)
      ) + std.join('', std.stringChars(s)[1:]),
    pascal(x): std.join(
      '',
      std.map(
        $.case.ucfirst,
        std.split(std.asciiLower(x), '_')
      )
    ),
  },
  enumeratorToVariant(indexing, idx, enumerator):
    '    %s = %s,' % [$.case.pascal(enumerator), indexing + idx],
  namedListToObject(poeVersion, list):: {
    [x.name]: x
    for x in list
  },
}
