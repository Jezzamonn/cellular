use image::{GenericImageView, GrayImage, ImageBuffer};
use minifb::{Window, WindowOptions};
use rand::prelude::*;
use std::time::Duration;


const SIZE: u32 = 200;
const DECAY: u8 = 5;
const MAKE_ALIVE_THRESHOLD: u8 = 200;
const COME_ALIVE_THRESHOLD: u8 = 100;
const ALIVE_START: u8 = 250;

const DIRECTIONS: [(i32, i32); 12] = [
    (-2, 0),
    (-1, -1),
    (-1, 0),
    (-1, 1),
    (0, -2),
    (0, -1),
    (0, 1),
    (0, 2),
    (1, -1),
    (1, 0),
    (1, 1),
    (2, 0),
];

fn next_generation(current: &GrayImage, next: &mut GrayImage) {
    let mut rng = rand::thread_rng();

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
            let mut sum: u32 = 0;
            for (dx, dy) in DIRECTIONS.iter() {
                let nx = (x as i32 + dx) as u32;
                let ny = (y as i32 + dy) as u32;

                if !current.in_bounds(nx, ny) {
                    continue;
                }

                sum += current.get_pixel(nx, ny)[0].checked_sub(MAKE_ALIVE_THRESHOLD).unwrap_or(0) as u32;
            }

            if rng.gen::<f32>() < 0.001 * sum as f32 {
                // Make it alive!
                *pixel = image::Luma([ALIVE_START]);
            }
        }
    }
}

fn main() {
    let mut current_image: GrayImage = ImageBuffer::new(SIZE, SIZE);
    let mut next_image: GrayImage = ImageBuffer::new(SIZE, SIZE);

    *current_image.get_pixel_mut(SIZE / 2, SIZE / 2) = image::Luma([ALIVE_START]);

    let mut buffer: Vec<u32> = vec![0; (SIZE * SIZE) as usize];
    let mut window = Window::new(
        "Hello!! :)",
        SIZE as usize,
        SIZE as usize,
        WindowOptions::default(),
    )
    .unwrap();

    // 60 fps
    window.limit_update_rate(Some(Duration::from_micros(16600)));

    // let mut i = 0;
    while window.is_open() {
        next_generation(&current_image, &mut next_image);
        // Swap the two
        let tmp = next_image;
        next_image = current_image;
        current_image = tmp;

        for (buf, &pixel) in buffer.iter_mut().zip(current_image.iter()) {
            let pixel = pixel as u32;
            *buf = pixel | pixel << 8 | pixel << 16
        }

        window
            .update_with_buffer(&buffer, SIZE as usize, SIZE as usize)
            .unwrap();

        // i += 1;
        // if i > 200 {
        //     img1.save(format!("export/generate{}.png", i)).unwrap();
        // }
    }
}
