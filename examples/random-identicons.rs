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
extern crate rand;
extern crate identicon;

use identicon::*;
use std::fs::File;
use std::path::Path;


fn main() {

    // one static hash to make the output comparable
    let mut pk: [u8; 32] = [
        190, 94, 7, 2, 219, 8, 181, 85, 72, 201, 209, 255, 113, 106, 161, 39, 4,
        198, 174, 163, 126, 121, 255, 255, 255, 227, 69, 62, 220, 152, 128, 102
    ];

    let img = pk_to_image(&pk, 80).unwrap();

    // Save the image as out.png
    let ref mut outfile = File::create(&Path::new("out.png")).unwrap();

    // indicate the image's color type and what format to save as
    drop(img.save(outfile, image::PNG));

    for pic in 0..32 {
        // generate some data
        for x in pk.iter_mut() {
            *x = rand::random();
        }

        let imgx = pk_to_image(&pk, 80).unwrap();
        let filename = format!("out{}.png", pic);
        // Save the image as out.png
        let ref mut outfilex = File::create(&Path::new(&filename)).unwrap();

        drop(imgx.save(outfilex, image::PNG));
    }
}
