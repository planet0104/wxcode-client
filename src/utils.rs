use image::RgbaImage;
use sixtyfps::{Rgba8Pixel, SharedPixelBuffer, Image};
use anyhow::Result;

pub fn load_image_from_memory(buf:&[u8]) -> Result<Image>{
    let img = image::load_from_memory(buf)?.to_rgba8();
    Ok(load_image_from_rgba8(img))
}

pub fn load_image_from_rgba8(i: RgbaImage) -> Image{
    let buffer = SharedPixelBuffer::<Rgba8Pixel>::clone_from_slice(
        i.as_raw(),
        i.width() as _,
        i.height() as _,
    );
    Image::from_rgba8(buffer)
}