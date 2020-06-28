use byteorder::{ByteOrder, LittleEndian};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::mem::size_of;
use wavr_engine::{AudioContext, AudioEngine, Effect};

struct SineWaveGenerator {
    freq: f64,
    amplitude: f64,
}

struct Saturator {
    power: f64,
}

impl Effect for SineWaveGenerator {
    fn process(&mut self, context: &AudioContext, data: &mut [f64]) {
        if !context.is_playing() {
            return;
        }

        for s in 0..context.buffer_size {
            for i in 0..context.channel_count {
                let idx = s * context.channel_count as usize + i as usize;
                let phase =
                    context.timestamp_offset(s).as_secs_f64() % (2.0 * std::f64::consts::PI);
                data[idx] = (self.freq * phase).sin() * self.amplitude;
            }
        }
    }
}

impl Effect for Saturator {
    fn process(&mut self, _context: &AudioContext, data: &mut [f64]) {
        for s in data {
            *s = (*s * self.power).tanh();
        }
    }
}

fn main() {
    let mut engine = AudioEngine::new(48000, 2, 512);
    {
        let rack = engine.get_rack_mut();
        rack.push_effect(SineWaveGenerator {
            freq: 80.0,
            amplitude: 1.0,
        });
        rack.push_effect(Saturator { power: 2.0 });
    }
    let spec = hound::WavSpec {
        channels: 2,
        sample_rate: 48000,
        bits_per_sample: 32,
        sample_format: hound::SampleFormat::Float,
    };
    let mut writer = hound::WavWriter::create("track.wav", spec).unwrap();
    let chunk_count = (48000f64 / 512.0).ceil() as usize;
    engine.set_context_state(wavr_engine::AudioContextState::Offline);
    for _ in 0..chunk_count {
        for sample in engine.generate() {
            writer.write_sample(sample as f32).unwrap();
        }
    }
}
