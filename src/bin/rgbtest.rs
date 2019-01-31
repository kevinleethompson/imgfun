use std::io::prelude::*;
use std::fs::File;
use nalgebra::Vector3;

fn main() {
    render();
}

fn render() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    let mut pix_vec = vec![Vector3::new(0.,0.,0.); WIDTH*HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            pix_vec[j+i*WIDTH] = Vector3::new(i as f32/HEIGHT as f32, j as f32/HEIGHT as f32, 0.); 
        }
    }

    save_ppm_image(WIDTH, HEIGHT, pix_vec);
}

fn save_ppm_image(width: usize, height: usize, pixels: Vec<Vector3<f32>>) {
    // header for the RGB ppm file format
    let mut buffer = ["P3\n", &(width.to_string()), " ", &(height.to_string()), "\n255\n"].concat(); 
    // for every pixel in our image 
    for x in 0..width*height {
        for n in 0..3 {
            // convert each color val of the pixel to binary decimal number and add to str buffer
            let color_val = format!("{} ", (255. * 0f32.max(1f32.min(pixels[x][n]))) as u32);
            buffer.push_str(&color_val);
        }
        // if the last pixel written is a multiple of the width of the image (so, at the edge), start a new row;
        // else add extra space of separation between this pixel and the following pixel
        let str_to_add = if x > 0 && x % width == 0 { "\n" } else { " " };
        buffer.push_str(str_to_add);
    }

    let mut f = File::create("test.ppm").unwrap();
    f.write(buffer.as_bytes()).unwrap();
}