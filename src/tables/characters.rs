#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct Characters {
    pub r#id: String,
    pub r#name: String,
    pub r#ao_file: String,
    pub r#act_file: String,
    pub r#base_max_life: i32,
    pub r#base_max_mana: i32,
    pub r#weapon_speed: i32,
    pub r#min_damage: i32,
    pub r#max_damage: i32,
    pub r#max_attack_distance: i32,
    pub r#icon: String,
    pub r#integer_id: i32,
    pub r#base_strength: i32,
    pub r#base_dexterity: i32,
    pub r#base_intelligence: i32,
    pub r#description: String,
    pub r#start_skill_gem: Option<SkillGemsRow>,
    pub r#character_size: i32,
    pub r#intro_sound_file: String,
    pub r#start_weapons: Vec<BaseItemTypesRow>,
    pub r#gender: String,
    pub r#trait_description: String,
    pub r#passive_tree_image: String,
    pub r#tencent_video: String,
    pub r#attrs_as_id: String,
    pub r#login_screen: String,
    pub r#player_critter: String,
    pub r#player_effect: String,
    pub r#after_image: String,
    pub r#mirage: Option<MonsterVarietiesRow>,
    pub r#clone_immobile: Option<MonsterVarietiesRow>,
    pub r#replicate_clone: Option<MonsterVarietiesRow>,
    pub r#lightning_clone: Option<MonsterVarietiesRow>,
    pub r#skill_tree_background: String,
    pub r#clone: Option<MonsterVarietiesRow>,
    pub r#double: Option<MonsterVarietiesRow>,
    pub r#mirage_warrior: Option<MonsterVarietiesRow>,
    pub r#double_two: Option<MonsterVarietiesRow>,
    pub r#dark_exile: Option<MonsterVarietiesRow>,
    pub r#attr: String,
    pub r#attr_lowercase: String,
    pub r#script: String,
}

#[allow(non_upper_case_globals)]
pub static TABLE_Characters: LazyLock<Vec<Characters>> = LazyLock::new(|| {
    RAW_TABLE_Characters
        .iter()
        .map(|x| Characters {
            r#id: x.r#id.clone(),
            r#name: x.r#name.clone(),
            r#ao_file: x.r#ao_file.clone(),
            r#act_file: x.r#act_file.clone(),
            r#base_max_life: x.r#base_max_life.clone(),
            r#base_max_mana: x.r#base_max_mana.clone(),
            r#weapon_speed: x.r#weapon_speed.clone(),
            r#min_damage: x.r#min_damage.clone(),
            r#max_damage: x.r#max_damage.clone(),
            r#max_attack_distance: x.r#max_attack_distance.clone(),
            r#icon: x.r#icon.clone(),
            r#integer_id: x.r#integer_id.clone(),
            r#base_strength: x.r#base_strength.clone(),
            r#base_dexterity: x.r#base_dexterity.clone(),
            r#base_intelligence: x.r#base_intelligence.clone(),
            r#description: x.r#description.clone(),
            r#start_skill_gem: x.r#start_skill_gem.map(SkillGemsRow),
            r#character_size: x.r#character_size.clone(),
            r#intro_sound_file: x.r#intro_sound_file.clone(),
            r#start_weapons: x
                .r#start_weapons
                .iter()
                .copied()
                .map(BaseItemTypesRow)
                .collect(),
            r#gender: x.r#gender.clone(),
            r#trait_description: x.r#trait_description.clone(),
            r#passive_tree_image: x.r#passive_tree_image.clone(),
            r#tencent_video: x.r#tencent_video.clone(),
            r#attrs_as_id: x.r#attrs_as_id.clone(),
            r#login_screen: x.r#login_screen.clone(),
            r#player_critter: x.r#player_critter.clone(),
            r#player_effect: x.r#player_effect.clone(),
            r#after_image: x.r#after_image.clone(),
            r#mirage: x.r#mirage.map(MonsterVarietiesRow),
            r#clone_immobile: x.r#clone_immobile.map(MonsterVarietiesRow),
            r#replicate_clone: x.r#replicate_clone.map(MonsterVarietiesRow),
            r#lightning_clone: x.r#lightning_clone.map(MonsterVarietiesRow),
            r#skill_tree_background: x.r#skill_tree_background.clone(),
            r#clone: x.r#clone.map(MonsterVarietiesRow),
            r#double: x.r#double.map(MonsterVarietiesRow),
            r#mirage_warrior: x.r#mirage_warrior.map(MonsterVarietiesRow),
            r#double_two: x.r#double_two.map(MonsterVarietiesRow),
            r#dark_exile: x.r#dark_exile.map(MonsterVarietiesRow),
            r#attr: x.r#attr.clone(),
            r#attr_lowercase: x.r#attr_lowercase.clone(),
            r#script: x.r#script.clone(),
        })
        .collect()
});

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharactersRow(pub usize);

