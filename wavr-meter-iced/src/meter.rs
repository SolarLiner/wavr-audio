/*
 * Copyright (c) 2020 the Wavr Audio project.
 * This source file, as well as the binaries generated by it,
 * are licensed under MIT.
 */
use iced::{
    canvas::{self, Path, Stroke},
    Canvas,
};
use iced_native::{
    layout::{Limits, Node},
    window::Backend,
    Background, Color, Element, Hasher, Layout, Length, MouseCursor, Point, Size, Widget,
};
use iced_wgpu::{Defaults, Primitive, Renderer};

use wavr_meter::decibel::{Linear, LUFS};
use wavr_meter::{decibel::Decibel, WavrMeterData};

use crate::core::Range;
use iced_wgpu::widget::canvas::Frame;
use num::Float;

#[derive(Copy, Clone, Debug)]
pub struct Meter {
    range: Range<f64>,
    peak_data: Decibel,
    loudness_data: LUFS,
    peak_color: Color,
    loudness_color: Color,
}

impl Meter {
    pub fn new<P: Into<Decibel>, L: Into<LUFS>>(peak: P, loudness: L) -> Self {
        Self {
            range: Range {
                min: -48.0,
                max: 6.0,
            },
            peak_data: peak.into(),
            loudness_data: loudness.into(),
            peak_color: Color::from([0.1, 1.0, 0.2, 1.0]),
            loudness_color: Color::from([0.2, 0.3, 1.0, 0.5]),
        }
    }

    pub fn set_values<P: Into<Decibel>, L: Into<LUFS>>(&mut self, peak: P, loudness: L) {
        self.peak_data = peak.into();
        self.loudness_data = loudness.into();
    }
}

impl Default for Meter {
    fn default() -> Self {
        Self::new(Linear(0.0), Linear(0.0))
    }
}

impl canvas::Drawable for Meter {
    fn draw(&self, frame: &mut Frame) {
        use canvas::{Path, Stroke};
        let size = frame.size();
        let peak_height = clamp(self.range.map(self.peak_data.0)) as f32;
        let loudness_height = clamp(self.range.map(self.loudness_data.0)) as f32;

        let peak_rect = Path::rectangle(
            Point::new(0.0, size.height * (1.0 - peak_height)),
            Size::new(size.width, size.height * peak_height),
        );
        let loudness_rect = Path::rectangle(
            Point::new(0.0, size.height * (1.0 - loudness_height)),
            Size::new(size.width, size.height * loudness_height),
        );

        let ticks = Path::new(|builder| {
            for (pos, value) in self.range.linspace(10.0) {
                let height = size.height * (1.0 - pos as f32);

                builder.move_to(Point::new(0., height));
                builder.line_to(Point::new(size.width, height));
            }
        });

        frame.fill(
            &Path::rectangle(Point::new(0.0, 0.0), size),
            Color::from([0.0, 0.0, 0.0, 0.1]),
        );
        frame.fill(&peak_rect, self.peak_color);
        frame.fill(&loudness_rect, self.loudness_color);
        frame.stroke(
            &ticks,
            Stroke {
                color: Color::BLACK,
                width: 0.5,
                ..Default::default()
            },
        );
    }
}

fn clamp<F: Float>(x: F) -> F {
    if x < F::zero() {
        F::zero()
    } else if x > F::one() {
        F::one()
    } else {
        x
    }
}
