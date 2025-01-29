#![allow(clippy::all)]
use serde::{Deserialize, Serialize};
use strum::{Display, EnumIter, FromRepr};
#[derive(Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy)]
pub enum MaybeVariant<T> {
    Variant(T),
    NotVariant(usize),
}

impl<T> MaybeVariant<T> {
    pub fn unwrap(self) -> T {
        match self {
            MaybeVariant::Variant(x) => x,
            MaybeVariant::NotVariant(x) => panic!("Not an enum variant {}", x),
        }
    }
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AbyssRegions {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AbyssTheme {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AccountQuestFlags {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AchievementSets {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ActiveSkillTargetTypes {
    TargetableGround = 1,
    Enemy = 2,
    WalkableGround = 3,
    AnywhereSelfTarget = 4,
    Item = 5,
    Corpse = 6,
    NoLineOfSight = 8,
    BehindMonster = 9,
    SelfOrigin = 10,
    RotateToTarget = 11,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AdditionalLifeScalingPerLevel {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AdditionalMonsterPacksStatMode {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AfflictionRewardTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AlternateBehaviourTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AlternateTreePassiveSizes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AncestralTrialMonsterRanks {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AncestralTrialOpinionTypes {
    Neutral = 0,
    Likes = 1,
    Dislikes = 2,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AncestralTrialShopSlots {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AncestralTrialUnitTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AreaType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ArmourClasses {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ArmourSurfaceTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AtlasEntities {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AtlasInfluenceOutcomeTypes {
    MonsterPacks = 0,
    EmpoweredBoss = 1,
    OnKill = 2,
    InitialPacks = 3,
    Awakener = 4,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AtlasModTiers {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Attributes {
    Strength = 1,
    Dexterity = 2,
    Intelligence = 3,
    None = 4,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AudioCharacterClass {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum AzmeriEncounterThemes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BattlePassRewardTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BetrayalDialogueCue {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BetrayalFlags {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BetrayalTargetFlags {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BetrayalUpgradeSlots {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BuffCategories {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BuffGroups {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BuffMergeModes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BuffStackUIModes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum BuffVisualSets {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CharacterPanelStatContexts {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ClientUIScreens {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CooldownBypassTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CooldownGroups {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CraftingBenchCustomActions {
    RemoveCraftedMods = 0,
    RemoveEnchantMods = 1,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CurrencyUseTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum CustomLeagueTemplate {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DamageHitTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DamageParticleEffectTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Default {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DelveUpgradeType {
    SulphiteCapacity = 0,
    FlareCapacity = 1,
    DynamiteCapacity = 2,
    CartLightRadius = 3,
    FlareLightRadius = 4,
    DynamiteRadius = 5,
    DynamiteDamage = 7,
    DarknessResistance = 8,
    FlareDuration = 9,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Directions {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DropReplacementCustomReplacements {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DropReplacementCustomTargets {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum DuplicateGrantedEffect {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Effectiveness {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum EvergreenAchievementTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ExpeditionDealFamilies {
    RerollPrefixesBest = 1,
    RerollSuffixesBest = 2,
    ValuesAffixesLucky = 3,
    ValuesPrefixesLucky = 4,
    ValuesSuffixesLucky = 5,
    ValuesImplicitsLucky = 6,
    UpgradeHighestTier = 7,
    UpgradeOneTier = 8,
    RemoveLowestLevel = 9,
    AddPrefixBest = 10,
    AddSuffixBest = 11,
    FillEmptyMods = 12,
    ReforgeSockets = 13,
    ReforgeLinks = 14,
    ApplyIncubator = 15,
    ApplyRandomQuality = 16,
    SetQuality = 17,
    ApplyAnointment = 18,
    Corrupt = 19,
    AddInfluence = 20,
    RemoveAllPrefixes = 21,
    RemoveAllSuffixes = 22,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ExpeditionRelicModCategories {
    Generic = 0,
    Runic = 1,
    RewardChest = 2,
    RewardPackSize = 3,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum FlaskType {
    Life = 1,
    Mana = 2,
    Hybrid = 3,
    Utility = 4,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum FlavourTextImages {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum GemItemVisualEffect {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum GemTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum GrantedEffectGroups {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum GroundEffectEffectTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum HarvestColours {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum HeistBlueprintWindowTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum HeistChestTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum HeistFormationMarkerType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum HeistRoomTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum InfluenceTypes {
    Shaper = 0,
    Elder = 1,
    Crusader = 2,
    Eyrie = 3,
    Basilisk = 4,
    Adjudicator = 5,
    None = 6,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum InvasionMonsterGroups {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum InvasionMonsterRoles {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum InventoryType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ItemClassFlags {
    Weapon = 0,
    OneHand = 1,
    Melee = 2,
    OffHand = 3,
    Flask = 4,
    Armour = 5,
    Jewellery = 6,
    Currency = 7,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ItemCreationTemplateCustomAction {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ItemSetNames {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LabyrinthCraftOptionFamily {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LabyrinthSecretLocations {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LeagueCategory {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LeagueQuestFlags {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LegionMonsterTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LegionRankTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum LegionRewardTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MapDeviceCustomEffects {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MapDevicePortalFormation {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MapFragmentFamilies {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionCategoryId {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionConditionalApparitionEventType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionConditionalApparitionOrientation {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionConditionalApparitionPosition {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionRecycleCategories {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionSlotId {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MicrotransactionTrackedStatistics {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MissionTileMap {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ModAuraFlags {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ModDomains {
    Item = 1,
    Flask = 2,
    Monster = 3,
    Chest = 4,
    Area = 5,
    SanctumRelic = 7,
    Crafted = 9,
    BaseJewel = 10,
    Atlas = 11,
    Leaguestone = 12,
    AbyssJewel = 13,
    MapDevice = 14,
    Dummy = 15,
    Delve = 16,
    DelveArea = 17,
    SynthesisA = 18,
    SynthesisGlobals = 19,
    SynthesisBonus = 20,
    AfflictionJewel = 21,
    HeistArea = 22,
    HeistNpc = 23,
    HeistTrinket = 24,
    Watchstone = 25,
    Veiled = 26,
    ExpeditionRelic = 27,
    Unveiled = 28,
    EldritchAltar = 29,
    Sentinel = 30,
    MemoryLine = 31,
    SanctumSpecial = 32,
    CrucibleMap = 33,
    Tincture = 34,
    AnimalCharm = 35,
    NecropolisMonster = 36,
    UberMap = 37,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ModGenerationType {
    Prefix = 1,
    Suffix = 2,
    Unique = 3,
    Nemesis = 4,
    Corrupted = 5,
    Bloodlines = 6,
    Torment = 7,
    Tempest = 8,
    Talisman = 9,
    Enchantment = 10,
    Essence = 11,
    Bestiary = 13,
    DelveArea = 14,
    SynthesisA = 15,
    SynthesisGlobals = 16,
    SynthesisBonus = 17,
    Blight = 18,
    BlightTower = 19,
    MonsterAffliction = 20,
    FlaskEnchantmentEnkindling = 21,
    FlaskEnchantmentInstilling = 22,
    ExpeditionLogbook = 23,
    ScourgeUpside = 24,
    ScourgeDownside = 25,
    ScourgeMap = 26,
    ExarchImplicit = 28,
    EaterImplicit = 29,
    WeaponTree = 31,
    WeaponTreeRecombined = 32,
    NecropolisHaunted = 34,
    NecropolisDevoted = 35,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ModSetNames {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterBehavior {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterFleeConditions {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterGroupNames {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterPushTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterScalingByLevel {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSize {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsAliveDead {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsAttackSpell {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsClientInstance {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsHull {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsOrientation {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsPlacement {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsReference {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsSequenceMode {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsShape {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsTargets {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum MonsterSkillsWaveDirection {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum NPCShopSellPriceType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum NPCTalkQuickActions {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum NPCTextAudioInterruptRules {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum OnHitEffectTarget {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Orientations {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PantheonMapBossSelection {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PassiveSkillTattooTargets {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PassiveSkillTypes {
    PassiveTree = 0,
    AtlasTree = 1,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PassiveTattooVendorGroups {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PerLevelValues {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum PreloadPriorities {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum ProjectileCollisionTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum QuestStateCalculation {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum RarityMask {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum RelativeImportanceConstants {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SanctumEffectTriggers {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SanctumImmediateEffectType {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SanctumPersistentEffectFamily {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SkillMines {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SkillMorphDisplayOverlayCondition {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SkillMorphDisplayOverlayStyle {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SkillTotems {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum StashId {
    Normal = 0,
    Premium = 1,
    Trade = 2,
    Currency = 3,
    Unique = 4,
    Map = 5,
    DivinationCard = 6,
    Quad = 7,
    Essence = 8,
    Fragment = 9,
    PcBangPremium = 10,
    PcBangEssence = 11,
    Delve = 12,
    Blight = 13,
    Metamorph = 14,
    Delirium = 15,
    Folder = 16,
    Flask = 17,
    Gem = 18,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum StatInterpolationTypes {
    Constant = 1,
    Linear = 2,
    Exponential = 3,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum StatSemantics {
    Percent = 1,
    OverrideValue = 2,
    Value = 3,
    Flag = 4,
    OverridePercent = 5,
    Permyriad = 6,
    OverridePermyriad = 7,
}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum SurgeCategory {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum TradeMarketCategoryStyleFlag {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum UITalkCategories {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum UniqueSetNames {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum UserInterfaceModeCondition {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum VisualItemSlotId {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum WeaponArmourCommon {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum WeaponDamageScaling {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum WeaponSoundTypes {}

#[derive(
    Debug, Deserialize, Serialize, PartialEq, Eq, Clone, Copy, FromRepr, EnumIter, Display,
)]
pub enum Wordlists {
    ItemPrefix = 1,
    ItemSuffix = 2,
    MonsterPrefix = 3,
    MonsterSuffix = 4,
    MonsterTitle = 5,
    UniqueItem = 6,
    StrongboxPrefix = 7,
    StrongboxSuffix = 8,
    Essence = 9,
}
