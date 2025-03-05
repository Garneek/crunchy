use plugin_utils::egui_utils::KnobLayout;
use plugin_utils::egui_utils::KnobPreset;

use nih_plug_egui::egui::Color32;

pub(crate) const WIDTH: u32 = 400;
pub(crate) const HEIGHT: u32 = 488;

// Ferra color palette

#[allow(dead_code)]
pub(crate) const FERRA_NIGHT: Color32 = Color32::from_rgb(42, 41, 45);
#[allow(dead_code)]
pub(crate) const FERRA_ASH: Color32 = Color32::from_rgb(55, 53, 57);
#[allow(dead_code)]
pub(crate) const FERRA_UMBER: Color32 = Color32::from_rgb(77, 66, 75);
#[allow(dead_code)]
pub(crate) const FERRA_BARK: Color32 = Color32::from_rgb(111, 93, 99);
#[allow(dead_code)]
pub(crate) const FERRA_MIST: Color32 = Color32::from_rgb(209, 209, 224);
#[allow(dead_code)]
pub(crate) const FERRA_SAGE: Color32 = Color32::from_rgb(177, 182, 149);
#[allow(dead_code)]
pub(crate) const FERRA_BLUSH: Color32 = Color32::from_rgb(254, 205, 178);
#[allow(dead_code)]
pub(crate) const FERRA_CORAL: Color32 = Color32::from_rgb(255, 160, 122);
#[allow(dead_code)]
pub(crate) const FERRA_ROSE: Color32 = Color32::from_rgb(246, 182, 201);
#[allow(dead_code)]
pub(crate) const FERRA_EMBER: Color32 = Color32::from_rgb(224, 107, 117);
#[allow(dead_code)]
pub(crate) const FERRA_HONEY: Color32 = Color32::from_rgb(245, 215, 110);

pub(crate) const KNOB_WIDTH: f32 = KNOB_PRESET.radius.unwrap() * 2_f32 + 4.75_f32 + 16_f32;
pub(crate) const SPACE_RIGHT_OF_KNOBS: f32 = WIDTH as f32 * 0.065_f32 + 1_f32;

pub(crate) const BACKGROUND_ROUNDING: f32 = 8_f32;
pub(crate) const BACKGROUND_OPACITY: f32 = 0.6_f32;

pub(crate) const KNOB_PRESET: KnobPreset = KnobPreset {
    radius: Some(45_f32),
    line_color: Some(FERRA_ROSE),
    background_color: Some(FERRA_ASH),
    text_color_override: Some(FERRA_BLUSH),
    knob_color: Some(FERRA_UMBER),
    center_size: None,
    line_width: None,
    center_to_line_space: None,
    hover_text: Some(true),
    show_center_value: None,
    text_size: Some(24_f32),
    outline: Some(true),
    padding: None,
    show_label: Some(true),
    swap_label_and_value: None,
    readable_box: None,
    background_radius: Some(BACKGROUND_ROUNDING),
    background_opacity: Some(BACKGROUND_OPACITY),
    layout: Some(KnobLayout::Vertical),
    arc_start: None,
    arc_end: None,
};
