#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_NPCTalk: LazyLock<Vec<NPCTalkRow>> = LazyLock::new(|| {
    let df = DAT_LOADER.write().unwrap().get_table("data/balance/npctalk.datc64").unwrap().clone();
    df.rows_iter()
        .map(|row| NPCTalkRow {
            r#npc_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(0..0 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCsRef::new(value as usize)
            },
            r#unknown16: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(16..16 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#dialogue_option: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(20..20 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown28: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(28..28 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#unknown44: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(44..44 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#unknown60: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(60..60 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| QuestFlagsRef::new(value as usize)).collect()
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(76..76 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#text_audio: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(84..84 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTextAudioRef::new(value as usize)
            },
            r#category: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(100..100 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTalkCategoryRef::new(value as usize)
            },
            r#quest_reward_offers_key: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(116..116 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestRewardOffersRef::new(value as usize)
            },
            r#quest_flag: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(132..132 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                QuestFlagsRef::new(value as usize)
            },
            r#npc_text_audio_keys: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(148..148 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values.into_iter().map(|value| NPCTextAudioRef::new(value as usize)).collect()
            },
            r#script2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(164..164 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown172: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(172).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown173: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(173).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown174: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(174..174 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown190: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(190..190 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown206: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(206..206 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown210: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(210..210 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown226: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(226..226 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown230: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(230).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown231: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(231..231 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown247: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(247..247 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 4)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i32_le())
                    .collect::<Vec<i32>>();
                values
            },
            r#unknown263: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(263).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#dialogue_option2: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(264..264 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown272: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(272..272 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown288: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(288..288 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown304: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(304..304 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown308: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(308..308 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
            r#unknown324: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(324..324 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown328: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(328..328 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown344: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(344..344 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown348: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(348..348 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown364: {
                // array_mutator column.array == false && column.type == 'bool'
                let cell_bytes = row.get(364).unwrap();
                let value = cell_bytes.to_le() != 0;
                value
            },
            r#unknown365: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(365..365 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown369: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(369..369 + 8).unwrap();
                let value = cell_bytes.get_i64_le();
                NPCTalkRef::new(value as usize)
            },
            r#unknown377: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(377..377 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type else
                let values = df
                    .array_from_offset(offset, count, 16)
                    .unwrap()
                    .iter()
                    .map(|x| x.clone().get_i64_le())
                    .collect::<Vec<i64>>();
                values
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct NPCTalkRow {
    pub r#npc_key: NPCsRef,
    pub r#unknown16: i32,
    pub r#dialogue_option: String,
    pub r#unknown28: Vec<QuestFlagsRef>,
    pub r#unknown44: Vec<QuestFlagsRef>,
    pub r#unknown60: Vec<QuestFlagsRef>,
    pub r#script: String,
    pub r#text_audio: NPCTextAudioRef,
    pub r#category: NPCTalkCategoryRef,
    pub r#quest_reward_offers_key: QuestRewardOffersRef,
    pub r#quest_flag: QuestFlagsRef,
    pub r#npc_text_audio_keys: Vec<NPCTextAudioRef>,
    pub r#script2: String,
    pub r#unknown172: bool,
    pub r#unknown173: bool,
    pub r#unknown174: Vec<i32>,
    pub r#unknown190: Vec<i32>,
    pub r#unknown206: i32,
    pub r#unknown210: Vec<i32>,
    pub r#unknown226: i32,
    pub r#unknown230: bool,
    pub r#unknown231: i64,
    pub r#unknown247: Vec<i32>,
    pub r#unknown263: bool,
    pub r#dialogue_option2: String,
    pub r#unknown272: i64,
    pub r#unknown288: i64,
    pub r#unknown304: i32,
    pub r#unknown308: Vec<i64>,
    pub r#unknown324: i32,
    pub r#unknown328: i64,
    pub r#unknown344: i32,
    pub r#unknown348: i64,
    pub r#unknown364: bool,
    pub r#unknown365: i32,
    pub r#unknown369: NPCTalkRef,
    pub r#unknown377: Vec<i64>,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct NPCTalkRef(pub usize);

impl Deref for NPCTalkRef {
    type Target = NPCTalkRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_NPCTalk[self.0]
    }
}

impl NPCTalkRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static NPCTalkRow {
        &TABLE_NPCTalk[self.0]
    }
    pub fn get(&self) -> &'static NPCTalkRow {
        &TABLE_NPCTalk[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_NPCTalk.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static NPCTalkRow)> {
        TABLE_NPCTalk.iter().enumerate().map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::hint::black_box;
    #[test]
    fn get_all_rows() {
        for row in TABLE_NPCTalk.iter() {
            black_box(row);
        }
    }
}
