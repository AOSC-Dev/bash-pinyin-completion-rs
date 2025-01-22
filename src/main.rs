use ib_pinyin::{matcher::PinyinMatcher, pinyin::PinyinNotation};

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

    let mut results: Vec<String> = Vec::new();
    // Read current location
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
                let path = entry.path();
                if let Some(file_name) = path.file_name() {
                    if let Some(file_name_str) = file_name.to_str() {
                        if matcher.is_match(file_name_str) && han_re.is_match(file_name_str) {
                            results.push(file_name_str.to_string());
                        }
                    }
                }
            }
        }
        for result in results {
            println!("{}", result);
        }
    }
