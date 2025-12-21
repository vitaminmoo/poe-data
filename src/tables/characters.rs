#![allow(clippy::all)]
use bytes::Buf;

use crate::dat_parser::DAT_LOADER;

#[allow(unused)]
use super::*;
use std::{ops::Deref, sync::LazyLock};

#[allow(non_upper_case_globals)]
pub static TABLE_Characters: LazyLock<Vec<CharactersRow>> = LazyLock::new(|| {
    let df = DAT_LOADER
        .write()
        .unwrap()
        .get_table("data/balance/characters.datc64")
        .unwrap()
        .clone();
    df.rows_iter()
        .map(|row| CharactersRow {
            r#id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(0..0 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#name: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(8..8 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#ao_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(16..16 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#act_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(24..24 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#base_max_life: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(32..32 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_max_mana: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(36..36 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#weapon_speed: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(40..40 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#min_damage: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(44..44 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_damage: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(48..48 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#max_attack_distance: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(52..52 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(56..56 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#integer_id: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(64..64 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_strength: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(68..68 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_dexterity: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(72..72 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#base_intelligence: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(76..76 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown80: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(80..80 + 16).unwrap();
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
            r#description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(96..96 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#start_skill_gem: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(104..104 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                BaseItemTypesRef::new(value as usize)
            },
            r#unknown120: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(120..120 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown136: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(136..136 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown140: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(140..140 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#character_size: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(144..144 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown148: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(148..148 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown152: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(152..152 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown156: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(156..156 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#intro_sound_file: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(160..160 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#start_weapons: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(168..168 + 16).unwrap();
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
                    .into_iter()
                    .map(|value| BaseItemTypesRef::new(value as usize))
                    .collect()
            },
            r#gender: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(184..184 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#trait_description: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(192..192 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown200: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(200..200 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown216: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(216..216 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown232: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(232..232 + 16).unwrap();
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
            r#unknown248: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(248..248 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown264: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(264..264 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown280: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(280..280 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#passive_tree_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(284..284 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown292: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(292..292 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown296: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(296..296 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#attrs_as_id: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(300..300 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#login_screen: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(308..308 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#player_critter: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(316..316 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#player_effect: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(324..324 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#after_image: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(332..332 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#mirage: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(340..340 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#clone_immobile: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(356..356 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#replicate_clone: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(372..372 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#lightning_clone: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(388..388 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown404: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(404..404 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown408: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(408..408 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#skill_tree_background: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(412..412 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#clone: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(420..420 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#unknown436: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(436..436 + 16).unwrap();
                let count = cell_bytes.get_u64_le() as usize;
                let offset = cell_bytes.get_u64_le() as usize;
                // array_mutator column.array == true && column.type == 'array'
                let values = (count, offset);
                values
            },
            r#mirage_warrior: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(452..452 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#double_two: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(468..468 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#dark_exile: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(484..484 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                MonsterVarietiesRef::new(value as usize)
            },
            r#attr: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(500..500 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#attr_lowercase: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(508..508 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#script: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(516..516 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown524: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(524..524 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown540: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(540..540 + 4).unwrap();
                let value = cell_bytes.get_i32_le();
                value
            },
            r#unknown544: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(544..544 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown548: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(548..548 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#unknown552: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(552..552 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#base_class: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(556..556 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown564: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(564..564 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown580: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(580..580 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown596: {
                // array_mutator column.array == true
                let mut cell_bytes = row.get(596..596 + 16).unwrap();
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
            r#unknown612: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(612..612 + 4).unwrap();
                let value = cell_bytes.get_f32_le();
                value
            },
            r#gem_cutting_icon: {
                // array_mutator column.array == false && column.type == 'string'
                let mut cell_bytes = row.get(616..616 + 8).unwrap();
                let offset = cell_bytes.get_i32_le() as usize;
                let value = df.string_from_offset(offset).unwrap();
                value
            },
            r#unknown624: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(624..624 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
            r#unknown640: {
                // array_mutator column.array == false && column.type != 'string|bool'
                let mut cell_bytes = row.get(640..640 + 16).unwrap();
                let value = cell_bytes.get_i64_le();
                value
            },
        })
        .collect()
});

#[derive(Debug)]
pub struct CharactersRow {
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
    pub r#unknown80: Vec<i64>,
    pub r#description: String,
    pub r#start_skill_gem: BaseItemTypesRef,
    pub r#unknown120: i64,
    pub r#unknown136: i32,
    pub r#unknown140: i32,
    pub r#character_size: i32,
    pub r#unknown148: i32,
    pub r#unknown152: i32,
    pub r#unknown156: i32,
    pub r#intro_sound_file: String,
    pub r#start_weapons: Vec<BaseItemTypesRef>,
    pub r#gender: String,
    pub r#trait_description: String,
    pub r#unknown200: i64,
    pub r#unknown216: i64,
    pub r#unknown232: Vec<i64>,
    pub r#unknown248: i64,
    pub r#unknown264: i64,
    pub r#unknown280: i32,
    pub r#passive_tree_image: String,
    pub r#unknown292: i32,
    pub r#unknown296: i32,
    pub r#attrs_as_id: String,
    pub r#login_screen: String,
    pub r#player_critter: String,
    pub r#player_effect: String,
    pub r#after_image: String,
    pub r#mirage: MonsterVarietiesRef,
    pub r#clone_immobile: MonsterVarietiesRef,
    pub r#replicate_clone: MonsterVarietiesRef,
    pub r#lightning_clone: MonsterVarietiesRef,
    pub r#unknown404: f32,
    pub r#unknown408: f32,
    pub r#skill_tree_background: String,
    pub r#clone: MonsterVarietiesRef,
    pub r#unknown436: (usize, usize),
    pub r#mirage_warrior: MonsterVarietiesRef,
    pub r#double_two: MonsterVarietiesRef,
    pub r#dark_exile: MonsterVarietiesRef,
    pub r#attr: String,
    pub r#attr_lowercase: String,
    pub r#script: String,
    pub r#unknown524: i64,
    pub r#unknown540: i32,
    pub r#unknown544: f32,
    pub r#unknown548: f32,
    pub r#unknown552: f32,
    pub r#base_class: String,
    pub r#unknown564: i64,
    pub r#unknown580: i64,
    pub r#unknown596: Vec<i64>,
    pub r#unknown612: f32,
    pub r#gem_cutting_icon: String,
    pub r#unknown624: i64,
    pub r#unknown640: i64,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, Ord, PartialOrd)]
pub struct CharactersRef(pub usize);

impl Deref for CharactersRef {
    type Target = CharactersRow;
    fn deref(&self) -> &'static Self::Target {
        &TABLE_Characters[self.0]
    }
}

impl CharactersRef {
    pub fn new(index: usize) -> Self {
        Self(index)
    }
    pub fn as_static_ref(&self) -> &'static CharactersRow {
        &TABLE_Characters[self.0]
    }
    pub fn get(&self) -> &'static CharactersRow {
        &TABLE_Characters[self.0]
    }
    pub fn iter() -> impl Iterator<Item = Self> {
        TABLE_Characters.iter().enumerate().map(|(i, _)| Self(i))
    }
    pub fn iter_with_refs() -> impl Iterator<Item = (Self, &'static CharactersRow)> {
        TABLE_Characters
            .iter()
            .enumerate()
            .map(|(i, x)| (Self(i), x))
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn print_all_rows() {
        // Print all rows
        for row in TABLE_Characters.iter() {
            println!("{:?}", row);
        }
    }
}
