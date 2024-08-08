#[derive(Clone, Eq, PartialEq)]
pub enum MorseSymbol {
    Dit,
    Dah,
    LetterSpace,
    WordSpace,
}

// https://www.itu.int/dms_pubrec/itu-r/rec/m/R-REC-M.1677-1-200910-I!!PDF-E.pdf
fn char_to_morse(c: char) -> Option<Vec<MorseSymbol>> {
    let code_str = match c {
        // letters
        'a' => ".-",
        'b' => "-...",
        'c' => "-.-.",
        'd' => "-..",
        'e' => ".",
        'é' => "..-..",
        'f' => "..-.",
        'g' => "--.",
        'h' => "....",
        'i' => "..",
        'j' => ".---",
        'k' => "-.-",
        'l' => ".-..",
        'm' => "--",
        'n' => "-.",
        'o' => "---",
        'p' => ".--.",
        'q' => "--.-",
        'r' => ".-.",
        's' => "...",
        't' => "-",
        'u' => "..-",
        'v' => "...-",
        'w' => ".--",
        'x' => "-..-",
        'y' => "-.--",
        'z' => "--..",

        // figures
        '1' => ".----",
        '2' => "..---",
        '3' => "...--",
        '4' => "....-",
        '5' => ".....",
        '6' => "-....",
        '7' => "--...",
        '8' => "---..",
        '9' => "----.",
        '0' => "-----",

        '.' => ".-.-.-",
        ',' => "--..--",
        ':' => "---...",
        '?' => "..--..",
        '\'' => ".----.",
        '-' => "-....-",
        '/' => "-..-.",
        '(' => "-.--.",
        ')' => "-.--.-",
        '\"' => ".-..-.",
        '=' => "-...-",
        '+' => ".-.-.",
        '×' => "-..-",
        '@' => ".--.-.",

        _ => return None,
    };

    let code = code_str
        .chars()
        .map(|c| match c {
            '.' => MorseSymbol::Dit,
            '-' => MorseSymbol::Dah,
            _ => panic!("Bad character to Morse code mapping"),
        })
        .collect();

    Some(code)
}

pub fn str_to_morse(s: &str) -> Vec<MorseSymbol> {
    let mut err = false;
    let mut i = 0;
    let ret = s
        .to_lowercase()
        .trim()
        .split(' ')
        .filter(|w| !w.is_empty())
        .map(|w| {
            w
            .chars()
            .filter_map(|c| {
                let m = char_to_morse(c);
                if m.is_none() {
                    match c {
                        '%' => {
                            println!("[WARNING] Percent symbol {c} at position {i}. Rewrite using -0/0")
                        }
                        '‰' => println!(
                            "[WARNING] Percent symbol {c} at position {i}. Rewrite using -0/00"
                        ),
                        _ => println!("[WARNING] Invalid character {c} at position {i}"),
                    }
                    err = true;
                }
                i += 1;
                m
            })
            .collect::<Vec<Vec<MorseSymbol>>>()
            .join(&MorseSymbol::LetterSpace)
        })
        .collect::<Vec<Vec<MorseSymbol>>>()
        .join(&MorseSymbol::WordSpace);
    if err {
        println!("Ignoring invalid characters");
    };

    ret
}

pub fn translate_morsesymbol(sym: &MorseSymbol) -> String {
    String::from(match sym {
        MorseSymbol::Dit => ".",
        MorseSymbol::Dah => "-",
        MorseSymbol::LetterSpace => " ",
        MorseSymbol::WordSpace => " / ",
    })
}

pub fn translate_morse(code: &[MorseSymbol]) -> String {
    code.iter().map(|e| translate_morsesymbol(e)).collect()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_str_to_morse() {
        assert_eq!(
            translate_morse(&str_to_morse("Hello World")),
            ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
        );
        assert_eq!(
            translate_morse(&str_to_morse("Hello World!")),
            ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
        );
        assert_eq!(
            translate_morse(&str_to_morse("H%ell$o Wor^ld!")),
            ".... . .-.. .-.. --- / .-- --- .-. .-.. -.."
        );
        assert_eq!(translate_morse(&str_to_morse("É")), "..-..");
        assert_eq!(translate_morse(&str_to_morse("\'\"")), ".----. .-..-.");
    }
}
