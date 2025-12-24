use std::collections::HashMap;
use std::env;
use std::fs::File;
use std::io::{BufReader, Read};
use std::str::FromStr;

use winnow::ascii::{dec_int, dec_uint, digit1, float, multispace0, multispace1, space1};
use winnow::combinator::{alt, delimited, empty, opt, preceded, repeat, separated, separated_pair, terminated};
use winnow::error::{ContextError, ParseError};
use winnow::stream::Stream;
use winnow::token::{take, take_until, take_while};
use winnow::ModalResult;
use winnow::Parser;

use crate::statdescriptions::{self, Descriptor, Language, LineFunction, LineSpec, Pattern, StatFunction};

pub fn load_file(path: &str) -> Result<statdescriptions::StatFile, StatLoaderError> {
    let real_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("data/files").join(path.replace('/', "@"));
    let file = File::open(real_path.clone()).unwrap_or_else(|x| panic!("Failed to open file {:?}: {}", real_path, x));
    let mut raw: Vec<u8> = Vec::new();
    let mut bytes = BufReader::new(file);
    bytes.read_to_end(&mut raw).expect("Failed to read file");

    let converted: Vec<u16> = raw.chunks_exact(2).map(|chunk| u16::from_le_bytes([chunk[0], chunk[1]])).collect();

    let decoded = String::from_utf16_lossy(&converted);

    stats_file.parse(decoded.as_str()).map_err(|e| StatLoaderError::from_parse(e, &decoded))
}

#[derive(Debug)]
pub struct StatLoaderError {
    message: String,
    span: std::ops::Range<usize>,
    input: String,
}

impl StatLoaderError {
    pub fn from_parse(error: ParseError<&str, ContextError>, input: &str) -> Self {
        let message = error.inner().to_string();
        let input = input.to_owned();
        let start = error.offset();
        let end = (start + 1..).find(|e| input.is_char_boundary(*e)).unwrap_or(start);
        Self {
            message,
            span: start..end,
            input,
        }
    }
}

impl std::fmt::Display for StatLoaderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = annotate_snippets::Level::Error.title(&self.message).snippet(
            annotate_snippets::Snippet::source(&self.input)
                .fold(true)
                .annotation(annotate_snippets::Level::Error.span(self.span.clone())),
        );
        let renderer = annotate_snippets::Renderer::plain();
        let rendered = renderer.render(message);
        rendered.fmt(f)
    }
}

impl std::error::Error for StatLoaderError {}

/*
    Primitive types
*/
fn doublequoted<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    delimited('"', take_until(0.., '"'), '"').parse_next(input)
}

/*
    Main file parser
*/
pub fn stats_file(input: &mut &str) -> ModalResult<statdescriptions::StatFile> {
    let _ = take(1_usize).parse_next(input)?; // skip 0xfeff
    let _ = multispace0.parse_next(input)?; // leading whitespace if there

    let _includes: Vec<String> = terminated(includes, multispace0).parse_next(input)?;
    let no_descriptions: Vec<String> = terminated(no_descriptions, multispace0).parse_next(input)?;

    let no_identifiers = opt(terminated("no_identifiers", multispace1)).parse_next(input)?.is_some();
    let has_identifiers = opt(terminated("has_identifiers", multispace1)).parse_next(input)?.is_some();

    let descriptions: HashMap<Vec<String>, Descriptor> = separated(1.., description, multispace1).parse_next(input)?;
    let _ = multispace0.parse_next(input)?;

    let mut indexed_descriptions: HashMap<String, (Vec<String>, Descriptor)> = HashMap::new();
    for (keys, desc) in descriptions.iter() {
        for key in keys.iter() {
            indexed_descriptions.insert(key.to_string(), (keys.clone(), desc.clone()));
        }
    }

    Ok(statdescriptions::StatFile {
        no_descriptions,
        has_identifiers,
        no_identifiers,
        descriptors: descriptions,
        indexed_descriptors: indexed_descriptions,
    })
}

