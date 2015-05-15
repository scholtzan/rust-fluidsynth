# rust-fluidsynth

> Note: This project is still under development, not all functionality to use fluidsynth is available yet.

__FluidSynth__ bindings for Rust.

Bindings for FluidSynth, a software synthesizer based on the SoundFont 2 specifications, in Rust.

FluidSynth website [here](http://fluidsynth.elementsofsound.org/).

## Installation

To use `rust-fluidsynth` you must install FluidSynth on your computer and add this to `Cargo.toml`:

```toml
[dependencies.fluidsynth]
git = "https://github.com/scholtzan/rust-fluidsynth"
```

## Short example

```Rust
extern crate fluidsynth;

use fluidsynth::*;
use std::rand::{thread_rng, Rng};
use std::old_io::Timer;
use std::time::duration::Duration;

fn main() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);
    let mut adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload("/path/to/soundfont.sf2", 1);

    let interval = Duration::milliseconds(1000);
    let mut timer = Timer::new().unwrap();

    for x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = 60 + num;
        syn.noteon(0, key, 80);
        timer.sleep(interval);
        syn.noteoff(0, key);
    }
}
```

## License 

`rust-fluidsynth` uses the same license as FluidSynth: the GNU Lesser General Public License.