impl Deref for CharactersRow {
    type Target = Characters;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_Characters[self.0]
    }
}

impl CharactersRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static Characters {
        &TABLE_Characters[self.0]
    }
    pub fn get(&self) -> &'static Characters {
        &TABLE_Characters[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Characters.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static Characters)> {
        TABLE_Characters
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_Characters: LazyLock<Vec<CharactersRaw>> = LazyLock::new(|| {
    const DATA: &str = include_str!("../../data/tables/English/Characters.json");
    serde_json::from_str(DATA).unwrap()
});

#[derive(Debug, Deserialize, Serialize)]
struct CharactersRaw {
    #[serde(rename = "Id")]
    pub r#id: String,
    #[serde(rename = "Name")]
    pub r#name: String,
    #[serde(rename = "AOFile")]
    pub r#ao_file: String,
    #[serde(rename = "ACTFile")]
    pub r#act_file: String,
    #[serde(rename = "BaseMaxLife")]
    pub r#base_max_life: i32,
    #[serde(rename = "BaseMaxMana")]
    pub r#base_max_mana: i32,
    #[serde(rename = "WeaponSpeed")]
    pub r#weapon_speed: i32,
    #[serde(rename = "MinDamage")]
    pub r#min_damage: i32,
    #[serde(rename = "MaxDamage")]
    pub r#max_damage: i32,
    #[serde(rename = "MaxAttackDistance")]
    pub r#max_attack_distance: i32,
    #[serde(rename = "Icon")]
    pub r#icon: String,
    #[serde(rename = "IntegerId")]
    pub r#integer_id: i32,
    #[serde(rename = "BaseStrength")]
    pub r#base_strength: i32,
    #[serde(rename = "BaseDexterity")]
    pub r#base_dexterity: i32,
    #[serde(rename = "BaseIntelligence")]
    pub r#base_intelligence: i32,
    #[serde(rename = "Description")]
    pub r#description: String,
    #[serde(rename = "StartSkillGem")]
    pub r#start_skill_gem: Option<usize>,
    #[serde(rename = "CharacterSize")]
    pub r#character_size: i32,
    #[serde(rename = "IntroSoundFile")]
    pub r#intro_sound_file: String,
    #[serde(rename = "StartWeapons")]
    pub r#start_weapons: Vec<usize>,
    #[serde(rename = "Gender")]
    pub r#gender: String,
    #[serde(rename = "TraitDescription")]
    pub r#trait_description: String,
    #[serde(rename = "PassiveTreeImage")]
    pub r#passive_tree_image: String,
    #[serde(rename = "TencentVideo")]
    pub r#tencent_video: String,
    #[serde(rename = "AttrsAsId")]
    pub r#attrs_as_id: String,
    #[serde(rename = "LoginScreen")]
    pub r#login_screen: String,
    #[serde(rename = "PlayerCritter")]
    pub r#player_critter: String,
    #[serde(rename = "PlayerEffect")]
    pub r#player_effect: String,
    #[serde(rename = "AfterImage")]
    pub r#after_image: String,
    #[serde(rename = "Mirage")]
    pub r#mirage: Option<usize>,
    #[serde(rename = "CloneImmobile")]
    pub r#clone_immobile: Option<usize>,
    #[serde(rename = "ReplicateClone")]
    pub r#replicate_clone: Option<usize>,
    #[serde(rename = "LightningClone")]
    pub r#lightning_clone: Option<usize>,
    #[serde(rename = "SkillTreeBackground")]
    pub r#skill_tree_background: String,
    #[serde(rename = "Clone")]
    pub r#clone: Option<usize>,
    #[serde(rename = "Double")]
    pub r#double: Option<usize>,
    #[serde(rename = "MirageWarrior")]
    pub r#mirage_warrior: Option<usize>,
    #[serde(rename = "DoubleTwo")]
    pub r#double_two: Option<usize>,
    #[serde(rename = "DarkExile")]
    pub r#dark_exile: Option<usize>,
    #[serde(rename = "Attr")]
    pub r#attr: String,
    #[serde(rename = "AttrLowercase")]
    pub r#attr_lowercase: String,
    #[serde(rename = "Script")]
    pub r#script: String,
}
