extern crate fluidsynth;
use fluidsynth::*;
use std::rand::{thread_rng, Rng};
use std::old_io::Timer;
use std::old_io::timer;
use std::time::duration::Duration;

fn main() {
    //test3();
}

/*fn test3() {
    let callback = |event| -> i32 {
        let mut me = midi::MidiEvent::wrap(event);
        println!("Event type:{}", me.get_type());
        me.get_type()
    };

    let mut settings = settings::Settings::new();
    settings.setstr("audio.driver", "jack");
    let mut mdriver = midi::MidiDriver::new(&mut settings, callback);

    let interval = Duration::milliseconds(1000);
    let mut timer = Timer::new().unwrap();


    for x in 0..12 {
        timer::sleep(interval);
    }

    mdriver.delete();
}

fn test2() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);

    let callback = |event| -> i32 {
        println!("yep");
        0
    };

    let mut router = midi::MidiRouter::new(&mut settings, callback, &mut syn);
    router.clear_rules();

    let mut rule = midi::MidiRouterRule::new();
    rule.set_chan(0, 15, 0.0, 5);
    rule.set_param1(60, 127, 1.0, 0);
    router.add_rule(&mut rule, midi::MidiRouterRuleType::NOTE);

    let mut adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    let sfont_id = syn.sfload("/Users/anna/mydata/drumming/drums.zip Folder/phattkit.sf2", 1);

    let interval = Duration::milliseconds(1000);
    let mut timer = Timer::new().unwrap();


    for x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = 60 + num;
        syn.noteon(0, key, 80);

        timer::sleep(interval);

        syn.noteoff(0, key);
    }


    router.delete();    
    syn.delete();
    settings.delete();
}

fn test1() {
    let mut settings = settings::Settings::new();
    let mut syn = synth::Synth::new(&mut settings);
    let mut adriver = audio::AudioDriver::new(&mut settings, &mut syn);
    let sfont_id = syn.sfload("/tmp/phattkit.sf2", 1);

    let interval = Duration::milliseconds(1000);
    let mut timer = Timer::new().unwrap();

    for x in 0..12 {
        let num: i32 = thread_rng().gen_range(0, 12);
        let key = 60 + num;
        syn.noteon(0, key, 80);

        timer::sleep(interval);

        syn.noteoff(0, key);
    }

    adriver.delete();
    syn.delete();
    settings.delete();
}*/

