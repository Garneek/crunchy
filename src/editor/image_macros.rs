use nih_plug_egui::egui;

macro_rules! handle_from_bytes {
    ($cx:ident, $path:expr, $name:expr, $scale:expr) => {{
        let image_data = image::load_from_memory(include_bytes!($path)).unwrap();
        let rescaled_size = [
            (image_data.width() as f64 * $scale) as usize,
            (image_data.height() as f64 * $scale) as usize,
        ];
        let color_image = egui::ColorImage::from_rgba_unmultiplied(
            rescaled_size,
            image_data
                .resize_to_fill(
                    rescaled_size[0] as u32,
                    rescaled_size[1] as u32,
                    image::imageops::FilterType::CatmullRom,
                )
                .into_rgba8()
                .as_bytes(),
        );

        $cx.load_texture($name, color_image, egui::TextureOptions::LINEAR)
    }};
}

macro_rules! insert_handle_to_map_from_bytes {
    ($map:ident, $cx:ident, $path:expr, $name:expr, $scale:expr) => {
        $map.insert($name, handle_from_bytes!($cx, $path, $name, $scale));
    };
}

macro_rules! get_image_handle_ref {
    ($user_state:ident, $name:expr) => {
        match &$user_state.handles {
            Some(handles) => match handles.get($name) {
                Some(v) => v,
                None => {
                    panic!("Could not find requested image");
                }
            },
            None => {
                panic!("Images were not initialized");
            }
        }
    };
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
                get_image_handle_ref!(user_state, image),
            ))
            .paint_at(ui, image_rect);

            ui_callback(ui);
        })
}
