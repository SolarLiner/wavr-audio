/*
 * Copyright (c) 2020 the Wavr Audio project.
 * This source file, as well as the binaries generated by it,
 * are licensed under MIT.
 */
//! # The Wavr Audio Engine
//!
//! Core structures for the audio engine. This crate implements the effects rack and monitoring
//! capabilities.

use wavr_audio_buffer::AudioBuffer;

use crate::{AudioContext, AudioContextState, Effect, Rack};

/// Structure holding the audio context and processing rack.
pub struct AudioEngine {
    context: AudioContext,
    rack: Rack,
}

impl AudioEngine {
    /// Creates a new audio engine with the given sample rate and channel count.
    pub fn new(sample_rate: u64, channel_count: u8) -> Self {
        let context = AudioContext::new(sample_rate, channel_count);
        Self {
            context,
            rack: Rack::new(),
        }
    }

    /// Fills the given audio buffer with audio data processed from the rack. The buffer's data is
    /// used as input into the rack.
    pub fn fill_interleaved(&mut self, input: &mut [f64]) {
        let mut buffer = AudioBuffer::new(self.context.channel_count as usize, input);
        self.fill_buffer(&mut buffer);
        buffer.move_into_interleaved(input);
    }

    /// Fills the `AudioBuffer` with audio data processed from the rack. The buffer's data is used
    /// as input into the rack.
    pub fn fill_buffer(&mut self, input: &mut AudioBuffer) {
        self.rack.process(&self.context, input);
        self.context.add_sample_cycle(input);
    }

    /// Returns a constant reference to the rack.
    pub fn get_rack(&self) -> &Rack {
        &self.rack
    }

    /// Returns a mutable reference to the rack.
    pub fn get_rack_mut(&mut self) -> &mut Rack {
        &mut self.rack
    }

    /// Returns a reference to the audio context.
    pub fn get_context(&self) -> &AudioContext {
        &self.context
    }

    /// Set the audio context state.
    pub fn set_context_state(&mut self, state: AudioContextState) {
        self.context.state = state;
    }
}
