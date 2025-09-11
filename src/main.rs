use ib_matcher::{
    matcher::{IbMatcher, PinyinMatchConfig},
    pinyin::{PinyinNotation},
};
use std::env;
use std::io::{BufRead, BufReader};

fn is_pure_english_path(s: &str) -> bool {
    // Consider a path "pure English" if every character is within a conservative
    // ASCII set that bash already handles well: letters, digits, '_', '-', '.', '/', '~'.
    // We also ignore trailing newlines/spaces (already trimmed).
    // Require at least one ASCII alphabetic letter so an empty string or just symbols
    // doesn't get suppressed accidentally.
    let mut has_alpha = false;
    for ch in s.chars() {
        if ch.is_ascii_alphabetic() {
            has_alpha = true;
            continue;
        }
        if ch.is_ascii_digit() || matches!(ch, '_' | '-' | '.' | '/' | '~') {
            continue;
        }
        // Any other (non ASCII or other punctuation) means it's not pure English.
        return false;
    }
    has_alpha
}

fn parse_pinyin_notation_env() -> PinyinNotation {
    let env_val = env::var("PINYIN_COMP_MODE").unwrap_or_default();
    let mut notation = PinyinNotation::empty();
    let mut shuangpin = Option::<PinyinNotation>::None;

    for mode in env_val.split(',') {
        let mode = mode.trim();
        match mode {
            "Quanpin" => {
                notation |= PinyinNotation::Ascii;
            }
            "ShuangpinAbc" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterAbc);
            }
            "ShuangpinJiajia" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterJiajia);
            }
            "ShuangpinMicrosoft" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterMicrosoft);
            }
            "ShuangpinThunisoft" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterThunisoft);
            }
            "ShuangpinXiaohe" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterXiaohe);
            }
            "ShuangpinZrm" => {
                shuangpin.get_or_insert(PinyinNotation::DiletterZrm);
            }
            _ => {}
        }
    }

    notation |= shuangpin.unwrap_or(PinyinNotation::empty());

    if notation.is_empty() {
        notation = PinyinNotation::Ascii;
    }

    if notation == PinyinNotation::Ascii {
        notation |= PinyinNotation::AsciiFirstLetter;
    }

    notation
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Print usage
    if args.len() < 2 {
        eprintln!("Usage: {} <pinyin>", args[0]);
        std::process::exit(1);
    }

    let input: &str = &args[1];
    let notation = parse_pinyin_notation_env();
    let pinyin_config = PinyinMatchConfig::builder(notation).build();

    let matcher = IbMatcher::builder(input)
        .starts_with(true)
        .pinyin(pinyin_config)
        .build();

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    for line_result in reader.lines() {
        let candidate = line_result.unwrap().trim_end().to_string();
        // Ignore Pure English Path
        if is_pure_english_path(&candidate) {
            continue;
        }
        if matcher.is_match(candidate.as_str()) {
            println!("{}", candidate);
        }
    }
}
