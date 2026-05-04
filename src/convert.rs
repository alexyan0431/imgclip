use std::io::Cursor;

use image::{DynamicImage, ExtendedColorType, ImageEncoder, ImageFormat, RgbaImage};

use crate::cli::OutputFormat;
use crate::error::AppError;

pub fn decode_file(data: &[u8]) -> Result<crate::clipboard::RawImage, AppError> {
    let img = image::ImageReader::new(Cursor::new(data))
        .with_guessed_format()
        .map_err(|e| AppError::Args(format!("cannot detect image format: {e}")))?
        .decode()
        .map_err(|e| AppError::Args(format!("cannot decode image: {e}")))?;

    let rgba = img.to_rgba8();
    Ok(crate::clipboard::RawImage {
        width: rgba.width(),
        height: rgba.height(),
        data: rgba.into_raw(),
    })
}

pub fn encode(
    raw: &crate::clipboard::RawImage,
    format: OutputFormat,
    quality: u8,
) -> Result<Vec<u8>, AppError> {
    let rgba = RgbaImage::from_raw(raw.width, raw.height, raw.data.clone()).ok_or_else(|| {
        AppError::Clipboard(format!(
            "image data size mismatch: expected {} bytes for {}x{} RGBA, got {}",
            raw.width as usize * raw.height as usize * 4,
            raw.width,
            raw.height,
            raw.data.len()
        ))
    })?;
    let dynamic = DynamicImage::ImageRgba8(rgba);

    match format {
        OutputFormat::Png => encode_png(&dynamic),
        OutputFormat::Jpeg => encode_jpeg(&dynamic, quality),
    }
}

fn encode_png(img: &DynamicImage) -> Result<Vec<u8>, AppError> {
    let mut buf = Vec::new();
    img.write_to(&mut Cursor::new(&mut buf), ImageFormat::Png)
        .map_err(|e| AppError::Clipboard(format!("failed to encode PNG: {e}")))?;
    Ok(buf)
}

fn encode_jpeg(img: &DynamicImage, quality: u8) -> Result<Vec<u8>, AppError> {
    // Composite onto white background so transparent regions don't turn black
    let mut bg =
        image::RgbaImage::from_pixel(img.width(), img.height(), image::Rgba([255, 255, 255, 255]));
    image::imageops::overlay(&mut bg, img, 0, 0);
    let rgb = DynamicImage::ImageRgba8(bg).to_rgb8();

    let mut buf = Vec::new();
    let encoder = image::codecs::jpeg::JpegEncoder::new_with_quality(&mut buf, quality);
    encoder
        .write_image(
            rgb.as_raw(),
            rgb.width(),
            rgb.height(),
            ExtendedColorType::Rgb8,
        )
        .map_err(|e| AppError::Clipboard(format!("failed to encode JPEG: {e}")))?;
    Ok(buf)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clipboard::RawImage;

    fn red_2x2() -> RawImage {
        RawImage {
            width: 2,
            height: 2,
            data: vec![
                255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255, 255, 0, 0, 255,
            ],
        }
    }

    #[test]
    fn png_starts_with_magic() {
        let img = red_2x2();
        let bytes = encode(&img, OutputFormat::Png, 85).unwrap();
        assert!(
            bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47]),
            "PNG magic bytes"
        );
    }

    #[test]
    fn jpeg_starts_with_magic() {
        let img = red_2x2();
        let bytes = encode(&img, OutputFormat::Jpeg, 85).unwrap();
        assert!(bytes.starts_with(&[0xFF, 0xD8, 0xFF]), "JPEG magic bytes");
    }

    #[test]
    fn jpeg_quality_affects_size() {
        let img = red_2x2();
        let low = encode(&img, OutputFormat::Jpeg, 10).unwrap();
        let high = encode(&img, OutputFormat::Jpeg, 100).unwrap();
        assert!(!low.is_empty() && !high.is_empty());
    }

    #[test]
    fn single_pixel() {
        let img = RawImage {
            width: 1,
            height: 1,
            data: vec![0, 255, 0, 255],
        };
        let png = encode(&img, OutputFormat::Png, 85).unwrap();
        assert!(png.starts_with(&[0x89, 0x50, 0x4E, 0x47]));
        let jpeg = encode(&img, OutputFormat::Jpeg, 85).unwrap();
        assert!(jpeg.starts_with(&[0xFF, 0xD8, 0xFF]));
    }

    #[test]
    fn jpeg_transparent_pixel_composited_to_white() {
        // Fully transparent pixel should become white, not black
        let img = RawImage {
            width: 1,
            height: 1,
            data: vec![128, 64, 32, 0], // semi-transparent, should blend to white bg
        };
        let bytes = encode(&img, OutputFormat::Jpeg, 100).unwrap();
        assert!(bytes.starts_with(&[0xFF, 0xD8, 0xFF]));
    }

    #[test]
    fn decode_png_roundtrip() {
        let original = red_2x2();
        let encoded = encode(&original, OutputFormat::Png, 85).unwrap();
        let decoded = decode_file(&encoded).unwrap();
        assert_eq!(decoded.width, 2);
        assert_eq!(decoded.height, 2);
    }

    #[test]
    fn encode_mismatched_dimensions_returns_error() {
        let img = RawImage {
            width: 3,
            height: 3,
            data: vec![0; 4], // only 1 pixel worth of data for 3x3 claim
        };
        let err = encode(&img, OutputFormat::Png, 85).unwrap_err();
        assert!(matches!(err, AppError::Clipboard(_)));
        let msg = err.to_string();
        assert!(msg.contains("mismatch"), "got: {msg}");
    }
}
