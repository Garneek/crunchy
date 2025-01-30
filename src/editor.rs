use crate::CrunchyParams;
use std::sync::Arc;

use nih_plug::editor::Editor;
use nih_plug::nih_error;

use nih_plug_vizia::assets;
use nih_plug_vizia::create_vizia_editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::ViziaState;
use nih_plug_vizia::ViziaTheming;

use nih_plug_vizia::widgets::*;

const BACKGROUND_IMAGE: &[u8] = include_bytes!("../resources/images/background.png");

const STYLE: &str = include_str!("../resources/styles/main.css");

#[derive(Lens)]
struct EditorData {
    params: Arc<CrunchyParams>,
}

impl Model for EditorData {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    #[cfg(not(feature = "debug"))]
    return ViziaState::new(|| (600 * 2 / 3, 732 * 2 / 3));

    #[cfg(feature = "debug")]
    return ViziaState::new(|| (600 * 2 / 3, 732 * 2 / 3));
}

fn load_data_into_context(cx: &mut Context) {
    assets::register_noto_sans_light(cx);
    assets::register_noto_sans_thin(cx);

    #[cfg(feature = "debug")]
    match cx.add_stylesheet(STYLE) {
        Ok(_) => {}
        Err(e) => {
            nih_error!("{:?}", e);
            panic!("Error loading stylesheet");
        }
    };

    #[cfg(not(feature = "debug"))]
    cx.add_stylesheet(STYLE).expect("Error loading stylesheet");

    let bg_img = image::load_from_memory_with_format(BACKGROUND_IMAGE, image::ImageFormat::Png)
        .expect("Image format error");

    let bg_img = bg_img.resize(
        bg_img.width() * 2 / 3,
        bg_img.height() * 2 / 3,
        image::imageops::FilterType::Lanczos3,
    );

    cx.load_image(
        "background_image.png",
        bg_img,
        ImageRetentionPolicy::Forever,
    );
}

macro_rules! knob_label_pair {
    ($cx: expr, $label_name: expr, $param_closure: expr) => {{
        VStack::new($cx, |cx| {
            Label::new(cx, $label_name)
                .class("label")
                .text_align(TextAlign::Center)
                .left(Pixels(24_f32));
            ParamSlider::new(cx, EditorData::params, $param_closure)
                .class("knob")
                .left(Pixels(24_f32));
        })
        .text_align(TextAlign::Center)
        .left(Pixels(40_f32))
        .class("knob-label-pair-container");
    }};
}

fn knobs_container(cx: &mut Context) {
    VStack::new(cx, |cx| {
        knob_label_pair!(cx, "Drive", |params| &params.drive);
        knob_label_pair!(cx, "Crunch", |params| &params.crunch);
        knob_label_pair!(cx, "Crush", |params| &params.crush);
        knob_label_pair!(cx, "Mix", |params| &params.mix);
        knob_label_pair!(cx, "Gain", |params| &params.gain);
    })
    .top(Pixels(12_f32))
    .class("knobs-vertical-subcontainer");
}

fn ui(cx: &mut Context) {
    #[cfg(not(feature = "debug"))]
    VStack::new(cx, |cx| {
        VStack::new(cx, |cx| {
            Label::new(cx, "Jest kranczips, jest impreza")
                .class("plugin-title")
                .text_align(TextAlign::Center)
                .top(Pixels(8_f32));
        })
        .class("title-container");
        VStack::new(cx, |cx| {
            knobs_container(cx);
        })
        .class("knobs-main-container");
    })
    .class("background-image-class");

    #[cfg(feature = "debug")]
    VStack::new(cx, |cx| {
        DebugLabel::new(cx)
            .class("plugin-title")
            .width(Pixels(300_f32));
        Label::new(cx, "test1")
            .class("plugin-title")
            .width(Pixels(350_f32))
            .top(Pixels(50_f32));
    })
    .class("knobs-main-container");
    ResizeHandle::new(cx);
}

pub(crate) fn create(
    params: Arc<CrunchyParams>,
    state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(state, ViziaTheming::Custom, move |cx, _| {
        load_data_into_context(cx);

        EditorData {
            params: params.clone(),
        }
        .build(cx);

        ui(cx);
    })
}
