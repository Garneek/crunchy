use nih_plug::prelude::ParamSetter;

use plugin_utils::egui_utils::*;

use crate::CrunchyParams;
use std::collections::HashMap;
use std::sync::Arc;

use image::EncodableLayout;

use nih_plug::editor::Editor;

use nih_plug_egui::create_egui_editor;
use nih_plug_egui::egui;
use nih_plug_egui::egui::CentralPanel;
use nih_plug_egui::EguiState;

pub(crate) const WIDTH: u32 = 400;
pub(crate) const HEIGHT: u32 = 488;

pub(crate) const SPACE_RIGHT_OF_KNOBS: f32 = WIDTH as f32 * 0.065_f32 + 1_f32;

pub(crate) const BACKGROUND_ROUNDING: f32 = 8_f32;
pub(crate) const BACKGROUND_OPACITY: f32 = 0.65_f32;

const WIDGET_STYLE: WidgetStyle = WidgetStyle::const_default()
    .set_size(96_f32)
    .set_text_size(24_f32)
    .set_background_opacity(BACKGROUND_OPACITY)
    .set_text_backdrop_color(premult_color32(
        ferra_color::FERRA_UMBER,
        BACKGROUND_OPACITY,
    ));

fn load_images(cx: &egui::Context) -> HashMap<&'static str, egui::TextureHandle> {
    let mut map = HashMap::new();
    insert_handle_to_map_from_bytes!(map, cx, "../resources/background.png", "background", 1_f64);
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

fn knob_container(ui: &mut egui::Ui, params: Arc<CrunchyParams>, setter: &ParamSetter) {
    ui.with_layout(
        nih_plug_egui::egui::Layout::right_to_left(egui::Align::Min),
        |ui| {
            ui.add_space(SPACE_RIGHT_OF_KNOBS);
            ui.add(
                ArcKnob::new(&params.crush, setter, KnobLayout::Vertical, &WIDGET_STYLE)
                    .set_hover_text(
                        "Bitcrusher applied to the frequency components of the sound".to_string(),
                    ),
            );
            ui.add(
                ArcKnob::new(&params.crunch, setter, KnobLayout::Vertical, &WIDGET_STYLE)
                    .set_hover_text(
                        "Clip applied to the frequency components of the sound".to_string(),
                    ),
            );
            ui.add(
                ArcKnob::new(&params.drive, setter, KnobLayout::Vertical, &WIDGET_STYLE)
                    .set_hover_text("Gain applied before further processing".to_string()),
            );
        },
    );
    ui.add_space(WIDGET_STYLE.element_size * 0.06_f32);
    ui.with_layout(
        nih_plug_egui::egui::Layout::right_to_left(egui::Align::Min),
        |ui| {
            ui.add_space(SPACE_RIGHT_OF_KNOBS);
            ui.add(
                ArcKnob::new(&params.gain, setter, KnobLayout::Vertical, &WIDGET_STYLE)
                    .set_hover_text("Gain applied after all processing".to_string()),
            );
            ui.add(
                ArcKnob::new(&params.mix, setter, KnobLayout::Vertical, &WIDGET_STYLE)
                    .set_hover_text("Amount of wet signal vs dry signal".to_string()),
            );
        },
    );
}

const TITLE_FONT_SIZE: f32 = 32_f32;
fn title_card(ui: &mut egui::Ui) {
    ui.vertical_centered(|ui| {
        ui.add_space(HEIGHT as f32 * (0.02_f32 + (1_f32 / 32_f32)));
        let rect = ui
            .allocate_space(egui::Vec2::new(
                WIDTH as f32 - (SPACE_RIGHT_OF_KNOBS * 2_f32),
                HEIGHT as f32 * 0.1_f32,
            ))
            .1;
        let painter = ui.painter_at(rect);
        ui.painter().rect_filled(
            rect,
            egui::CornerRadius::from(BACKGROUND_ROUNDING),
            ferra_color::FERRA_ASH.linear_multiply(BACKGROUND_OPACITY),
        );

        painter.text(
            egui::Pos2::new(rect.center().x, rect.center().y + 0_f32),
            egui::Align2::CENTER_CENTER,
            "Jest kranczips, jest impreza",
            egui::FontId::proportional(TITLE_FONT_SIZE),
            ferra_color::FERRA_BLUSH,
        );
    });
}

const AUTHOR_FONT_SIZE: f32 = 12_f32;
fn author_text(ui: &mut egui::Ui) {
    ui.with_layout(
        nih_plug_egui::egui::Layout::right_to_left(egui::Align::Min),
        |ui| {
            ui.add_space(SPACE_RIGHT_OF_KNOBS);

            let rect = ui
                .allocate_space(egui::Vec2::new(
                    WIDTH as f32 * 0.3_f32,
                    HEIGHT as f32 * 0.05_f32,
                ))
                .1;
            let painter = ui.painter_at(rect);
            ui.painter().rect_filled(
                rect,
                egui::CornerRadius::from(BACKGROUND_ROUNDING),
                ferra_color::FERRA_ASH.linear_multiply(BACKGROUND_OPACITY),
            );

            painter.text(
                rect.center(),
                egui::Align2::CENTER_CENTER,
                "Crunchy 0.2.3 by Garneek",
                egui::FontId::proportional(AUTHOR_FONT_SIZE),
                ferra_color::FERRA_BLUSH,
            );
        },
    );
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
                Arc::new(egui::FontData::from_static(include_bytes!(
                    "../resources/futura/FuturaCondensed.ttf"
                ))),
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
                .frame(egui::Frame::NONE)
                .show(cx, |ui| {
                    background_image(ui, user_state, egui::Frame::NONE, "background", |ui| {
                        ui.vertical(|ui| {
                            title_card(ui);
                            ui.add_space(HEIGHT as f32 * 0.11_f32);
                            knob_container(ui, params.clone(), &setter);
                            ui.add_space(HEIGHT as f32 * 0.025_f32);
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
