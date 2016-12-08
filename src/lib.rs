/*
    Copyright © 2016 sudden6 <sudden6@gmx.at>

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU General Public License as published by
    the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU General Public License for more details.

    You should have received a copy of the GNU General Public License
    along with this program.  If not, see <http://www.gnu.org/licenses/>.
*/

extern crate image;
extern crate hsl;

use image::{
    DynamicImage,
    ImageBuffer
};

use hsl::HSL;

// specifies how many bytes should define the foreground color, must be smaller than 8, else there'll be overflows
const IDENTICON_COLOR_BYTES: u8 = 7;

// number of colors to use for the identicon
const COLORS: usize = 2;

// specifies how many rows of blocks the identicon should have
const IDENTICON_ROWS: u8 = 5;
// width from the center to the outside, for 5 columns it's 3, 6 -> 3, 7 -> 4
const ACTIVE_COLS: u8 = (IDENTICON_ROWS + 1)/2;

// min length of the hash in bytes, 7 bytes control the color,
// the rest controls the pixel placement
const HASH_MIN_LEN: u8 = ACTIVE_COLS*IDENTICON_ROWS + (COLORS as u8)*IDENTICON_COLOR_BYTES;

#[derive(Debug)]
pub enum ErrorConvert {
    HashTooShort,
    InvalidSize
}

/**
Returns the RGB colors of a HSL color. Expects all HSL values in the range 0.0 - 1.0
*/
fn hsl2rgb(h: f64, s: f64, l: f64) -> [u8; 3] {
    let rgb = (HSL { h: 360_f64 * h as f64, s: s, l: l }).to_rgb();
    [rgb.0, rgb.1, rgb.2]
}

fn normalize(value: u64, bytes: u8) -> f64 {
    value as f64 / ((1_i64 << (8 * (bytes - 1))) as  f64) // normalize to 0.0 ... 1.0
}

fn bytes_to_color(bytes: &[u8]) -> f64 {

    if bytes.len() == IDENTICON_COLOR_BYTES as usize {
        println!("hash_color:{:?}", bytes);
        // get foreground color
        let mut fg_hue: u64 = bytes[0] as u64;

        // convert the last bytes to an uint
        for x in 1..(IDENTICON_COLOR_BYTES as usize - 1) {
            fg_hue = fg_hue << 8;
            fg_hue += bytes[x] as u64;
        }

        return normalize(fg_hue, IDENTICON_COLOR_BYTES)
    }
    0.0
}

pub fn pk_to_image(hash: &[u8], size_factor: u16) -> Result<DynamicImage, ErrorConvert> {
    if hash.len() < HASH_MIN_LEN as usize {
        return Err(ErrorConvert::HashTooShort)
    }
    
    if size_factor < 1 {
        return Err(ErrorConvert::InvalidSize)
    }

    println!("hash: {:?}", hash);

    // length of one image side in pixels, must be divisible by 8
    let img_side: u32 = IDENTICON_ROWS as u32 * size_factor as u32;

    let mut colors: [[u8; 3]; COLORS] = [[0, 0, 0]; COLORS];

    for color_index in 0..COLORS
    {
        let hash_part = &hash[hash.len() - (color_index + 1) * IDENTICON_COLOR_BYTES as usize.. (hash.len() - color_index * IDENTICON_COLOR_BYTES as usize)];

        let hue = bytes_to_color(hash_part);
        let lig = (color_index as f64)*0.5 + 0.3;
        let sat = 0.5;
        colors[color_index] = hsl2rgb(hue, sat, lig);
    }

    println!("colors: {:?}", colors);

    // println!("fg color {:?}, bg color {:?}", fg_color, bg_color);

    let mut color_map = [[&colors[0]; ACTIVE_COLS as usize]; IDENTICON_ROWS as usize];


    // compute the block colors from the hash
    for x in 0..(IDENTICON_ROWS * ACTIVE_COLS) as usize
    {
        let row = x % (IDENTICON_ROWS as usize);
        let col = x / (IDENTICON_ROWS as usize);
        let col_index = (hash[x] as usize % COLORS) as usize;

        color_map[row][col] = &colors[col_index];
    }


    let mut img = ImageBuffer::new(img_side as u32, img_side as u32);

    //println!("{:?}", color_map);

    // draw a picture from the color_map
    for (x, y, pixel) in img.enumerate_pixels_mut() {
        let row: usize = (y / (size_factor as u32)) as usize;
        let col_tm: usize = (x / (size_factor as u32)) as usize;
        let col: usize = ((col_tm as isize *2 - (IDENTICON_ROWS as isize - 1))/2).abs() as usize; // mirror on vertical axis

        *pixel = image::Rgb(*color_map[row][col]);
    }
    Ok(image::ImageRgb8(img))
}
