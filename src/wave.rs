use std::f32::consts::PI;

use psimple::Simple;
use pulse::error::PAErr;
use pulse::sample::{Format, Spec};

// pulse audio documentation is kind of bad...
// channel data uses interleaved format for > 1 channel
//
// explicitly calling drain() does not seem necessary
// since write() seems to already block while sound is played
//
// flush() will only clear out the buffer most recently written to
//
// so it's not that multiple writes accumulate in a buffer and then are drained
// (I guess that's why it's called a streamed)

pub const SPEC: Spec = Spec {
    format: Format::S16NE,
    channels: 1,
    rate: 44100,
};
const BYTES_PER_SAMPLE: usize = 2;

const BUFSIZE: usize = 1024;

fn write_wave<F>(simple: &Simple, dur: f32, wave_fn: F) -> Result<(), PAErr>
where
    F: Fn(usize) -> i16,
{
    const {
        assert!(matches!(SPEC.format, Format::S16NE));
        assert!(BYTES_PER_SAMPLE == 2);
        assert!(SPEC.channels == 1);
        assert!(BUFSIZE % 2 == 0);
    }
    // (samples / sec) * (bytes / sample) = bytes / sec
    let num_bytes = ((SPEC.rate as f32) * (BYTES_PER_SAMPLE as f32) * dur).round() as usize;
    let mut buf = [0u8; BUFSIZE];

    let mut i = 0;

    'outer: loop {
        for v in buf.chunks_mut(BYTES_PER_SAMPLE) {
            if i * BYTES_PER_SAMPLE >= num_bytes {
                break 'outer;
            }
            v.copy_from_slice(&wave_fn(i).to_ne_bytes());
            i += 1;
        }
        simple.write(&buf)?;
    }

    let rem = (i * BYTES_PER_SAMPLE) % BUFSIZE;
    simple.write(&buf[..rem])
}

// freq in Hz and dur in seconds
pub fn write_sine(simple: &Simple, amp: f32, freq: f32, dur: f32) -> Result<(), PAErr> {
    // (1 / sec) / (samples / sec) = 1 / samples
    let sample_freq = freq / (SPEC.rate as f32);

    write_wave(simple, dur, |i| {
        (amp * ((i as f32) * 2.0 * PI * sample_freq).sin()).round() as i16
    })
}

pub fn write_square(simple: &Simple, amp: f32, freq: f32, dur: f32) -> Result<(), PAErr> {
    // (1 / sec) / (samples / sec) = 1 / samples
    let sample_freq = freq / (SPEC.rate as f32);

    write_wave(simple, dur, |i| {
        (amp * ((i as f32) * 2.0 * PI * sample_freq).sin().signum()).round() as i16
    })
}

pub fn write_silence(simple: &Simple, dur: f32) -> Result<(), PAErr> {
    write_wave(simple, dur, |_i| 0)
}

// make this a macro so the return type can be dynamically inferred
#[macro_export]
macro_rules! panic_pa_error {
    ($unused:ident) => {
        //|err| panic!("{}", err.to_string().expect("PAErr to_string failed"))
        |err| panic!("[ERROR] PulseAudio failed: {}", err)
    };
}

#[macro_export]
macro_rules! warn_pa_error {
    ($unused:ident) => {
        |err| eprintln!("[WARNING] PulseAudio failed: {}", err)
    };
}