/*
    Child parsers in order of appearance-ish
*/
pub fn includes(input: &mut &str) -> ModalResult<Vec<String>> {
    separated(0.., preceded(("include", space1), doublequoted).map(|x| x.to_string()), multispace1).parse_next(input)
}

pub fn no_descriptions(input: &mut &str) -> ModalResult<Vec<String>> {
    separated(0.., preceded(("no_description", space1), identifier).map(|x| x.to_string()), multispace1).parse_next(input)
}

pub fn description(input: &mut &str) -> ModalResult<(Vec<String>, Descriptor)> {
    let name: Option<&str> = delimited("description", opt(preceded(space1, identifier)), multispace1).parse_next(input)?;

    let num_keys: usize = terminated(digit1, space1).map(|x: &str| x.parse::<usize>().unwrap()).parse_next(input)?;

    let keys: Vec<&str> = terminated(separated(num_keys, identifier, space1), multispace1).parse_next(input)?;

    let english_linespecs = statsblock(num_keys).parse_next(input)?;

    let checkpoint = input.checkpoint();
    let _other_linespecs: HashMap<Language, Vec<LineSpec>> = preceded(
        multispace1,
        separated(
            0..,
            (
                delimited(
                    ("lang", space1),
                    doublequoted.map(|x: &str| Language::from_str(x).unwrap_or_else(|_| panic!("language {}", x))),
                    multispace1,
                ),
                statsblock(num_keys),
            ),
            multispace1,
        ),
    )
    .parse_next(input)?;

    if _other_linespecs.is_empty() {
        input.reset(&checkpoint);
    }

    Ok((
        keys.iter().map(|x| x.to_string()).collect(),
        Descriptor {
            name: name.map(|s| s.to_string()),
            linespecs: english_linespecs,
        },
    ))
}

pub fn statsblock(num_keys: usize) -> impl FnMut(&mut &str) -> ModalResult<Vec<LineSpec>> {
    move |input: &mut &str| {
        let num_lines: usize = terminated(digit1, multispace1).map(|x: &str| x.parse::<usize>().unwrap()).parse_next(input)?;

        separated(num_lines, linespec(num_keys), multispace1).parse_next(input)
    }
}

pub fn linespec(num_keys: usize) -> impl FnMut(&mut &str) -> ModalResult<LineSpec> {
    move |input: &mut &str| {
        let conditions: Vec<Pattern> = terminated(separated(num_keys, pattern_part, space1), space1).parse_next(input)?;
        let text = doublequoted.parse_next(input)?;
        let extras = extra.parse_next(input)?;
        Ok(LineSpec {
            conditions,
            text: text.to_string(),
            line_functions: extras,
        })
    }
}

fn pattern_part(input: &mut &str) -> ModalResult<Pattern> {
    alt((
        "!0".value(Pattern::NotZero),
        terminated(dec_int, "|#").map(Pattern::GreaterOrEqual),
        preceded("#|", dec_int).map(Pattern::LessOrEqual),
        terminated(dec_int, "|1|#").map(Pattern::GreaterOrEqual), // broken bullshit in data; make it work
        separated_pair(dec_int, "|", dec_int).map(Pattern::Between),
        "#".value(Pattern::Any),
        dec_int.map(Pattern::Exactly),
    ))
    .parse_next(input)
}

