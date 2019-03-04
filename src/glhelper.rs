use std::io::Cursor;
use image;
use glium;

// path: path to image file
// format: example image::PNG
#[allow(dead_code)]
pub fn load_texture(
    path: &str, 
    format: image::ImageFormat, 
    display: &glium::Display) 
    -> glium::texture::Texture2d
{
    let data: Vec<u8> = std::fs::read(path).unwrap();
    let img = image::load(
        Cursor::new(data), format).unwrap().to_rgba();
    let dims = img.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(
        &img.into_raw(), dims);
    glium::texture::Texture2d::new(display, image).unwrap()
}
