use core::panic;
use std::collections::{BTreeMap, HashMap};
use std::fmt;
use std::str::FromStr;
use std::sync::{LazyLock, RwLock};

use crate::statdescriptions_parser;

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone)]
pub enum Language {
    English,
    French,
    German,
    Korean,
    Portuguese,
    Russian,
    SimplifiedChinese,
    Spanish,
    Thai,
    TraditionalChinese,
    Japanese,
}

impl FromStr for Language {
    type Err = ();
    fn from_str(input: &str) -> Result<Language, Self::Err> {
        match input {
            "English" => Ok(Language::English),
            "French" => Ok(Language::French),
            "German" => Ok(Language::German),
            "Japanese" => Ok(Language::Japanese),
            "Korean" => Ok(Language::Korean),
            "Portuguese" => Ok(Language::Portuguese),
            "Russian" => Ok(Language::Russian),
            "Simplified Chinese" => Ok(Language::SimplifiedChinese),
            "Spanish" => Ok(Language::Spanish),
            "Thai" => Ok(Language::Thai),
            "Traditional Chinese" => Ok(Language::TraditionalChinese),
            _ => Err(()),
        }
    }
}

#[derive(Default)]
pub struct StatLoader {
    // path -> stat file struct
    stat_files: HashMap<String, StatFile>,
}

/*
description asdf
    2 chance_to_gain_onslaught_on_kill_% onslaught_time_granted_on_kill_ms
    4
        1|# 0 "{0}% chance to gain Onslaught for 4 seconds on Kill" reminderstring ReminderTextOnslaught
        100 0 "Gain Onslaught for 4 seconds on Kill" reminderstring ReminderTextOnslaught
        1|# 1|# "{0}% chance to gain Onslaught for {1} seconds on Kill" milliseconds_to_seconds 2 reminderstring ReminderTextOnslaught
        100 1|# "Gain Onslaught for {1} seconds on Kill" milliseconds_to_seconds 2 reminderstring ReminderTextOnslaught
*/

#[derive(Default, Debug)]
pub struct StatFile {
    pub no_descriptions: Vec<String>,
    pub has_identifiers: bool,
    pub no_identifiers: bool,
    pub descriptors: HashMap<
        Vec<String>, // key = Vec<stat_keys>, len = number of statkeys/statvalues (e.g. 2)
        Descriptor,  //
    >,
    pub indexed_descriptors: HashMap<String, (Vec<String>, Descriptor)>,
}

#[derive(Default, Debug, Clone)]
pub struct Descriptor {
    pub name: Option<String>,
    pub linespecs: Vec<LineSpec>,
}

impl Descriptor {
    fn find(&self, vals: &[i32]) -> Option<&LineSpec> {
        self.linespecs.iter().find(|m| m.matches(vals))
    }
}

#[derive(Default, Debug, Clone)]
pub struct LineSpec {
    pub conditions: Vec<Pattern>, // len = number of conditions
    // the actual text of the stat, with {0} {1} etc. placeholders for the stat values
    pub text: String,
    // outer vec is for each stat, inner vec is functions to run in order to
    // modify the stat value or appearance
    pub line_functions: Vec<LineFunction>,
}

