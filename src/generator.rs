use std::{path::Path, ffi::OsStr, error::Error};
use image::{io::Reader as ImageReader, ImageError};

const CHAR_MAP: [char; 69] = [
    '$', '@', 'B', '%', '8', 
    '&', 'W', 'M', '#', '*', 
    'o', 'a', 'h', 'k', 'b', 
    'd', 'p', 'q', 'w', 'm', 
    'Z', 'O', '0', 'Q', 'L', 
    'C', 'J', 'U', 'Y', 'X', 
    'z', 'c', 'v', 'u', 'n', 
    'x', 'r', 'j', 'f', 't', 
    '/', '\\', '|', '(', ')', 
    '1', '{', '}', '[', ']', 
    '?', '-', '_', '\"', '+', 
    '~', '<', '>', 'i', '!', 
    'l', 'I', ';', ':', ',', 
    '^', '`', '\'', '.'
];

struct AsciiChar {
    val: char,
    intensity: u16
}

fn get_char_from_intensity(intensity: u16) -> char {
   
}

pub fn generate_ascii(filename: &str) -> Result<(), ImageError> {
    let img = ImageReader::open(filename)?.decode()?;

    for pixel in img.to_rgb32f().iter() {

        println!("{:?}", pixel);
    }

    Ok(()) 
}
