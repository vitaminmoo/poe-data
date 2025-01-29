#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Essences {
    pub r#base_item_types_key: Option<BaseItemTypesRow>,
    pub r#display_wand_mods_key: Option<ModsRow>,
    pub r#display_bow_mods_key: Option<ModsRow>,
    pub r#display_quiver_mods_key: Option<ModsRow>,
    pub r#display_amulet_mods_key: Option<ModsRow>,
    pub r#display_ring_mods_key: Option<ModsRow>,
    pub r#display_belt_mods_key: Option<ModsRow>,
    pub r#display_gloves_mods_key: Option<ModsRow>,
    pub r#display_boots_mods_key: Option<ModsRow>,
    pub r#display_body_armour_mods_key: Option<ModsRow>,
    pub r#display_helmet_mods_key: Option<ModsRow>,
    pub r#display_shield_mods_key: Option<ModsRow>,
    pub r#hash32: i32,
    pub r#drop_level: Vec<i32>,
    pub r#monster_mods_keys: Vec<ModsRow>,
    pub r#essence_type_key: Option<EssenceTypeRow>,
    pub r#level: i32,
    pub r#display_weapon_mods_key: Option<ModsRow>,
    pub r#display_melee_weapon_mods_key: Option<ModsRow>,
    pub r#display_one_hand_weapon_mods_key: Option<ModsRow>,
    pub r#display_two_hand_weapon_mods_key: Option<ModsRow>,
    pub r#display_two_hand_melee_weapon_mods_key: Option<ModsRow>,
    pub r#display_armour_mods_key: Option<ModsRow>,
    pub r#display_ranged_weapon_mods_key: Option<ModsRow>,
    pub r#helmet_mods_key: Option<ModsRow>,
    pub r#body_armour_mods_key: Option<ModsRow>,
    pub r#boots_mods_key: Option<ModsRow>,
    pub r#gloves_mods_key: Option<ModsRow>,
    pub r#bow_mods_key: Option<ModsRow>,
    pub r#wand_mods_key: Option<ModsRow>,
    pub r#staff_mods_key: Option<ModsRow>,
    pub r#two_hand_sword_mods_key: Option<ModsRow>,
    pub r#two_hand_axe_mods_key: Option<ModsRow>,
    pub r#two_hand_mace_mods_key: Option<ModsRow>,
    pub r#claw_mods_key: Option<ModsRow>,
    pub r#dagger_mods_key: Option<ModsRow>,
    pub r#one_hand_sword_mods_key: Option<ModsRow>,
    pub r#one_hand_thrusting_sword_mods_key: Option<ModsRow>,
    pub r#one_hand_axe_mods_key: Option<ModsRow>,
    pub r#one_hand_mace_mods_key: Option<ModsRow>,
    pub r#sceptre_mods_key: Option<ModsRow>,
    pub r#display_monster_mods_key: Option<ModsRow>,
    pub r#item_level_restriction: i32,
    pub r#belt_mods_key: Option<ModsRow>,
    pub r#amulet_mods_key: Option<ModsRow>,
    pub r#ring_mods_key: Option<ModsRow>,
    pub r#display_jewellery_mods_key: Option<ModsRow>,
    pub r#shield_mods_key: Option<ModsRow>,
    pub r#display_items_mods_key: Option<ModsRow>,
    pub r#is_screaming_essence: bool,
    pub r#memory_lines: Vec<ModsRow>,
    pub r#level2: Vec<i32>,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Essences: LazyLock<Vec<Essences>> = LazyLock::new(|| {
    RAW_TABLE_Essences
        .iter()
        .map(|x| Essences {
            r#base_item_types_key: x.r#base_item_types_key.map(BaseItemTypesRow),
            r#display_wand_mods_key: x.r#display_wand_mods_key.map(ModsRow),
            r#display_bow_mods_key: x.r#display_bow_mods_key.map(ModsRow),
            r#display_quiver_mods_key: x.r#display_quiver_mods_key.map(ModsRow),
            r#display_amulet_mods_key: x.r#display_amulet_mods_key.map(ModsRow),
            r#display_ring_mods_key: x.r#display_ring_mods_key.map(ModsRow),
            r#display_belt_mods_key: x.r#display_belt_mods_key.map(ModsRow),
            r#display_gloves_mods_key: x.r#display_gloves_mods_key.map(ModsRow),
            r#display_boots_mods_key: x.r#display_boots_mods_key.map(ModsRow),
            r#display_body_armour_mods_key: x.r#display_body_armour_mods_key.map(ModsRow),
            r#display_helmet_mods_key: x.r#display_helmet_mods_key.map(ModsRow),
            r#display_shield_mods_key: x.r#display_shield_mods_key.map(ModsRow),
            r#hash32: x.r#hash32.clone(),
            r#drop_level: x.r#drop_level.clone(),
            r#monster_mods_keys: x.r#monster_mods_keys.iter().copied().map(ModsRow).collect(),
            r#essence_type_key: x.r#essence_type_key.map(EssenceTypeRow),
            r#level: x.r#level.clone(),
            r#display_weapon_mods_key: x.r#display_weapon_mods_key.map(ModsRow),
            r#display_melee_weapon_mods_key: x.r#display_melee_weapon_mods_key.map(ModsRow),
            r#display_one_hand_weapon_mods_key: x.r#display_one_hand_weapon_mods_key.map(ModsRow),
            r#display_two_hand_weapon_mods_key: x.r#display_two_hand_weapon_mods_key.map(ModsRow),
            r#display_two_hand_melee_weapon_mods_key: x
                .r#display_two_hand_melee_weapon_mods_key
                .map(ModsRow),
            r#display_armour_mods_key: x.r#display_armour_mods_key.map(ModsRow),
            r#display_ranged_weapon_mods_key: x.r#display_ranged_weapon_mods_key.map(ModsRow),
            r#helmet_mods_key: x.r#helmet_mods_key.map(ModsRow),
            r#body_armour_mods_key: x.r#body_armour_mods_key.map(ModsRow),
            r#boots_mods_key: x.r#boots_mods_key.map(ModsRow),
            r#gloves_mods_key: x.r#gloves_mods_key.map(ModsRow),
            r#bow_mods_key: x.r#bow_mods_key.map(ModsRow),
            r#wand_mods_key: x.r#wand_mods_key.map(ModsRow),
            r#staff_mods_key: x.r#staff_mods_key.map(ModsRow),
            r#two_hand_sword_mods_key: x.r#two_hand_sword_mods_key.map(ModsRow),
            r#two_hand_axe_mods_key: x.r#two_hand_axe_mods_key.map(ModsRow),
            r#two_hand_mace_mods_key: x.r#two_hand_mace_mods_key.map(ModsRow),
            r#claw_mods_key: x.r#claw_mods_key.map(ModsRow),
            r#dagger_mods_key: x.r#dagger_mods_key.map(ModsRow),
            r#one_hand_sword_mods_key: x.r#one_hand_sword_mods_key.map(ModsRow),
            r#one_hand_thrusting_sword_mods_key: x.r#one_hand_thrusting_sword_mods_key.map(ModsRow),
            r#one_hand_axe_mods_key: x.r#one_hand_axe_mods_key.map(ModsRow),
            r#one_hand_mace_mods_key: x.r#one_hand_mace_mods_key.map(ModsRow),
            r#sceptre_mods_key: x.r#sceptre_mods_key.map(ModsRow),
            r#display_monster_mods_key: x.r#display_monster_mods_key.map(ModsRow),
            r#item_level_restriction: x.r#item_level_restriction.clone(),
            r#belt_mods_key: x.r#belt_mods_key.map(ModsRow),
            r#amulet_mods_key: x.r#amulet_mods_key.map(ModsRow),
            r#ring_mods_key: x.r#ring_mods_key.map(ModsRow),
            r#display_jewellery_mods_key: x.r#display_jewellery_mods_key.map(ModsRow),
            r#shield_mods_key: x.r#shield_mods_key.map(ModsRow),
            r#display_items_mods_key: x.r#display_items_mods_key.map(ModsRow),
            r#is_screaming_essence: x.r#is_screaming_essence.clone(),
            r#memory_lines: x.r#memory_lines.iter().copied().map(ModsRow).collect(),
            r#level2: x.r#level2.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct EssencesRow(pub usize);

impl Deref for EssencesRow {
    type Target = Essences;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Essences[self.0]
    }
}

impl EssencesRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Essences {
        &TABLE_Essences[self.0]
    }
    pub fn get(&self) -> &'static Essences {
        &TABLE_Essences[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Essences.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Essences)> {
        TABLE_Essences.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Essences: LazyLock<Vec<EssencesRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Essences.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct EssencesRaw {
    #[serde(rename = "BaseItemTypesKey")]
    pub r#base_item_types_key: Option<usize>,
    #[serde(rename = "Display_Wand_ModsKey")]
    pub r#display_wand_mods_key: Option<usize>,
    #[serde(rename = "Display_Bow_ModsKey")]
    pub r#display_bow_mods_key: Option<usize>,
    #[serde(rename = "Display_Quiver_ModsKey")]
    pub r#display_quiver_mods_key: Option<usize>,
    #[serde(rename = "Display_Amulet_ModsKey")]
    pub r#display_amulet_mods_key: Option<usize>,
    #[serde(rename = "Display_Ring_ModsKey")]
    pub r#display_ring_mods_key: Option<usize>,
    #[serde(rename = "Display_Belt_ModsKey")]
    pub r#display_belt_mods_key: Option<usize>,
    #[serde(rename = "Display_Gloves_ModsKey")]
    pub r#display_gloves_mods_key: Option<usize>,
    #[serde(rename = "Display_Boots_ModsKey")]
    pub r#display_boots_mods_key: Option<usize>,
    #[serde(rename = "Display_BodyArmour_ModsKey")]
    pub r#display_body_armour_mods_key: Option<usize>,
    #[serde(rename = "Display_Helmet_ModsKey")]
    pub r#display_helmet_mods_key: Option<usize>,
    #[serde(rename = "Display_Shield_ModsKey")]
    pub r#display_shield_mods_key: Option<usize>,
    #[serde(rename = "HASH32")]
    pub r#hash32: i32,
    #[serde(rename = "DropLevel")]
    pub r#drop_level: Vec<i32>,
    #[serde(rename = "Monster_ModsKeys")]
    pub r#monster_mods_keys: Vec<usize>,
    #[serde(rename = "EssenceTypeKey")]
    pub r#essence_type_key: Option<usize>,
    #[serde(rename = "Level")]
    pub r#level: i32,
    #[serde(rename = "Display_Weapon_ModsKey")]
    pub r#display_weapon_mods_key: Option<usize>,
    #[serde(rename = "Display_MeleeWeapon_ModsKey")]
    pub r#display_melee_weapon_mods_key: Option<usize>,
    #[serde(rename = "Display_OneHandWeapon_ModsKey")]
    pub r#display_one_hand_weapon_mods_key: Option<usize>,
    #[serde(rename = "Display_TwoHandWeapon_ModsKey")]
    pub r#display_two_hand_weapon_mods_key: Option<usize>,
    #[serde(rename = "Display_TwoHandMeleeWeapon_ModsKey")]
    pub r#display_two_hand_melee_weapon_mods_key: Option<usize>,
    #[serde(rename = "Display_Armour_ModsKey")]
    pub r#display_armour_mods_key: Option<usize>,
    #[serde(rename = "Display_RangedWeapon_ModsKey")]
    pub r#display_ranged_weapon_mods_key: Option<usize>,
    #[serde(rename = "Helmet_ModsKey")]
    pub r#helmet_mods_key: Option<usize>,
    #[serde(rename = "BodyArmour_ModsKey")]
    pub r#body_armour_mods_key: Option<usize>,
    #[serde(rename = "Boots_ModsKey")]
    pub r#boots_mods_key: Option<usize>,
    #[serde(rename = "Gloves_ModsKey")]
    pub r#gloves_mods_key: Option<usize>,
    #[serde(rename = "Bow_ModsKey")]
    pub r#bow_mods_key: Option<usize>,
    #[serde(rename = "Wand_ModsKey")]
    pub r#wand_mods_key: Option<usize>,
    #[serde(rename = "Staff_ModsKey")]
    pub r#staff_mods_key: Option<usize>,
    #[serde(rename = "TwoHandSword_ModsKey")]
    pub r#two_hand_sword_mods_key: Option<usize>,
    #[serde(rename = "TwoHandAxe_ModsKey")]
    pub r#two_hand_axe_mods_key: Option<usize>,
    #[serde(rename = "TwoHandMace_ModsKey")]
    pub r#two_hand_mace_mods_key: Option<usize>,
    #[serde(rename = "Claw_ModsKey")]
    pub r#claw_mods_key: Option<usize>,
    #[serde(rename = "Dagger_ModsKey")]
    pub r#dagger_mods_key: Option<usize>,
    #[serde(rename = "OneHandSword_ModsKey")]
    pub r#one_hand_sword_mods_key: Option<usize>,
    #[serde(rename = "OneHandThrustingSword_ModsKey")]
    pub r#one_hand_thrusting_sword_mods_key: Option<usize>,
    #[serde(rename = "OneHandAxe_ModsKey")]
    pub r#one_hand_axe_mods_key: Option<usize>,
    #[serde(rename = "OneHandMace_ModsKey")]
    pub r#one_hand_mace_mods_key: Option<usize>,
    #[serde(rename = "Sceptre_ModsKey")]
    pub r#sceptre_mods_key: Option<usize>,
    #[serde(rename = "Display_Monster_ModsKey")]
    pub r#display_monster_mods_key: Option<usize>,
    #[serde(rename = "ItemLevelRestriction")]
    pub r#item_level_restriction: i32,
    #[serde(rename = "Belt_ModsKey")]
    pub r#belt_mods_key: Option<usize>,
    #[serde(rename = "Amulet_ModsKey")]
    pub r#amulet_mods_key: Option<usize>,
    #[serde(rename = "Ring_ModsKey")]
    pub r#ring_mods_key: Option<usize>,
    #[serde(rename = "Display_Jewellery_ModsKey")]
    pub r#display_jewellery_mods_key: Option<usize>,
    #[serde(rename = "Shield_ModsKey")]
    pub r#shield_mods_key: Option<usize>,
    #[serde(rename = "Display_Items_ModsKey")]
    pub r#display_items_mods_key: Option<usize>,
    #[serde(rename = "IsScreamingEssence")]
    pub r#is_screaming_essence: bool,
    #[serde(rename = "MemoryLines")]
    pub r#memory_lines: Vec<usize>,
    #[serde(rename = "Level2")]
    pub r#level2: Vec<i32>,
}
