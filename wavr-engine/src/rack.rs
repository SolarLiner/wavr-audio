use crate::context::AudioContext;
use crate::effect::Effect;
use std::collections::LinkedList;
use std::ops::Deref;

pub struct RackEffect {
    effect: Box<dyn Effect>,
    enabled: bool,
}

pub struct Rack {
    effects: LinkedList<RackEffect>,
}

impl RackEffect {
    pub fn new<E: 'static + Effect>(effect: E) -> Self {
        Self {
            effect: Box::new(effect),
            enabled: true,
        }
    }

    pub fn enabled(&self) -> bool {
        self.enabled
    }

    pub fn disable(&mut self) {
        self.enabled = false;
    }

    pub fn enable(&mut self) {
        self.enabled = true;
    }

    pub fn toggle(&mut self) {
        self.enabled = !self.enabled;
    }
}

impl Effect for RackEffect {
    fn process(&mut self, context: &AudioContext, data: &mut [f64]) {
        self.effect.process(context, data)
    }
}

impl Rack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push_effect<E: 'static + Effect>(&mut self, effect: E) {
        self.effects.push_back(RackEffect::new(effect));
    }

    pub fn insert_effect<E: 'static + Effect>(&mut self, pos: usize, effect: E) {
        let mut after = self.effects.split_off(pos);
        after.push_front(RackEffect::new(effect));
        self.effects.append(&mut after);
    }

    pub fn remove_effect(&mut self, pos: usize) {
        let mut after = self.effects.split_off(pos);
        after.pop_front();
        self.effects.append(&mut after);
    }

    pub fn reorder_effect(&mut self, pos_src: usize, pos_dest: usize) {
        let mut after = self.effects.split_off(pos_src);
        let effect = after.pop_front().unwrap();
        self.effects.append(&mut after);

        let mut after = self.effects.split_off(pos_dest);
        after.push_front(effect);
        self.effects.append(&mut after);
    }
}

impl Default for Rack {
    fn default() -> Self {
        Self {
            effects: LinkedList::new(),
        }
    }
}

impl Effect for Rack {
    fn process(&mut self, context: &AudioContext, data: &mut [f64]) {
        if !self.effects.is_empty() {
            for effect in self.effects.iter_mut().filter(|e| e.enabled) {
                effect.process(context, data);
            }
        }
    }
}
