use crate::{AudioContext, Effect, Rack, AudioContextState};

pub struct AudioEngine {
    context: AudioContext,
    rack: Rack,
    inner_buffer: Vec<f64>,
}

impl AudioEngine {
    pub fn new(sample_rate: u64, channel_count: u8, buffer_size: usize) -> Self {
        let context = AudioContext::new(sample_rate, channel_count, buffer_size);
        Self {
            context,
            rack: Rack::new(),
            inner_buffer: Vec::with_capacity(context.interleaved_buffer_length()),
        }
    }

    pub fn generate(&mut self) -> Vec<f64> {
        let mut buffer = vec![0.0; self.context.interleaved_buffer_length()];
        self.rack.process(&self.context, &mut buffer);
        self.context.add_sample_cycle();
        buffer
    }

    pub fn fill(&mut self, buffer: &mut [f64]) {
        self.context.buffer_size = buffer.len() / self.context.channel_count as usize;

        self.rack.process(&self.context, buffer);
        self.context.add_sample_cycle();
    }

    pub fn get_buffer(&self) -> &[f64] {
        &self.inner_buffer
    }

    pub fn get_rack(&self) -> &Rack {
        &self.rack
    }

    pub fn get_rack_mut(&mut self) -> &mut Rack {
        &mut self.rack
    }

    pub fn get_context(&self) -> &AudioContext {
        &self.context
    }

    pub fn set_buffer_size(&mut self, buffer_size: usize) {
        self.context.buffer_size = buffer_size;
        self.inner_buffer = Vec::with_capacity(self.context.interleaved_buffer_length());
    }
    
    pub fn set_context_state(&mut self, state: AudioContextState) {
        self.context.state = state;
    }
}

fn fill<T: Copy>(vec: &mut Vec<T>, value: T) {
    vec.iter_mut().for_each(|v| *v = value);
}
