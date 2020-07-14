/*
 * Copyright (c) 2020 the Wavr Audio project.
 * This source file, as well as the binaries generated by it,
 * are licensed under MIT.
 */
use std::marker::PhantomData;
use std::sync::mpsc::{sync_channel, Receiver, SyncSender};
use std::thread::JoinHandle;

use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use cpal::{
    default_host, BuildStreamError, Device, Sample, SampleFormat, SampleRate, Stream, StreamConfig,
    StreamError,
};
use gtk::WidgetExt;
use num::ToPrimitive;
use relm::{connect, Channel, Component, ContainerWidget, Relm, Sender, Update, Widget};
use relm_derive::Msg;

use wavr_meter::{WavrMeter, WavrMeterData};
use wavr_meter_relm::{Messages as WidgetMessages, WavrMeterWidget};

#[derive(Msg, Clone, Debug)]
enum AppMessages {
    MeterMessage(WidgetMessages),
    Quit,
}

struct App {
    root: gtk::Window,
    meter: Component<WavrMeterWidget>,
    channel: Channel<AppMessages>,
    stream: cpal::Stream,
}

impl Update for App {
    type Model = (Channel<AppMessages>, cpal::Stream);
    type ModelParam = (cpal::Device, cpal::SupportedStreamConfig);
    type Msg = AppMessages;

    fn model(relm: &Relm<Self>, (device, stream_config): Self::ModelParam) -> Self::Model {
        let (channel, sender) = Channel::new({
            let stream = relm.stream().clone();
            move |msg| stream.emit(msg)
        });
        let config = stream_config.config();
        let mut meter = WavrMeter::new(config.channels as u32, config.sample_rate.0);
        let stream = match stream_config.sample_format() {
            SampleFormat::U16 => device.build_input_stream::<u16, _, _>(
                &config,
                move |data, _cb_info| {
                    let buf = data
                        .iter()
                        .map(|v| cpal::Sample::to_f32(v) as f64)
                        .collect::<Vec<_>>();
                    meter.add_samples(&buf);
                    sender
                        .send(AppMessages::MeterMessage(WidgetMessages::Value(
                            meter.get_values(),
                        )))
                        .unwrap();
                },
                error_fn,
            ),
            SampleFormat::I16 => device.build_input_stream::<i16, _, _>(
                &config,
                move |data, _cb_info| {
                    let buf = data
                        .iter()
                        .map(|v| cpal::Sample::to_f32(v) as f64)
                        .collect::<Vec<_>>();
                    meter.add_samples(&buf);
                    sender
                        .send(AppMessages::MeterMessage(WidgetMessages::Value(
                            meter.get_values(),
                        )))
                        .unwrap();
                },
                error_fn,
            ),
            SampleFormat::F32 => device.build_input_stream::<f32, _, _>(
                &config,
                move |data, _cb_info| {
                    let buf = data.iter().map(|v| *v as f64).collect::<Vec<_>>();
                    meter.add_samples(&buf);
                    sender
                        .send(AppMessages::MeterMessage(WidgetMessages::Value(
                            meter.get_values(),
                        )))
                        .unwrap();
                },
                error_fn,
            ),
        }
        .unwrap();
        relm.stream()
            .emit(AppMessages::MeterMessage(WidgetMessages::Setup(
                config.channels,
            )));
        stream.play().unwrap();
        (channel, stream)
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            AppMessages::Quit => gtk::main_quit(),
            AppMessages::MeterMessage(event) => self.meter.emit(event),
        }
    }
}

fn error_fn(err: cpal::StreamError) {
    eprintln!("An error occurred on the audio thread: {:?}", err);
}

impl Widget for App {
    type Root = gtk::Window;

    fn init_view(&mut self) {
        self.root.show_all();
    }

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, (channel, stream): Self::Model) -> Self {
        let root = gtk::WindowBuilder::new()
            .border_width(8)
            .title("Wavr Meter")
            .hexpand(true)
            .vexpand(true)
            .build();
        connect!(
            relm,
            root,
            connect_delete_event(_, _),
            return (Some(AppMessages::Quit), gtk::Inhibit(false))
        );
        let meter = root.add_widget::<WavrMeterWidget>(());
        Self {
            root,
            meter,
            stream,
            channel,
        }
    }
}

fn main() {
    let host = default_host();
    let device = host.default_input_device().unwrap();
    let stream_config = device.default_input_config().unwrap();
    relm::run::<App>((device, stream_config)).unwrap();
}
