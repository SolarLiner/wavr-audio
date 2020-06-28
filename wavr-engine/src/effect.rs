use crate::context::AudioContext;

pub trait Effect {
    fn process(&mut self, context: &AudioContext, data: &mut [f64]);
}
