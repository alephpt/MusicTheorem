// Disposition is a Music Theory concept that is used to determine the color of the note (later might add a fractional degree or mixing of colors as a vector) << -- we are incorporating this through pitchgroups and scales
// The intent and purpose of this system, alongside the sequence Data, is to interface the pitchgroups and scales with the audio and graphics systems to help correlate
// the color of the notes with the sound and the key of the music. This is a proprietary system that is being developed for the Ancillary via Big Stick Studios, and is not to be used without permission.

// Copyright (c) 2024 by Richard I Christopher, Big Stick Studio - All Rights Reserved, Proprietary License - The NEXUS Project

/*  Proposed Questions 
    - can we use audio structure as an indicator for an AI system to learn to analyze speech patterns, emotion, phonetics, and intent?
    - Is there a correlation to audio and emotion?
    - Can we prove that life comes from Oxygon and Hydrogen when met with Carbon and Nitrogen?
    - Will I make my first million before I'm 35?
*/

use std::collections::HashSet;
use super::{Chord, PitchGroupKernel, Tonic};
use crate::types::{Tone, Interval, Scale};

pub struct Subsequence {
    pub tones: HashSet<Tonic>,          // These are initially the tones being played, and we add the tones from the pitchgroupkernel across the entire bounds
    pub chords: Vec<Chord>,             // We need to split this further into n_inversions and n_shapes
    pub scales: Vec<Scale>,             // We can use these to determine "gravity"
    pub kernel: PitchGroupKernel, 
    pub upper_bound: u8,                // This is the upper bound of the dynamic range for a set of keys + 7 but we may need to make this part of a filter (proprietary NEXUS)
    pub lower_bound: u8,                // This is the lower bound of the dynamic range for a set of keys - 7
}

impl Subsequence {
    pub fn new() -> Subsequence { 
        Subsequence { 
            tones: HashSet::new(), 
            chords: Vec::new(), 
            scales: Vec::new(), 
            kernel: PitchGroupKernel::new(Vec::new()),
            upper_bound: 144,       // These need to be swapped for a filter type
            lower_bound: -1
        } 
    }

    // We need logic for the following:
    // add note
    // remove note
    // update kernel

    pub fn upper_bound(&self) -> u8 { self.tones.iter().map(|t| t.index()).max().unwrap_or(144) }
    pub fn lower_bound(&self) -> u8 { self.tones.iter().map(|t| t.index()).min().unwrap_or(-1)}

    // We calculate +7 and -7 from the current upper and lower bounds of the tones or 
    // max of 143 and min of 0
    pub fn calculate_bounds(&mut self) {
        self.upper_bound = clamp(self.upper_bound() + 7, 0, 143); // 144 is the max index
        self.lower_bound = clamp(self.lower_bound() - 7, 0, 143); // 0 is the min index
    }

    pub fn within_bounds(&self, index: u8) -> bool 
        {
            self.upper_bound >= index && self.lower_bound <= index
        }

    // Returns the gap between the upper and lower bounds (should be less than 28 for a scale and 12 (or 24) for a chord) - and smaller for tetrachordal voicings
    pub fn limits(&self) -> u8 { self.upper_bound() - self.lower_bound() }

    fn construct_chords(&mut self, tones: HashSet<Tone>) 
        { 

            for root in tones.iter().map(|d| d.tone) 
                {
                    let root_note = tone.note();
                    let mut chord_shape = Vec::new();

                    for dis in disposition.iter() 
                        {
                            if root != dis.tone 
                                { // This is a mess that needs to be agnostic to pitchgroups (comment generated by the AI)
                                    /*chord_shape.push(
                                        Chord { root: dis.tone, intervals: Interval::from_tones(root, dis.tone) }   // This finds a given interval between two notes
                                    );*/
                                }
                        }

 //                   self.chords.push(Chord{ root: root_note, intervals: chord_shape })
                }
        }

    fn find_scales(&mut self) 
        { // This is a mess that needs to be agnostic to pitchgroups
            self.scales.clear();
            if self.tones.len() == 0 { return; }

            let  scales = Vec::new();
            
            // we need to find all the scales that contain the given intervals
            // we are going to iterate over all the scales and check if the intervals are present

            self.scales = scales;
        }

    pub fn play_note(&mut self, index: u8, velocity: u8)
        {
            // We need to check if the note is within the bounds of the sequence
            if self.tones.len() == 0 
                { 
                    self.tones.insert(Tone::from_iv(index, velocity)); 
                    self.calculate_bounds();
                }

            self.calculate_bounds();

            // so, we need to check if the index is already in the vector
            if let Some(tone) = self.tones.get(&index)
                { 
                    // if it is, we need to update the velocity if it is greater, and exit.
                    if tone.velocity() < velocity 
                        { 
                            self.tones.remove(&index); 
                            self.tones.insert(Tonic::from_ivh(index, velocity, 0));
                        }
                }
                
            self.tones.insert(Tonic::from_ivh(index, velocity, 0));
        }

    pub fn calculate_harmonies(&mut self) 
        {
            
        }
}