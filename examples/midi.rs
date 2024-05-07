//
// Copyright 2023 Richard I. Christopher, NeoTec Digital. All Rights Reserved.
//


use std::{ops::{Deref, DerefMut}, sync::{Arc, Mutex}};
use rodio::{OutputStream, Source};
use tokio::time::{self, Duration};
use audiotheorem::runtime::{Sequence, WaveTableOsc};
use audiotheorem::types::Tuning;


async fn midi_loop(sequence: Arc<Mutex<Sequence>>, oscillator: Arc<Mutex<WaveTableOsc>>) {
        
    // midir wrapper that is used to convert out midi into a sequence of tones
    // that can then be used to generate a sequence of chords, scales, pitchclasses, 
    // and pitchgroups, which is turn become the foundation for finding a root note of 
    // a mode, in a key, and then mapping that to a scale, and then mapping that to a
    // pitchgroup, and then mapping that to a wavetable, and then mapping that to a
    // synthesizer, and then mapping that to a soundfont, and then mapping that to a
    // midi output, and then mapping that to a midi input, and then mapping that to a .. turtles all the way down ..
                        // .. and up (oscillators, filters, envelopes, effects, etc.)
    audiotheorem::runtime::Events::read_midi(move |index, velocity| {
        // This acts as our buffer handle for the midi input - which we can then user for gfx/ui
        let mut seq_snd = sequence.lock().unwrap().deref_mut().clone();
        let mut binding = oscillator.lock().unwrap();
        let osc_write = binding.deref_mut();
        
        // this maintains state for a given set of tones and their dynamics => midi state
        seq_snd.process_input(index, velocity);
//        seq_snd.print_state();

        let (_stream, _handle) = OutputStream::try_default().unwrap();
        let osc_read = oscillator.lock().unwrap().deref().clone();
        let _res = _handle.play_raw(osc_read.clone().convert_samples());
    

        // this is where we go from a sequence of midi events to a sequence of tones -> pitches
        let tone = Sequence::get_tone(index, velocity).unwrap();
        osc_write.set_frequency(tone.pitch().frequency(Tuning::A4_440Hz));

        *sequence.lock().unwrap() = seq_snd;
    });
}

async fn graphics_loop(sequence: Arc<Mutex<Sequence>>) {
    loop {
        let size = sequence.lock().unwrap().get_size();

        if size > 0 {
            sequence.lock().unwrap().print_state();
            println!("Sequence Size: {}", size);
        } else {
            print!("\x1B[2J\x1B[1;1H");
            println!("=====================");
            println!("!!! Audio Theorem !!!");
            println!("=====================\n");

            println!("Sequence Size: {}", size);
        }

        time::sleep(Duration::from_millis(10)).await;
    }
}

async fn playback_loop(oscillator: Arc<Mutex<WaveTableOsc>>) {
    
    loop {
        println!("Playing Oscillator");
    }
}

// TODO: map out all the scales and chords
//  - then map out all the pitch groups mapped to the scales based on number of pitchgroups
//  - then map out those statically as a lookup for a given cursor position i.e. 'root' note
//  - then map out the root note and mode in a 'turing complete' way
#[tokio::main]
async fn main() {
    let wave_table_size = 1024;
    let sample_rate = 44100;
    let _buffer_size = 1024;

    let mut wave_table: Vec<f32> = Vec::with_capacity(wave_table_size);

    for i in 0..wave_table_size {
        wave_table.push((i as f32 / wave_table_size as f32 * 2.0 * std::f32::consts::PI).sin());
    }

    let sequence = Arc::new(Mutex::new(Sequence::new()));
    let oscillator = Arc::new(Mutex::new(WaveTableOsc::new(sample_rate, wave_table)));

    let midi_task = midi_loop(sequence.clone(), oscillator.clone());
    let graphics_task = graphics_loop(sequence.clone());
    let playback_task = playback_loop(oscillator.clone());

    tokio::join!(midi_task, playback_task, graphics_task);

    println!("Audio Theorem Complete!");
}
