use image::DynamicImage;
use zxingcpp::{read, BarcodeFormat, ImageView};

fn decode_image(img: DynamicImage) -> Result<String, String> {
    let img = img.to_luma8();
    let (w, h) = img.dimensions();
    let view =
        ImageView::from_slice(&img, w as i32, h as i32, zxingcpp::ImageFormat::Lum)
            .map_err(|e| format!("Failed to create image view: {e}"))?;
    let results = read()
        .formats(&[BarcodeFormat::QRCode, BarcodeFormat::AllLinear])
        .try_invert(false)
        .from(&view)
        .map_err(|e| format!("Decode error: {e}"))?;
    if results.is_empty() {
        return Err("No barcode found in image".to_string());
    }
    let texts: Vec<String> = results.iter().map(|r| r.text()).collect();
    Ok(texts.join("\n"))
}

pub fn decode_image_file(path: &std::path::Path) -> Result<String, String> {
    let img = image::open(path).map_err(|e| format!("Failed to open image: {e}"))?;
    decode_image(img)
}

pub fn decode_image_bytes(bytes: &[u8]) -> Result<String, String> {
    let img = image::load_from_memory(bytes)
        .map_err(|e| format!("Failed to decode image bytes: {e}"))?;
    decode_image(img)
}
