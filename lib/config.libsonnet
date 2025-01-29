local schema = import 'schema.min.json';

local explicitTables = ['BaseItemTypes', 'Essences', 'CraftingBenchOptions'];
// good simple case for testing
//local explicitTables = ['DamageParticleEffects'];
{
  tableBlacklist:: {
    //ActiveSkills: true,
  },
  kvSchema:: {
    [table.name]: table
    for table in schema.tables
    if table.validFor == 1 || table.validFor == 3
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
  patch: '3.25.3.4',
  translations: ['English'],
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
  files: [
    'Metadata/StatDescriptions/Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Active_Skill_Gem_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Advanced_Mod_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Atlas_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Aura_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Banner_Aura_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Beam_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Brand_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Buff_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Chest_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Curse_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Debuff_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Expedition_Relic_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Gem_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Heist_Equipment_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Leaguestone_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Map_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Minion_Attack_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Minion_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Minion_Spell_Damage_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Minion_Spell_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Monster_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Necropolis_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Offering_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Passive_Skill_Aura_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Passive_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Primordial_Altar_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Sanctum_Relic_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Secondary_Debuff_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Sentinel_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Single_Minion_Spell_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Skillpopup_Stat_Filters.txt',
    'Metadata/StatDescriptions/Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Tincture_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Vaal_Side_Area_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Variable_Duration_Skill_Stat_Descriptions.txt',
    'Metadata/StatDescriptions/Village_Stat_Descriptions.txt',
  ],
}
