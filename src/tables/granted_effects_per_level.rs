#![allow(clippy::all)]
#[allow(unused_imports)]
use super::*;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use std::sync::LazyLock;
#[derive(Debug)]
pub struct GrantedEffectsPerLevel {
    pub r#granted_effect: Option<GrantedEffectsRow>,
    pub r#level: i32,
    pub r#player_level_req: f32,
    pub r#cost_multiplier: i32,
    pub r#stored_uses: i32,
    pub r#cooldown: i32,
    pub r#cooldown_bypass_type: MaybeVariant<CooldownBypassTypes>,
    pub r#vaal_souls: i32,
    pub r#vaal_stored_uses: i32,
    pub r#cooldown_group: i32,
    pub r#soul_gain_prevention_duration: i32,
    pub r#attack_speed_multiplier: i32,
    pub r#cost_amounts: Vec<i32>,
    pub r#cost_types: Vec<CostTypesRow>,
    pub r#mana_reservation_flat: i32,
    pub r#mana_reservation_percent: i32,
    pub r#life_reservation_flat: i32,
    pub r#life_reservation_percent: i32,
    pub r#attack_time: i32,
}

#[allow(non_upper_case_globals)]
pub static TABLE_GrantedEffectsPerLevel: LazyLock<Vec<GrantedEffectsPerLevel>> =
    LazyLock::new(|| {
        RAW_TABLE_GrantedEffectsPerLevel
            .iter()
            .map(|x| GrantedEffectsPerLevel {
                r#granted_effect: x.r#granted_effect.map(GrantedEffectsRow),
                r#level: x.r#level.clone(),
                r#player_level_req: x.r#player_level_req.clone(),
                r#cost_multiplier: x.r#cost_multiplier.clone(),
                r#stored_uses: x.r#stored_uses.clone(),
                r#cooldown: x.r#cooldown.clone(),
                r#cooldown_bypass_type: CooldownBypassTypes::from_repr(x.r#cooldown_bypass_type)
                    .map_or(
                        MaybeVariant::NotVariant(x.r#cooldown_bypass_type),
                        MaybeVariant::Variant,
                    ),
                r#vaal_souls: x.r#vaal_souls.clone(),
                r#vaal_stored_uses: x.r#vaal_stored_uses.clone(),
                r#cooldown_group: x.r#cooldown_group.clone(),
                r#soul_gain_prevention_duration: x.r#soul_gain_prevention_duration.clone(),
                r#attack_speed_multiplier: x.r#attack_speed_multiplier.clone(),
                r#cost_amounts: x.r#cost_amounts.clone(),
                r#cost_types: x.r#cost_types.iter().copied().map(CostTypesRow).collect(),
                r#mana_reservation_flat: x.r#mana_reservation_flat.clone(),
                r#mana_reservation_percent: x.r#mana_reservation_percent.clone(),
                r#life_reservation_flat: x.r#life_reservation_flat.clone(),
                r#life_reservation_percent: x.r#life_reservation_percent.clone(),
                r#attack_time: x.r#attack_time.clone(),
            })
            .collect()
    });

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct GrantedEffectsPerLevelRow(pub usize);

impl Deref for GrantedEffectsPerLevelRow {
    type Target = GrantedEffectsPerLevel;

    fn deref(&self) -> &'static Self::Target {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
}

impl GrantedEffectsPerLevelRow {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static GrantedEffectsPerLevel {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
    pub fn get(&self) -> &'static GrantedEffectsPerLevel {
        &TABLE_GrantedEffectsPerLevel[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_GrantedEffectsPerLevel
            .iter()
            .enumerate()
            .map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static GrantedEffectsPerLevel)> {
        TABLE_GrantedEffectsPerLevel
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[allow(non_upper_case_globals)]
static RAW_TABLE_GrantedEffectsPerLevel: LazyLock<Vec<GrantedEffectsPerLevelRaw>> =
    LazyLock::new(|| {
        const DATA: &str = include_str!("../../data/tables/English/GrantedEffectsPerLevel.json");
        serde_json::from_str(DATA).unwrap()
    });

#[derive(Debug, Deserialize, Serialize)]
struct GrantedEffectsPerLevelRaw {
    #[serde(rename = "GrantedEffect")]
    pub r#granted_effect: Option<usize>,
    #[serde(rename = "Level")]
    pub r#level: i32,
    #[serde(rename = "PlayerLevelReq")]
    pub r#player_level_req: f32,
    #[serde(rename = "CostMultiplier")]
    pub r#cost_multiplier: i32,
    #[serde(rename = "StoredUses")]
    pub r#stored_uses: i32,
    #[serde(rename = "Cooldown")]
    pub r#cooldown: i32,
    #[serde(rename = "CooldownBypassType")]
    pub r#cooldown_bypass_type: usize,
    #[serde(rename = "VaalSouls")]
    pub r#vaal_souls: i32,
    #[serde(rename = "VaalStoredUses")]
    pub r#vaal_stored_uses: i32,
    #[serde(rename = "CooldownGroup")]
    pub r#cooldown_group: i32,
    #[serde(rename = "SoulGainPreventionDuration")]
    pub r#soul_gain_prevention_duration: i32,
    #[serde(rename = "AttackSpeedMultiplier")]
    pub r#attack_speed_multiplier: i32,
    #[serde(rename = "CostAmounts")]
    pub r#cost_amounts: Vec<i32>,
    #[serde(rename = "CostTypes")]
    pub r#cost_types: Vec<usize>,
    #[serde(rename = "ManaReservationFlat")]
    pub r#mana_reservation_flat: i32,
    #[serde(rename = "ManaReservationPercent")]
    pub r#mana_reservation_percent: i32,
    #[serde(rename = "LifeReservationFlat")]
    pub r#life_reservation_flat: i32,
    #[serde(rename = "LifeReservationPercent")]
    pub r#life_reservation_percent: i32,
    #[serde(rename = "AttackTime")]
    pub r#attack_time: i32,
}
