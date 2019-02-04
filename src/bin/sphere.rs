use std::f32;

mod utils;
use utils::Vector3;
use utils::objects::{Sphere, Material};
use utils::render_funcs::save_ppm_image;

fn main() {
    render(Sphere::new(Vector3::new(-3., 0., -16.), 2., Material{ diffuse_color: Vector3::new(-3., 0., -16.) }));
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
            pix_vec[j+i*WIDTH] = cast_ray(&Vector3::new(0.,0.,0.), &dir, &sphere); 
        }
    }

    save_ppm_image("sphere.ppm", WIDTH, HEIGHT, pix_vec);
}

fn cast_ray(orig: &Vector3<f32>, dir: &Vector3<f32>, sphere: &Sphere) -> Vector3<f32> {
    let mut sphere_dist: f32 = f32::MAX;
    if !sphere.ray_intersect(orig, dir, &mut sphere_dist) {
        return Vector3::new(0.2, 0.7, 0.8); // background color
    }

    Vector3::new(0.4, 0.4, 0.3)
}