# rust-fluidsynth

[![Build status (master)](https://api.travis-ci.org/scholtzan/rust-fluidsynth.svg?branch=master)](https://travis-ci.org/scholtzan/rust-fluidsynth)

> Note: This project is currently not under active development. Not all functionality to use fluidsynth is available yet. Nevertheless, feel free to fork and and send pull requests with additional functionality or bug fixes.

__FluidSynth__ bindings for Rust.

Bindings for FluidSynth, a software synthesizer based on the SoundFont 2 specifications, in Rust.

FluidSynth website [here](http://fluidsynth.elementsofsound.org/).
A documentation of the FluidSynth API is available [here](http://fluidsynth.sourceforge.net/api/index.html).

## Installation

To use `rust-fluidsynth` you must install FluidSynth on your computer and add this to `Cargo.toml`:

```toml
[dependencies.fluidsynth]
git = "https://github.com/scholtzan/rust-fluidsynth"
```

## Short example

```Rust
extern crate fluidsynth;
extern crate rand;
extern crate time;

use fluidsynth::*;
use rand::{thread_rng, Rng};
use std::thread;

fn main() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);
    let _adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload("/path/to/soundfont.sf2", 1);

    let interval = Duration::milliseconds(1000);

    for _x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = 60 + num;
        syn.noteon(0, key, 80);
        thread::sleep_ms(1000);
        syn.noteoff(0, key);
    }
}
```
