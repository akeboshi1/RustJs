extern crate cfg_if;
extern crate image;
extern crate console_error_panic_hook;


use std::io::{Cursor, Read, Seek, SeekFrom};
use std::panic;

use cfg_if::cfg_if;
use image::DynamicImage;
use image::ImageFormat;

use crate::log;

cfg_if! {
    // When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
    // allocator.
    if #[cfg(feature = "wee_alloc")] {
        extern crate wee_alloc;
        #[global_allocator]
        static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;
    }
}

fn load_image_from_array(_array: &[u8]) -> DynamicImage {
    panic::set_hook(Box::new(console_error_panic_hook::hook));
    let img = match image::load_from_memory(_array) {
        Ok(img) => img,
        Err(error) => {
            log!("There was a problem opening the file: {:?}", error);
            panic!("There was a problem opening the file: {:?}", error)
        }
    };
    img
}

fn get_image_as_array(_img: DynamicImage) -> Vec<u8> {
    // Create fake "file"
    let mut c = Cursor::new(Vec::new());

    match _img.write_to(&mut c, ImageFormat::PNG) {
        Ok(c) => c,
        Err(error) => {
            log!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            );
            panic!(
                "There was a problem writing the resulting buffer: {:?}",
                error
            )
        }
    };
    c.seek(SeekFrom::Start(0)).unwrap();

    // Read the "file's" contents into a vector
    let mut out = Vec::new();
    c.read_to_end(&mut out).unwrap();

    log!("Sends array back");
    out
}

pub fn load(_array: &[u8], _deg: u16) -> Vec<u8> {
    let mut img = load_image_from_array(_array);

    img = match _deg {
        90 => img.rotate90(),
        180 => img.rotate180(),
        270 => img.rotate270(),
        _ => img,
    };

    get_image_as_array(img)
}

pub fn grayscale(_array: &[u8]) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.grayscale();
    get_image_as_array(img)
}

pub fn invert(_array: &[u8]) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img.invert();
    get_image_as_array(img)
}


pub fn blur(_array: &[u8], _sigma: f32) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.blur(_sigma);
    get_image_as_array(img)
}


pub fn unsharpen(_array: &[u8], _sigma: f32, _threshold: i32) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.unsharpen(_sigma, _threshold);
    get_image_as_array(img)
}


pub fn adjust_contrast(_array: &[u8], _contrast: f32) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.adjust_contrast(_contrast);
    get_image_as_array(img)
}


pub fn brighten(_array: &[u8], _value: i32) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.brighten(_value);
    get_image_as_array(img)
}


pub fn hue_rotate(_array: &[u8], _value: i32) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = img.huerotate(_value);
    get_image_as_array(img)
}


pub fn flip(_array: &[u8], _axis: u8) -> Vec<u8> {
    let mut img = load_image_from_array(_array);
    img = match _axis {
        0 => img.fliph(),
        1 => img.flipv(),
        _ => img,
    };
    get_image_as_array(img)
}


pub fn crop(_array: &[u8], _start_x: u32, _start_y: u32, _end_x: u32, _end_y: u32) -> Vec<u8> {
    log!("Received buffer");
    let mut img = load_image_from_array(_array);
    img = img.crop(_start_x, _start_y, _end_x, _end_y);
    get_image_as_array(img)
}


pub fn resize(
    _array: &[u8],
    _width: u32,
    _height: u32,
    _filter: u8,
    _aspect_ratio_preserve: bool,
    _fill: bool,
    _as_thumbnail: bool,
) -> Vec<u8> {
    log!("Received buffer");

    let _checked_filter: image::FilterType = match _filter {
        0 => image::FilterType::Nearest,
        1 => image::FilterType::Lanczos3,
        2 => image::FilterType::Gaussian,
        3 => image::FilterType::CatmullRom,
        4 => image::FilterType::Triangle,
        _ => image::FilterType::Nearest,
    };

    let mut img = load_image_from_array(_array);

    if _aspect_ratio_preserve {
        if _as_thumbnail {
            img = img.thumbnail(_width, _height);
        } else {
            if _fill {
                img = img.resize_to_fill(_width, _height, _checked_filter);
            } else {
                img = img.resize(_width, _height, _checked_filter);
            }
        }
    } else {
        if _as_thumbnail {
            img = img.thumbnail_exact(_width, _height);
        } else {
            img = img.resize_exact(_width, _height, _checked_filter);
        }
    };

    get_image_as_array(img)
}
