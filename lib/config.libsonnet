local emptyfiles = import 'emptyfiles.libsonnet';
local schema = import 'schema.min.json';
local types = import 'types.libsonnet';
local util = import 'util.libsonnet';
local validfiles = import 'validfiles.libsonnet';

local config = {
  poeVersion:: 3,
  included:: ['Acts', 'NPCTalkDialogueTextAudio', 'MtxTypeGameSpecific'],
  excluded:: {
    AncestralTrialUnits: true,
    AwardDisplay: true,
    BeyondFactions: true,
    CraftingBenchOptions: true,
    CraftingBenchSortCategories: true,
    GroundEffects: true,
    HarvestPerLevelValues: true,
    HudVisualsFromStat: true,
    ItemNoteCode: true,
    ItemVisualReplacement: true,
    LegionBalancePerLevel: true,
    LegionRanks: true,
    NPCMaster: true,
    PassiveJewelRadii: true,
    PassiveOverrideLimits: true,
    RelicItemEffectVariations: true,
    SanctumSelectionDisplayOverride: true,
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
  invalidTables: [
    table
    for table in allTables
    if !std.contains(validfiles, std.asciiLower(table)) ||
    std.contains(emptyfiles, std.asciiLower(table)) ||
    std.objectHas(super.excluded, table)
  ],
  tables: [
    tablesKV[table]
    for table in allTables
    if !std.objectHas(super.excluded, table) &&
    !std.contains(self.invalidTables, table)
  ],
  enums:: [e for e in std.objectValues(enumsKV)],
  datPath:: if config.poeVersion == 2 then 'data/balance' else 'data/',
}
