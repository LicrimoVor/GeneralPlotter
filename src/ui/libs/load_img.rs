use eframe::egui;
use std::path::PathBuf;

pub fn get_image_path(file_name: &str) -> PathBuf {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("assets");
    path.push(file_name);
    path
}

pub fn load_img<'a>(ctx: &'a egui::Context, name: &str, size: egui::Vec2) -> egui::Image<'a> {
    let image = image::open(get_image_path(name))
        .expect("Не удалось загрузить изображение")
        .to_rgba8();

    let size_img = [image.width() as usize, image.height() as usize];
    let color_image = egui::ColorImage::from_rgba_unmultiplied(size_img, &image);
    let texture = ctx.load_texture(name, color_image, Default::default());
    egui::Image::new(&texture).max_size(size)
}
