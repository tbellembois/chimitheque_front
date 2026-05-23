use std::error::Error;

use base64::{Engine as _, engine::general_purpose::STANDARD};
use egui::{Color32, ColorImage, TextureHandle, TextureOptions};
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

pub fn html_color_to_egui(hex: &str) -> Option<Color32> {
    // Ensure the string starts with '#'
    let hex = hex.trim_start_matches('#');

    // Parse the hex string into RGB values
    // hex::decode returns a Vec<u8>, we expect 3 bytes for RGB
    let bytes = hex::decode(hex).ok()?;

    if bytes.len() == 3 {
        Some(Color32::from_rgb(bytes[0], bytes[1], bytes[2]))
    } else {
        None // Handle invalid length (e.g., RGBA or short hex)
    }
}
