local emptyfiles = import 'emptyfiles.libsonnet';
local schema = import 'schema.min.json';
local types = import 'types.libsonnet';
local util = import 'util.libsonnet';
local validfiles = import 'validfiles.libsonnet';

local config = {
  poeVersion:: 2,
  included:: ['Acts', 'NPCTalkDialogueTextAudio', 'MtxTypeGameSpecific'],
  excluded:: {
    AncestralTrialUnits: true,
    CraftingBenchSortCategories: true,
    CraftingBenchOptions: true,
    SanctumSelectionDisplayOverride: true,
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

local allTables = std.objectFields(tablesKV);

config {
  //tables: [tablesKV[table] for table in util.depsolver.allTables(tablesKV, super.included, super.excluded)],
  tables: [tablesKV[table] for table in allTables if !std.objectHas(super.excluded, table) && std.member(validfiles, std.asciiLower(table)) && !std.member(emptyfiles, std.asciiLower(table))],
  invalidTables: [table for table in allTables if !std.member(validfiles, std.asciiLower(table))],
  enums:: [e for e in std.objectValues(enumsKV)],
}
