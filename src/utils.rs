use std::error::Error;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use egui::{ColorImage, TextureHandle, TextureOptions};
use image::ImageFormat;

pub fn base64_to_egui_texture(
    ctx: &egui::Context,
    base64_str: &str,
    image_name: &str,
) -> Result<TextureHandle, Box<dyn Error>> {
    // 1. Decode base64 string to bytes
    let data = STANDARD.decode(base64_str)?;

    // 2. Load image from memory (specify format, e.g., PNG)
    let img =
        image::load_from_memory_with_format(&data, ImageFormat::Png).expect("Failed to load image");

    // 3. Convert to RGBA format and get raw bytes
    let rgba = img.to_rgba8();
    let size = [img.width() as usize, img.height() as usize];

    // 4. Create ColorImage from raw RGBA bytes
    let color_image = ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());

    // 5. Load texture into egui context
    Ok(ctx.load_texture(image_name, color_image, TextureOptions::default()))
}
