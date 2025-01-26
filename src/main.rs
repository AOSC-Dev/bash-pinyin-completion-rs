use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};
use std::io::{BufRead, BufReader};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    // Print usage
    if args.len() < 2 {
        eprintln!("Usage: {} <pinyin>", args[0]);
        std::process::exit(1);
    }

    let input: &str = &args[1];
    let matcher = PinyinMatcher::builder(input)
        .pinyin_notations(PinyinNotation::Ascii | PinyinNotation::AsciiFirstLetter)
        .build();

    let han_re = regex::Regex::new(r"\p{Han}").unwrap();

    let stdin = std::io::stdin();
    let reader = BufReader::new(stdin.lock());
    for line_result in reader.lines() {
            let candidate = line_result.unwrap().trim_end().to_string();
            if han_re.is_match(&candidate) && matcher.is_match(&candidate) {
                println!("{}", candidate);
            }
        }
    }
