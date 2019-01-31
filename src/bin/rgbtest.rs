extern crate image;
extern crate nalgebra;
use std::io::prelude::*;
use std::fs::File;

fn main() {
    render();
}

fn render() {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    let mut pix_vec = vec![[0_f32;3];WIDTH*HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            pix_vec[j+i*WIDTH] = [i as f32/HEIGHT as f32, j as f32/HEIGHT as f32, 0_f32]; 
        }
    }

    let mut buffer = ["P3\n", &(WIDTH.to_string()), " ", &(HEIGHT.to_string()), "\n255\n"].concat();
    for x in 0..WIDTH*HEIGHT {
        for n in 0..3 {
            buffer.push_str(&(((255_f32 * 0_f32.max(1.0_f32.min(pix_vec[x][n]))) as u32).to_string()));
            buffer.push_str(" ");
        }
        let str_to_add = if x > 0 && x % WIDTH == 0 { "\n" } else { " " };
        buffer.push_str(str_to_add);
    }

    let mut f = File::create("test.ppm").unwrap();
    f.write(buffer.as_bytes()).unwrap();
}