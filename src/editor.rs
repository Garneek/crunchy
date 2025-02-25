use nih_plug::prelude::ParamSetter;

use plugin_utils::egui_utils::*;

mod style;
use style::*;

use crate::CrunchyParams;
use std::collections::HashMap;
use std::sync::Arc;

use image::EncodableLayout;

use nih_plug::editor::Editor;

use nih_plug_egui::create_egui_editor;
use nih_plug_egui::egui;
use nih_plug_egui::egui::CentralPanel;
use nih_plug_egui::EguiState;

fn load_images(cx: &egui::Context) -> HashMap<&'static str, egui::TextureHandle> {
    let mut map = HashMap::new();
    insert_handle_to_map_from_bytes!(map, cx, "../resources/background.png", "background", 1_f64);
    insert_handle_to_map_from_bytes!(
        map,
        cx,
        "../resources/background.png",
        "background_scaled",
        1_f64 / 5_f64
    );
    map
}

pub(crate) struct UserState {
    pub(crate) handles: Option<HashMap<&'static str, egui::TextureHandle>>,
}

impl Default for UserState {
    fn default() -> Self {
        Self { handles: None }
    }
}

impl UserState {
    fn get_handle_ref(&self, name: &str) -> &egui::TextureHandle {
        match &self.handles {
            Some(handles) => match handles.get(name) {
                Some(v) => v,
                None => {
                    panic!("Could not find requested image");
                }
            },
            None => {
                panic!("Images were not initialized");
            }
        }
    }
}

const SPACE_RIGHT_OF_KNOBS: f32 = WIDTH as f32 / 32_f32;
fn knob_container(ui: &mut egui::Ui, params: Arc<CrunchyParams>, setter: &ParamSetter) {
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - SPACE_RIGHT_OF_KNOBS - KNOB_WIDTH * 3_f32);
        ui.add(
            ArcKnob::for_param(&params.drive, setter, 0_f32, KnobLayout::Vertical)
                .apply_preset(&KNOB_PRESET)
                .set_hover_text("Gain applied before further processing".to_string()),
        );
        ui.add(
            ArcKnob::for_param(&params.crunch, setter, 0_f32, KnobLayout::Vertical)
                .apply_preset(&KNOB_PRESET)
                .set_hover_text(
                    "Clip applied to the frequency components of the sound".to_string(),
                ),
        );
        ui.add(
            ArcKnob::for_param(&params.crush, setter, 0_f32, KnobLayout::Vertical)
                .apply_preset(&KNOB_PRESET)
                .set_hover_text(
                    "Bitcrusher applied to the frequency components of the sound".to_string(),
                ),
        );
    });
    ui.add_space(KNOB_PRESET.radius.unwrap_or(0_f32) * 0.25);
    ui.horizontal(|ui| {
        ui.add_space(ui.available_width() - SPACE_RIGHT_OF_KNOBS - KNOB_WIDTH * 2_f32);
        ui.add(
            ArcKnob::for_param(&params.mix, setter, 0_f32, KnobLayout::Vertical)
                .apply_preset(&KNOB_PRESET)
                .set_hover_text("Amount of wet signal vs dry signal".to_string()),
        );
        ui.add(
            ArcKnob::for_param(&params.gain, setter, 0_f32, KnobLayout::Vertical)
                .apply_preset(&KNOB_PRESET)
                .set_hover_text("Gain applied after all processing".to_string()),
        );
    });
}

const TITLE_FONT_SIZE: f32 = 28_f32;
fn title_card(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(HEIGHT as f32 / 32_f32);
        let rect = ui
            .allocate_space(egui::Vec2::new(
                WIDTH as f32 * 0.8_f32,
                HEIGHT as f32 * 0.1_f32,
            ))
            .1;
        let painter = ui.painter_at(rect);
        ui.painter().rect_filled(
            rect,
            egui::Rounding::from(BACKGROUND_ROUNDING),
            FERRA_ASH.linear_multiply(BACKGROUND_OPACITY),
        );

        painter.text(
            egui::Pos2::new(rect.center().x, rect.center().y + 0_f32),
            egui::Align2::CENTER_CENTER,
            "Jest kranczips, jest impreza",
            egui::FontId::proportional(TITLE_FONT_SIZE),
            FERRA_BLUSH,
        );
    });
}

const AUTHOR_FONT_SIZE: f32 = 12_f32;
fn author_text(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.add_space(WIDTH as f32 * 0.7_f32 - SPACE_RIGHT_OF_KNOBS - 16_f32);

        let rect = ui
            .allocate_space(egui::Vec2::new(
                WIDTH as f32 * 0.3_f32,
                HEIGHT as f32 * 0.05_f32,
            ))
            .1;
        let painter = ui.painter_at(rect);
        ui.painter().rect_filled(
            rect,
            egui::Rounding::from(BACKGROUND_ROUNDING),
            FERRA_ASH.linear_multiply(BACKGROUND_OPACITY),
        );

        painter.text(
            rect.center(),
            egui::Align2::CENTER_CENTER,
            "Crunchy by Garneek",
            egui::FontId::proportional(AUTHOR_FONT_SIZE),
            FERRA_BLUSH,
        );
    });
}

pub(crate) fn default_state() -> Arc<EguiState> {
    EguiState::from_size(WIDTH, HEIGHT)
}

pub(crate) fn create(params: Arc<CrunchyParams>, state: Arc<EguiState>) -> Option<Box<dyn Editor>> {
    create_egui_editor(
        state,
        UserState::default(),
        |cx, user_state| {
            user_state.handles = Some(load_images(cx));
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.insert(
                "futura".to_string(),
                egui::FontData::from_static(include_bytes!(
                    "../resources/futura/FuturaCondensed.ttf"
                )),
            );
            fonts
                .families
                .entry(egui::FontFamily::Proportional)
                .or_default()
                .insert(0, "futura".to_string());
            cx.set_fonts(fonts);
        },
        move |cx, setter, user_state| {
            CentralPanel::default()
                .frame(egui::Frame::none())
                .show(cx, |ui| {
                    background_image(ui, user_state, egui::Frame::none(), "background", |ui| {
                        ui.vertical(|ui| {
                            title_card(ui);
                            ui.add_space(HEIGHT as f32 * 0.24_f32);
                            knob_container(ui, params.clone(), &setter);
                            ui.add_space(HEIGHT as f32 * 0.020_f32);
                            author_text(ui);
                        });
                    });
                });
        },
    )
}

pub fn background_image<T>(
    outer_ui: &mut egui::Ui,
    user_state: &crate::editor::UserState,
    frame: egui::Frame,
    image: &str,
    ui_callback: T,
) -> egui::InnerResponse<()>
where
    T: Fn(&mut egui::Ui),
{
    egui::CentralPanel::default()
        .frame(frame)
        .show_inside(outer_ui, |ui| {
            let mut image_rect = ui.available_rect_before_wrap();
            image_rect.set_height(ui.available_height() + 2_f32);
            egui::Image::from_texture(egui::load::SizedTexture::from_handle(
                user_state.get_handle_ref(image),
            ))
            .paint_at(ui, image_rect);

            ui_callback(ui);
        })
}
