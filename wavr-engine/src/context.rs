use std::time::Duration;

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum AudioContextState {
    Paused,
    Playing,
    Offline,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AudioContext {
    pub sample_rate: u64,
    pub channel_count: u8,
    pub buffer_size: usize,
    pub current_sample: usize,
    pub state: AudioContextState,
}

impl AudioContext {
    pub fn new(sample_rate: u64, channel_count: u8, buffer_size: usize) -> Self {
        Self {
            sample_rate,
            channel_count,
            buffer_size,
            current_sample: 0,
            state: AudioContextState::Paused,
        }
    }

    pub fn interleaved_buffer_length(&self) -> usize {
        self.channel_count as usize * self.buffer_size as usize
    }

    pub fn timestamp(&self) -> Duration {
        let seconds = self.current_sample as f64 / self.sample_rate as f64;
        Duration::from_secs_f64(seconds)
    }

    pub fn timestamp_offset(&self, offset: usize) -> Duration {
        let seconds = (self.current_sample + offset) as f64 / self.sample_rate as f64;
        Duration::from_secs_f64(seconds)
    }

    pub fn add_sample_cycle(&mut self) {
        self.current_sample += self.buffer_size;
    }

    pub fn is_playing(&self) -> bool {
        match self.state {
            AudioContextState::Playing | AudioContextState::Offline => true,
            _ => false,
        }
    }
}
