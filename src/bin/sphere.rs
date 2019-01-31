extern crate image;
extern crate nalgebra;
use std::io::prelude::*;
use std::fs::File;
use std::f32;

mod utils;
use utils::utils::Vector3;
use utils::utils::Sphere;

fn main() {
    render(Sphere::new(Vector3::new(-3_f32, 0_f32, -16_f32), 2_f32));
}

fn cast_ray(orig: Vector3<f32>, dir: Vector3<f32>, sphere: Sphere) -> Vector3<f32> {
    let sphere_dist: f32 = f32::MAX;
    if !sphere.ray_intersect(orig, dir, &sphere_dist) {
        return Vector3::new(0.2, 0.7, 0.8); // background color
    }

    Vector3::new(0.4, 0.4, 0.3)
}

fn render(sphere: Sphere) {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const FOV: u32 = (f32::consts::PI/2.0) as u32;
    let mut pix_vec = vec![Vector3::new(0_f32,0_f32,0_f32); WIDTH*HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let x =  (2.0 * (i as f32 + 0.5)/WIDTH as f32  - 1_f32) * (FOV as f32/2.).tan() * WIDTH as f32/HEIGHT as f32;
            let y = -(2.0 * (j as f32 + 0.5)/HEIGHT as f32 - 1_f32) * (FOV as f32/2.).tan();
            let dir = Vector3::new(x, y, -1_f32).normalize();
            pix_vec[j+i*WIDTH] = cast_ray(Vector3::new(0_f32,0_f32,0_f32), dir, sphere); 
        }
    }

    let mut buffer = ["P3\n", &(WIDTH.to_string()), " ", &(HEIGHT.to_string()), "\n255\n"].concat();
    for x in 0..WIDTH*HEIGHT {
        for n in 0..3 {
            buffer.push_str(&(((255.0 * 0_f32.max(1_f32.min(pix_vec[x][n]))) as u32).to_string()));
            buffer.push_str(" ");
        }
        let str_to_add = if x > 0 && x % WIDTH == 0 { "\n" } else { " " };
        buffer.push_str(str_to_add);
    }

    let mut f = File::create("sphere.ppm").unwrap();
    f.write(buffer.as_bytes()).unwrap();
}