impl LineSpec {
    fn matches(&self, vals: &[i32]) -> bool {
        self.conditions
            .iter()
            .enumerate()
            .all(|(i, c)| c.matches(vals[i]))
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Pattern {
    Any,
    NotZero,
    Exactly(i32),
    GreaterOrEqual(i32),
    LessOrEqual(i32),
    Between((i32, i32)),
}

#[derive(Debug, Copy, Clone)]
pub enum PatternParseError {
    PatternParseError,
}

impl TryFrom<String> for Pattern {
    type Error = PatternParseError;
    fn try_from(pattern: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = pattern.splitn(2, '|').collect();

        match parts.len() {
            0 => Err(PatternParseError::PatternParseError),
            1 => {
                if pattern == "!0" {
                    Ok(Pattern::NotZero)
                } else if parts[0] == "#" {
                    Ok(Pattern::Any)
                } else {
                    Err(PatternParseError::PatternParseError)
                }
            }
            2 => {
                if parts[0] == "#" {
                    let val: i32 = parts[1].parse().unwrap();
                    Ok(Pattern::LessOrEqual(val))
                } else if parts[1] == "#" {
                    let val: i32 = parts[0].parse().unwrap();
                    Ok(Pattern::GreaterOrEqual(val))
                } else {
                    let lval = parts[0].parse().unwrap();
                    let rval = parts[1].parse().unwrap();
                    Ok(Pattern::Between((lval, rval)))
                }
            }
            _ => Err(PatternParseError::PatternParseError),
        }
    }
}

impl Pattern {
    fn matches(&self, val: i32) -> bool {
        match self {
            Pattern::Any => true,
            Pattern::NotZero => val != 0,
            Pattern::Exactly(x) => val == *x,
            Pattern::GreaterOrEqual(x) => val >= *x,
            Pattern::LessOrEqual(x) => val <= *x,
            Pattern::Between((l, r)) => *l <= val && val <= *r,
        }
    }
}

pub static STAT_LOADER: LazyLock<RwLock<StatLoader>> =
    LazyLock::new(|| RwLock::new(StatLoader::default()));

impl StatLoader {
    pub fn load_file(&mut self, path: &str) {
        if self.stat_files.contains_key(path) {
            return;
        }
        match statdescriptions_parser::load_file(path) {
            Ok(parsed) => {
                self.stat_files.insert(path.to_string(), parsed);
            }
            Err(err) => panic!("Couldn't load path {}: {}", path, err),
        }
    }

    pub fn get_affix_text(&mut self, path: &str, mut vals: BTreeMap<String, i32>) -> String {
        self.load_file(path);

        let mut stats = Vec::new();
        for stat_key in vals.clone().keys() {
            if !vals.contains_key(stat_key) {
                continue;
            }
            if let Some((ids, descriptor)) = self.stat_files[path].indexed_descriptors.get(stat_key)
            {
                let mut stat = Vec::new();
                for id in ids.iter() {
                    if let Some(x) = vals.remove_entry(id) {
                        stat.push(x);
                    } else {
                        stat.push((id.to_string(), 0));
                    }
                }
                stats.push((stat, descriptor));
            }
        }

        assert!(!stats.is_empty(), "no stats found");

        // I don't understand why but it's backwards without this
        stats.reverse();

        let mut texts: Vec<String> = Vec::new();
        for stat in stats.iter() {
            let (stat, descriptor) = stat;

            if let Some(description) =
                descriptor.find(&stat.iter().map(|(_, v)| *v).collect::<Vec<_>>())
            {
                let stat_vals: Vec<_> = stat.iter().map(|(_, v)| Stat::new(*v as f64)).collect();
                let mut stats_line = StatsLine {
                    stats: stat_vals,
                    markup: false,
                    canonical_line: false,
                    reminder_strings: Vec::new(),
                };

                for lf in description.line_functions.iter() {
                    lf.apply(&mut stats_line);
                }
                let mut text = description.text.clone();

                for (i, c) in stats_line.stats.iter().enumerate() {
                    // {}
                    if i == 0 {
                        text = text.replace("{}", &c.to_string());
                    }

                    // {0}
                    let fmt = format!("{{{}}}", i);
                    text = text.replace(&fmt, &c.to_string());

                    // {0:+d}
                    let fmt = format!("{{{}:+d}}", i);
                    if c.f < 0.0 {
                        text = text.replace(&fmt, &format!("-{}", &c));
                    } else if c.f > 0.0 {
                        text = text.replace(&fmt, &format!("+{}", &c));
                    } else {
                        text = text.replace(&fmt, &c.to_string());
                    }

                    // {0:d}
                    let fmt = format!("{{{}:d}}", i);
                    text = text.replace(&fmt, &c.to_string());
                }
                texts.push(text);
            } else {
                panic!(
                    "No descriptor found for statkeys: {:?}",
                    stat.iter().map(|(x, _)| x)
                );
            }
        }

        texts.join("\n")
    }
}

#[derive(Debug, PartialEq)]
pub struct Stat {
    // float value of the stat. Will be either StatNMin/StatNMax from the Mods
    // table if we're displaying the possibilities for an affix, or the actual
    // rolled stat if it's being displayed for a specific instance of an item
    pub f: f64,
    // Some functions translate stats to strings, so if this is set it should be
    // interpolated directly
    pub s: Option<String>,
    // format string for the stat value, used for controlling displayed
    // precision when interpolating into a translation string
    pub fmt: String,
    // canonical_stat exists for this stat - some sorting shit for trade sites
    pub canonical: bool,
}
impl Stat {
    pub fn new(f: f64) -> Self {
        Stat {
            f,
            s: None,
            fmt: "{}".to_string(),
            canonical: false,
        }
    }
}

impl fmt::Display for Stat {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.s.is_some() {
            write!(f, "{}", self.s.as_ref().unwrap())
        } else {
            match self.fmt.as_str() {
                "{.0}" => write!(f, "{:.0}", self.f),
                "{.1}" => write!(f, "{:.1}", self.f),
                "{.2}" => write!(f, "{:.2}", self.f),
                _ => write!(f, "{}", self.f),
            }
        }
    }
}

#[derive(Debug, Clone)]
pub enum StatFunction {
    DivideBy(f64),
    DivideByRound(f64, u32),
    MultiplyBy(f64),
    Plus(f64),
    InvertChance,
    PassiveHash,
    Precision(u32),
    IfRequired,
    CanonicalStat,
    Unsupported,
}
impl StatFunction {
    pub fn apply(&self, s: &mut Stat) {
        match self {
            StatFunction::DivideBy(x) => {
                s.f /= x;
                s.fmt = "{.0}".to_owned();
            }
            StatFunction::DivideByRound(x, dp) => {
                s.f /= x;
                s.fmt = match dp {
                    0 => "{.0}".to_owned(),
                    1 => "{.1}".to_owned(),
                    2 => "{.2}".to_owned(),
                    _ => unreachable!(),
                }
            }
            StatFunction::MultiplyBy(x) => s.f *= x,
            StatFunction::Plus(x) => s.f += x,
            StatFunction::InvertChance => s.f = 100.0 - s.f,
            StatFunction::PassiveHash => {
                if s.f < 0.0 {
                    s.f += 65536.0
                }
            }
            StatFunction::Precision(x) => s.fmt = format!("{{.{}}}", x),
            StatFunction::IfRequired => {
                if s.f == s.f.floor() {
                    s.fmt = "{.5}".to_string()
                }
            }
            StatFunction::CanonicalStat => s.canonical = true,
            StatFunction::Unsupported => {}
        }
    }
}

#[derive(Debug, Clone)]
pub enum LineFunction {
    StatFunction(StatFunction, usize),
    Markup,
    CanonicalLine,
    ReminderString(String),
}

pub struct StatsLine {
    stats: Vec<Stat>,
    markup: bool,
    canonical_line: bool,
    reminder_strings: Vec<String>,
}

impl LineFunction {
    pub fn apply(&self, stats_line: &mut StatsLine) {
        match self {
            LineFunction::StatFunction(stat_function, idx) => {
                for (i, stat) in stats_line.stats.iter_mut().enumerate() {
                    if i == *idx {
                        stat_function.apply(stat)
                    }
                }
            }
            LineFunction::Markup => stats_line.markup = true,
            LineFunction::CanonicalLine => stats_line.canonical_line = true,
            LineFunction::ReminderString(reminder) => {
                stats_line.reminder_strings.push(reminder.clone())
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn basic() {
        let path = "Metadata/StatDescriptions/Stat_Descriptions.txt";
        let mut sl = StatLoader::default();

        let result = sl.get_affix_text(
            path,
            [
                ("local_minimum_added_chaos_damage".to_string(), 98),
                ("local_maximum_added_chaos_damage".to_string(), 140),
            ]
            .into_iter()
            .collect(),
        );

        assert_eq!(result, "Adds 98 to 140 Chaos Damage");
    }

    #[test]
    fn missing_key() {
        let path = "Metadata/StatDescriptions/Stat_Descriptions.txt";
        let mut sl = StatLoader::default();

        // first key only
        let result = sl.get_affix_text(
            path,
            [("local_physical_damage_+%".to_string(), -50)]
                .into_iter()
                .collect(),
        );
        assert_eq!(result, "50% reduced Physical Damage");

        // second key only
        let result = sl.get_affix_text(
            path,
            [("local_weapon_no_physical_damage".to_string(), 1)]
                .into_iter()
                .collect(),
        );
        assert_eq!(result, "No Physical Damage");
    }

    #[test]
    fn two_separate_stats() {
        let path = "Metadata/StatDescriptions/Stat_Descriptions.txt";
        let mut sl = StatLoader::default();

        let result = sl.get_affix_text(
            path,
            BTreeMap::from([
                ("local_evasion_rating_+%".to_string(), 80),
                ("base_maximum_life".to_string(), 10),
            ]),
        );

        assert_eq!(result, "80% increased Evasion Rating\n+10 to maximum Life");
    }

    #[test]
    fn with_math() {
        let path = "Metadata/StatDescriptions/Stat_Descriptions.txt";
        let mut sl = StatLoader::default();
        assert_eq!(
            sl.get_affix_text(
                path,
                [("chest_item_quantity_+%".to_string(), 50)]
                    .into_iter()
                    .collect()
            ),
            "50% increased Quantity of Items Found from Chests"
        );
        assert_eq!(
            sl.get_affix_text(
                path,
                [("chest_item_quantity_+%".to_owned(), -50)]
                    .into_iter()
                    .collect()
            ),
            "50% reduced Quantity of Items Found from Chests"
        );
    }
}
