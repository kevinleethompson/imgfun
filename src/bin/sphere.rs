use std::io::prelude::*;
use std::fs::File;
use std::f32;

mod utils;
use utils::utils::Vector3;
use utils::utils::Sphere;

fn main() {
    render(Sphere::new(Vector3::new(-3., 0., -16.), 2.));
}

fn cast_ray(orig: Vector3<f32>, dir: Vector3<f32>, sphere: &Sphere) -> Vector3<f32> {
    let sphere_dist: f32 = f32::MAX;
    if !sphere.ray_intersect(orig, dir, &sphere_dist) {
        return Vector3::new(0.2, 0.7, 0.8); // background color
    }

    Vector3::new(0.4, 0.4, 0.3)
}

fn render(sphere: Sphere) {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const FOV: u32 = (f32::consts::PI/2.) as u32;
    let mut pix_vec = vec![Vector3::new(0.,0.,0.); WIDTH*HEIGHT];
    
    for i in 0..HEIGHT {
        for j in 0..WIDTH {
            let x =  (2. * (i as f32 + 0.5)/WIDTH as f32  - 1.) * (FOV as f32/2.).tan() * WIDTH as f32/HEIGHT as f32;
            let y = -(2. * (j as f32 + 0.5)/HEIGHT as f32 - 1.) * (FOV as f32/2.).tan();
            let dir = Vector3::new(x, y, -1.).normalize();
            pix_vec[j+i*WIDTH] = cast_ray(Vector3::new(0.,0.,0.), dir, &sphere); 
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

    let mut f = File::create("sphere.ppm").unwrap();
    f.write(buffer.as_bytes()).unwrap();
}