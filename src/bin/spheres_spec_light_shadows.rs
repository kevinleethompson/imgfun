use std::f32;

mod utils;
use utils::{Vector3, Vector4};
use utils::objects::{Sphere, Material, MaterialQuality as mq, Light};
use utils::render_funcs::{cast_ray, save_ppm_image};

fn main() {
    let base_qual = vec![mq::Smooth];
    let ivory = Material::new(Vector3::new(0.4, 0.4, 0.3), Some(Vector4::new(0.6, 0.3, 0.1, 0.)), Some(1.), Some(50.), Some(base_qual.clone()));
    let red_rubber = Material::new(Vector3::new(0.3, 0.1, 0.1), Some(Vector4::new(0.9, 0.1, 0., 0.)), Some(1.), Some(10.), Some(base_qual));

    let spheres = vec![
        Sphere::new(Vector3::new(-3., 0., -16.), 2., ivory.clone()),
        Sphere::new(Vector3::new(-1.0, -1.5, -12.), 2., red_rubber.clone()),
        Sphere::new(Vector3::new( 1.5, -0.5, -18.), 3., red_rubber),
        Sphere::new(Vector3::new(7., 5., -18.), 4., ivory)
    ];
    
    let lights = vec![
        Light::new(&Vector3::new(-20., 20., 20.), &1.5),
        Light::new(&Vector3::new(30., 50., -25.), &1.8),
        Light::new(&Vector3::new(30., 20., 30.), &1.7)
    ];

    render(&spheres, &lights);
}

fn render(spheres: &Vec<Sphere>, lights: &Vec<Light>) {
    const WIDTH: usize = 800;
    const HEIGHT: usize = 600;
    const FOV: u32 = (f32::consts::PI/2.) as u32;
    let mut pix_vec = vec![Vector3::new(0.,0.,0.); WIDTH*HEIGHT];

    for j in 0..HEIGHT {
        for i in 0..WIDTH {
            let x = (2. * (i as f32 + 0.5)/WIDTH as f32 - 1.) * (FOV as f32/2.).tan() * WIDTH as f32/HEIGHT as f32;
            let y = -(2. * (j as f32 + 0.5)/HEIGHT as f32 - 1.) * (FOV as f32/2.).tan();
            let dir = Vector3::new(x, y, -1.).normalize();
            pix_vec[i+j*WIDTH] = cast_ray(&Vector3::new(0.,0.,0.), &dir, &spheres, Option::Some(lights), None, false); 
        }
    }

    save_ppm_image("spheres_spec_light_shadows.ppm", WIDTH, HEIGHT, pix_vec);
}