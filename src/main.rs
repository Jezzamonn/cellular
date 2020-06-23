extern crate image;

use image::{GenericImageView, GrayImage, ImageBuffer};

const SIZE: u32 = 200;
const DECAY: u8 = 2;
const MAKE_ALIVE_THRESHOLD: u8 = 200;
const COME_ALIVE_THRESHOLD: u8 = 100;

const DIRECTIONS: [(i32, i32); 4] = [(0, -1), (-1, 0), (1, 0), (0, 1)];

fn next_generation(current: &GrayImage, next: &mut GrayImage) {
    for ((x, y, pixel), (_, _, current_pixel)) in
        next.enumerate_pixels_mut().zip(current.enumerate_pixels())
    {
        let next_pixel = if current_pixel[0] > DECAY {
            current_pixel[0] - DECAY
        } else {
            0
        };
        *pixel = image::Luma([next_pixel]);

        // TODO: Consider refactoring to remove branching
        if current_pixel[0] <= COME_ALIVE_THRESHOLD {
            for (dx, dy) in DIRECTIONS.iter() {
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;

                if !current.in_bounds(nx, ny) {
                    continue;
                }

                if current.get_pixel(nx, ny)[0] < MAKE_ALIVE_THRESHOLD {
                    continue;
                }

                // Make it alive!
                *pixel = image::Luma([255u8]);
                break;
            }
        }
    }
}

fn main() {
    let mut img1: GrayImage = ImageBuffer::new(SIZE, SIZE);
    let mut img2: GrayImage = ImageBuffer::new(SIZE, SIZE);

    *img1.get_pixel_mut(SIZE / 2, SIZE / 2) = image::Luma([255u8]);

    for i in 0..205 {
        next_generation(&img1, &mut img2);
        // Swap the two
        let tmp = img2;
        img2 = img1;
        img1 = tmp;

        if i > 200 {
            img1.save(format!("export/generate{}.png", i)).unwrap();
        }
    }
}
