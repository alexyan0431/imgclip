pub struct RawImage {
    pub width: u32,
    pub height: u32,
    pub data: Vec<u8>,
}

pub fn read_image() -> Result<RawImage, crate::error::AppError> {
    let mut clipboard = arboard::Clipboard::new()?;
    let img = clipboard.get_image()?;

    Ok(RawImage {
        width: img.width as u32,
        height: img.height as u32,
        data: img.bytes.into_owned(),
    })
}

pub fn write_image(img: &RawImage) -> Result<(), crate::error::AppError> {
    let mut clipboard = arboard::Clipboard::new()?;
    let image_data = arboard::ImageData {
        width: img.width as usize,
        height: img.height as usize,
        bytes: img.data.as_slice().into(),
    };
    clipboard.set_image(image_data)?;
    Ok(())
}