pub fn value(input: &mut &str) -> ModalResult<f64> {
    let numword = alt([
        "one".value(1.0),
        "two".value(2.0),
        "three".value(3.0),
        "four".value(4.0),
        "five".value(5.0),
        "six".value(6.0),
        "seven".value(7.0),
        "eight".value(8.0),
        "nine".value(9.0),
        "ten".value(10.0),
        "eleven".value(11.0),
        "twelve".value(12.0),
        "thirteen".value(13.0),
        "fourteen".value(14.0),
        "fifteen".value(15.0),
        "sixteen".value(16.0),
        "fifteen".value(15.0),
        "sixteen".value(16.0),
        "seventeen".value(17.0),
        "eighteen".value(18.0),
        "nineteen".value(19.0),
        "twenty".value(20.0),
        "thirty".value(30.0),
        "fourty".value(40.0),
        "fifty".value(50.0),
        "sixty".value(60.0),
        "seventy".value(70.0),
        "eighty".value(80.0),
        "ninety".value(90.0),
    ]);
    let decimal = alt((
        "point_one".value(0.1),
        "point_two".value(0.2),
        "point_three".value(0.3),
        "point_four".value(0.4),
        "point_five".value(0.5),
        "point_six".value(0.6),
        "point_seven".value(0.7),
        "point_eight".value(0.8),
        "point_nine".value(0.9),
    ));
    let magword = alt(("hundred".value(100.0), "thousand".value(1000.0), "million".value(1_000_000.0)));

    let x = alt((numword, float)).parse_next(input)?;
    let y = alt((preceded("_", decimal), empty.value(0.0))).parse_next(input)?;
    let z = alt((preceded("_", magword), empty.value(1.0))).parse_next(input)?;
    Ok((x + y) * z)
}

pub fn identifier<'a>(input: &mut &'a str) -> ModalResult<&'a str> {
    take_while(1.., ('a'..='z', 'A'..='Z', '0'..='9', '%', '-', '_', '+')).parse_next(input)
}

pub fn extra(input: &mut &str) -> ModalResult<Vec<LineFunction>> {
    repeat(0.., preceded(space1, function))
        .map(|x: Vec<Vec<LineFunction>>| x.into_iter().flatten().collect())
        .parse_next(input)
}

