mod wave;
use psimple::Simple;
use pulse::stream::Direction;

#[derive(Clone, Eq, PartialEq)]
enum MorseSymbol {
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

fn str_to_morse(s: &str) -> Vec<MorseSymbol> {
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

fn translate_morse(code: &[MorseSymbol]) -> String {
    code.iter()
        .map(|e| {
            String::from(match e {
                MorseSymbol::Dit => ".",
                MorseSymbol::Dah => "-",
                MorseSymbol::LetterSpace => " ",
                MorseSymbol::WordSpace => " / ",
            })
        })
        .collect()
}

#[allow(dead_code)]
enum Waveform {
    Sine,
    Square,
}

fn play_morse(
    code: &[MorseSymbol],
    waveform: Waveform,
    simple: &Simple,
    amp: f32,
    freq: f32,
    dit_dur: f32,
) {
    let dah_dur = dit_dur * 3.0;
    let letterspace_dur = dit_dur * 3.0;
    let wordspace_dur = dit_dur * 7.0;

    let wave_func = match waveform {
        Waveform::Sine => wave::write_sine,
        Waveform::Square => wave::write_square,
    };

    for e in code {
        match e {
            MorseSymbol::Dit => wave_func(simple, amp, freq, dit_dur),
            MorseSymbol::Dah => wave_func(simple, amp, freq, dah_dur),
            MorseSymbol::LetterSpace => wave::write_silence(simple, letterspace_dur),
            MorseSymbol::WordSpace => wave::write_silence(simple, wordspace_dur),
        }
        .unwrap_or_else(warn_pa_error!(err));
        if *e == MorseSymbol::Dit || *e == MorseSymbol::Dah {
            wave::write_silence(simple, dit_dur).unwrap_or_else(warn_pa_error!(err));
        }
    }

    simple.drain().unwrap_or_else(warn_pa_error!(err));
}

// TODO use clap for command args
fn main() {
    let msg = std::env::args().nth(1).unwrap();
    let code = str_to_morse(msg.as_str());
    println!("{}", translate_morse(&code));

    let simple = Simple::new(
        None,                // Use the default server
        "morse_server",      // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "morse",             // Description of our stream
        &wave::SPEC,         // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    )
    // translate error code
    .unwrap_or_else(panic_pa_error!(err));

    play_morse(&code, Waveform::Sine, &simple, 8192.0, 440.0, 0.1);
}
