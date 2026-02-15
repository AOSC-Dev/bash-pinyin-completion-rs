use ib_matcher::{
    matcher::{IbMatcher, PinyinMatchConfig, RomajiMatchConfig},
    pinyin::PinyinNotation,
    romaji::HepburnRomanizer,
};
use std::env;
use std::io::{BufRead, BufReader};
use std::path::PathBuf;

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

/// Returns whether romaji is enabled (single full mode).
fn parse_romaji_enabled() -> bool {
    let env_val = env::var("PINYIN_COMP_MODE").unwrap_or_default();
    for mode in env_val.split(',') {
        if mode.trim() == "Romaji" { return true }
    }
    false
}

/// Get the cache directory path, respecting XDG_CACHE_HOME
fn get_cache_dir() -> Option<PathBuf> {
    // Check for custom cache path first
    if let Ok(cache_path) = env::var("PINYIN_COMP_CACHE_DIR") {
        return Some(PathBuf::from(cache_path));
    }
    
    // Use XDG_CACHE_HOME or default to ~/.cache
    let cache_home = env::var("XDG_CACHE_HOME")
        .map(PathBuf::from)
        .or_else(|_| env::var("HOME").map(|h| PathBuf::from(h).join(".cache")))
        .ok()?;
    
    Some(cache_home.join("bash-pinyin-completion"))
}

/// Get the cache file path for romaji
fn get_cache_path() -> Option<PathBuf> {
    let cache_dir = get_cache_dir()?;
    Some(cache_dir.join("romanizer.cache"))
}

/// Create or load a HepburnRomanizer, using cache when available
fn get_or_create_romanizer() -> HepburnRomanizer {
    let (kana, kanji, word) = (true, true, true);
    
    // Use upstream builder caching API for elegant cache handling
    if let Some(cache_path) = get_cache_path() {
        HepburnRomanizer::builder()
            .kana(kana)
            .kanji(kanji)
            .word(word)
            .build_cached(cache_path)
    } else {
        // Fallback without cache
        HepburnRomanizer::builder()
            .kana(kana)
            .kanji(kanji)
            .word(word)
            .build()
    }
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

    // Build romaji config based on mode
    // - disabled if not requested
    // - enabled: full mode with word dictionary (cached for startup speed)
    let romaji_enabled = parse_romaji_enabled();
    let romanizer = romaji_enabled.then(get_or_create_romanizer);

    let romaji_config = romanizer.as_ref().map(|r| {
        RomajiMatchConfig::builder().romanizer(r).build()
    });

    let matcher = if let Some(ref romaji) = romaji_config {
        IbMatcher::builder(input)
            .starts_with(true)
            .pinyin(pinyin_config)
            .romaji(romaji.shallow_clone())
            .build()
    } else {
        IbMatcher::builder(input)
            .starts_with(true)
            .pinyin(pinyin_config)
            .build()
    };

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    for line_result in reader.lines() {
        let candidate = match line_result {
            Ok(line) => line.trim_end().to_string(),
            Err(_) => {
                continue;
            }
        };
        // Ignore Pure English Path
        if is_pure_english_path(&candidate) {
            continue;
        }
        if matcher.is_match(candidate.as_str()) {
            println!("{}", candidate);
        }
    }
}