pub fn function(input: &mut &str) -> ModalResult<Vec<LineFunction>> {
    let base_function = alt((
        alt([
            "per_minute_to_per_second".value(StatFunction::DivideByRound(60.0, 1)),
            "locations_to_metres".value(StatFunction::DivideBy(10.0)),
            "multiplicative_damage_modifier".value(StatFunction::Plus(100.0)),
            "invert_chance".value(StatFunction::InvertChance),
            "negate_and_double".value(StatFunction::MultiplyBy(-2.0)),
            "negate".value(StatFunction::MultiplyBy(-1.0)),
            "double".value(StatFunction::MultiplyBy(2.0)),
            "canonical_stat".value(StatFunction::CanonicalStat),
            "passive_hash".value(StatFunction::PassiveHash),
            "display_indexable_support".value(StatFunction::Unsupported),
            "mod_value_to_item_class".value(StatFunction::Unsupported),
            "affliction_reward_type".value(StatFunction::Unsupported),
            "display_indexable_skill".value(StatFunction::Unsupported),
            "display_indexable_support".value(StatFunction::Unsupported),
            "mod_value_to_item_class".value(StatFunction::Unsupported),
            "old_leech_percent".value(StatFunction::Unsupported),
            "old_leech_permyriad".value(StatFunction::Unsupported),
            "tempest_mod_text".value(StatFunction::Unsupported),
            "tree_expansion_jewel_passive".value(StatFunction::Unsupported),
            "weapon_tree_unique_base_type_name".value(StatFunction::Unsupported),
            "divide_by_twenty_then_double".value(StatFunction::Unsupported),
            "divide_by_one_hundred_and_negate".value(StatFunction::Unsupported),
        ]),
        alt((
            preceded(alt(("multiply_by_", "times_")), value).map(StatFunction::MultiplyBy),
            preceded("divide_by_", value).map(StatFunction::DivideBy),
            preceded("plus_", value).map(StatFunction::Plus),
            terminated(value, "%_of_value").map(|x| StatFunction::MultiplyBy(x / 100.0)),
            "milliseconds_to_seconds".value(StatFunction::DivideByRound(1000.0, 0)),
            "deciseconds_to_seconds".value(StatFunction::DivideByRound(10.0, 2)),
        )),
    ));

    let functions = (
        base_function,
        opt((
            delimited('_', dec_uint, "dp").map(StatFunction::Precision),
            opt("_if_required".value(StatFunction::IfRequired)),
        )),
    )
        .map(|(f, p)| {
            let mut ret = vec![f];
            if let Some((p1, p2)) = p {
                ret.push(p1);
                if let Some(p2) = p2 {
                    ret.push(p2);
                }
            }
            ret
        });

    alt((
        separated_pair(functions, space1, dec_uint).map(|(stat_functions, idx): (Vec<StatFunction>, usize)| {
            stat_functions
                .into_iter()
                .map(|stat_function| LineFunction::StatFunction(stat_function, idx - 1))
                .collect()
        }),
        ("canonical_line" as &str).value(vec![LineFunction::CanonicalLine]),
        ("markup" as &str).value(vec![LineFunction::Markup]),
        preceded(("reminderstring" as &str, space1), identifier).map(|ident| vec![LineFunction::ReminderString(ident.to_string())]),
    ))
    .parse_next(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_description() {
        let input = r##"description
	1 heist_coins_from_monsters_+%
	2
		1|# "{0}% increased Rogue's Markers dropped by monsters"
		#|-1 "{0}% reduced Rogue's Markers dropped by monsters" negate 1
	lang "German"
	2
		1|# "{0}% erhöhte Menge fallen gelassener Signa der Abtrünnigen von Monstern"
		#|-1 "{0}% verringerte Menge fallen gelassener Signa der Abtrünnigen von Monstern" negate 1
	lang "Spanish"
	2
		1|# "Cantidad de monedas del renegado arrojadas por monstruos aumentada un {0}%"
		#|-1 "Cantidad de monedas del renegado arrojadas por monstruos reducida un {0}%" negate 1
	lang "Traditional Chinese"
	2
		1|# "增加 {0}% 怪物掉落的盜賊之印"
		#|-1 "減少 {0}% 怪物掉落的盜賊之印" negate 1
	lang "Russian"
	2
		1|# "{0}% увеличение количества выпадающих из монстров разбойничьих жетонов"
		#|-1 "{0}% уменьшение количества выпадающих из монстров разбойничьих жетонов" negate 1
	lang "Korean"
	2
		1|# "몬스터가 떨어뜨리는 도둑의 증표 {0}% 증가"
		#|-1 "몬스터가 떨어뜨리는 도둑의 증표 {0}% 감소" negate 1
	lang "Portuguese"
	2
		1|# "Insígnias do Renegado derrubada por monstros aumentadas em {0}%"
		#|-1 "Insígnias do Renegado derrubada por monstros reduzidas em {0}%" negate 1
	lang "French"
	2
		1|# "{0}% d'Augmentation du nombre de Pièces marquées cédées par les Monstres"
		#|-1 "{0}% de Réduction du nombre de Pièces marquées cédées par les Monstres" negate 1
	lang "Simplified Chinese"
	2
		1|# "怪物掉落赏金猎人印记的几率提高 {0}%"
		#|-1 "怪物掉落赏金猎人印记的几率降低 {0}%" negate 1
	lang "Thai"
	2
		1|# "เพิ่ม เหรียญกองโจร ที่ดรอปจากมอนสเตอร์ {0}%"
		#|-1 "ลด เหรียญกองโจร ที่ดรอปจากมอนสเตอร์ {0}%" negate 1
	lang "Japanese"
	2
		1|# "モンスターがドロップするローグマーカーが{0}%増加する"
		#|-1 "モンスターがドロップするローグマーカーが{0}%減少する" negate 1"##;
        let result = description.parse(input);
        assert!(result.is_ok(), "{}", result.unwrap_err());
    }
}
