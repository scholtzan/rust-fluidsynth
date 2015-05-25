# rust-fluidsynth

> Note: This project is still under development, not all functionality to use fluidsynth is available yet.

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
extern create rand;
extern crate time;

use fluidsynth::*;
use rand::{thread_rng};
use time::duration::Duration;
use std::thread;

fn main() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);
    let mut adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    syn.sfload("/path/to/soundfont.sf2", 1);

    let interval = Duration::milliseconds(1000);

    for x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = 60 + num;
        syn.noteon(0, key, 80);
        thread::sleep_ms(interval);
        syn.noteoff(0, key);
    }
}
```

## License 

`rust-fluidsynth` uses the same license as FluidSynth: the GNU Lesser General Public License.



