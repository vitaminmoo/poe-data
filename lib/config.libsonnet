local schema = import 'schema.min.json';
local types = import 'types.libsonnet';
local util = import 'util.libsonnet';

local config = {
  poeVersion:: 2,
  included:: ['Acts', 'NPCTalkDialogueTextAudio', 'MtxTypeGameSpecific'],
  excluded:: {
    //ActiveSkills: true,
  },
};

local tables = [table {
  name_snake: util.case.snake(table.name),
  name_lower: std.asciiLower(table.name),
  columns: types.mutate_columns(table.columns),
} for table in schema.tables];

local tablesKV = util.namedListToObject(config.poeVersion, tables);
local enumsKV = util.namedListToObject(config.poeVersion, schema.enumerations);

config {
  tables: [tablesKV[table] for table in util.depsolver.allTables(tablesKV, $.included, $.excluded)],
  enums:: [e for e in std.objectValues(enumsKV)],
}
