mod morse;
use morse::*;
mod wave;
use clap::{Parser, ValueEnum};
use psimple::Simple;
use pulse::stream::Direction;
use std::io::{stdout, Write};
use wave::*;

#[derive(Clone, ValueEnum)]
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
        print!("{}", translate_morsesymbol(e));
        // if stdout isn't flushed explicitly then pulseaudio
        // buffers the output until the program ends
        stdout()
            .flush()
            .unwrap_or_else(|err| eprintln!("[WARNING] stdout flush failed: {}", err));
        match e {
            MorseSymbol::Dit => wave_func(simple, amp, freq, dit_dur),
            MorseSymbol::Dah => wave_func(simple, amp, freq, dah_dur),
            MorseSymbol::LetterSpace => write_silence(simple, letterspace_dur),
            MorseSymbol::WordSpace => write_silence(simple, wordspace_dur),
        }
        .unwrap_or_else(warn_pa_error!(err));
        if *e == MorseSymbol::Dit || *e == MorseSymbol::Dah {
            wave::write_silence(simple, dit_dur).unwrap_or_else(warn_pa_error!(err));
        }
    }
    println!();
    simple.drain().unwrap_or_else(warn_pa_error!(err));
}

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    /// Message to be played as Morse code
    message: String,

    /// Duration of a dit (ms)
    #[arg(short, long)]
    #[arg(default_value_t = 100.0)]
    #[arg(value_parser = validate_duration)]
    #[arg(allow_negative_numbers = true)]
    duration: f32,

    /// Frequency of waveform (Hz)
    #[arg(short, long)]
    #[arg(default_value_t = 440.0)]
    #[arg(allow_negative_numbers = true)]
    frequency: f32,

    /// Amplitude of waveform
    #[arg(short, long)]
    #[arg(default_value_t = 8192.0)]
    #[arg(allow_negative_numbers = true)]
    amplitude: f32,

    /// Shape of waveform
    #[arg(short, long)]
    #[arg(value_enum)]
    #[arg(default_value_t = Waveform::Sine)]
    waveform: Waveform,
}

fn validate_duration(s: &str) -> Result<f32, String> {
    let dur: f32 = s
        .parse()
        .map_err(|_| String::from("invalid float literal"))?;
    if dur >= 0.0 {
        Ok(dur)
    } else {
        Err(String::from("invalid negative duration"))
    }
}

// TODO: return error from main instead of panicking, using map err question mark
// TODO: add saw and triangle waves
fn main() {
    let cli = Cli::parse();
    let msg = cli.message;
    let code = str_to_morse(msg.as_str());
    println!("{}", translate_morse(&code));

    let simple = Simple::new(
        None,                // Use the default server
        "morse_server",      // Our application’s name
        Direction::Playback, // We want a playback stream
        None,                // Use the default device
        "morse",             // Description of our stream
        &SPEC,               // Our sample format
        None,                // Use default channel map
        None,                // Use default buffering attributes
    )
    // translate error code
    .unwrap_or_else(panic_pa_error!(err));

    play_morse(
        &code,
        cli.waveform,
        &simple,
        cli.amplitude,
        cli.frequency,
        cli.duration / 1000.0,
    );
}
