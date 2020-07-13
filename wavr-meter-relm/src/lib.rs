use gtk::ContainerExt;
use relm::{Component, ContainerWidget, Relm, Update, Widget};

use wavr_meter::WavrMeterData;

use crate::meter::SingleMeter;

mod meter;
mod range;

pub enum Messages {
    Setup(u16),
    Value(WavrMeterData),
}

pub struct WavrMeterWidget {
    model: Option<WavrMeterData>,
    root: gtk::Box,
    meters_box: gtk::Box,
    meters: Vec<Component<SingleMeter>>,
}

impl Update for WavrMeterWidget {
    type Model = Option<WavrMeterData>;
    type ModelParam = ();
    type Msg = Messages;

    fn model(relm: &Relm<Self>, param: Self::ModelParam) -> Self::Model {
        None
    }

    fn update(&mut self, event: Self::Msg) {
        match event {
            Messages::Setup(channelcount) => {
                for _ in 0..channelcount {
                    self.meters
                        .push(self.meters_box.add_widget::<SingleMeter>(()));
                }
            }
            Messages::Value(data) => {
                self.model = Some(data);
                for (i, meter) in self.meters.iter().enumerate() {
                    meter.emit(meter::Messages::Value(
                        data.peak[i].into(),
                        data.loudness.clone(),
                    ));
                }
            }
        }
    }
}

impl Widget for WavrMeterWidget {
    type Root = gtk::Box;

    fn root(&self) -> Self::Root {
        self.root.clone()
    }

    fn view(relm: &Relm<Self>, model: Option<WavrMeterData>) -> Self {
        let root = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Vertical)
            .spacing(10)
            .build();
        let loudness_label = gtk::LabelBuilder::new()
            .label(if let Some(data) = model.as_ref() {
                &format!("{}", data.loudness)
            } else {
                "--.-- dB"
            })
            .build();
        root.add(&loudness_label);
        let meters_box = gtk::BoxBuilder::new()
            .orientation(gtk::Orientation::Horizontal)
            .spacing(7)
            .build();
        Self {
            root,
            model,
            meters_box,
            meters: vec![],
        }
    }
}
