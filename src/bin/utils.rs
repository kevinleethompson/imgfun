pub use nalgebra::{Vector3, normalize};

pub mod objects {
    use super::Vector3;

    #[derive(Clone, Copy)]
    pub struct Material {
        pub diffuse_color: Vector3<f32>
    }

    impl Material {
        pub fn new(color: Vector3<f32>) -> Material {
            Material { diffuse_color: color }
        }
    }


    #[derive(Clone, Copy)]
    pub struct Sphere {
        pub center: Vector3<f32>,
        pub radius: f32,
        pub material: Material
    }

    impl Sphere {

        pub fn new(c: Vector3<f32>, r: f32, m: Material) -> Sphere {
            Sphere {center: c, radius: r, material: m }
        }

        pub fn ray_intersect(&self, orig: &Vector3<f32>, dir: &Vector3<f32>, t0: &mut f32) -> bool {
            let L = self.center - orig;
            let tca: f32 = L.dot(&dir);
            let d2: f32 = L.dot(&L) - tca * tca;
            if d2 > self.radius * self.radius { return false; }
            let thc: f32 = (self.radius * self.radius - d2).sqrt();
            *t0 = tca - thc;
            let t1 = tca + thc;
            if *t0 < 0. { *t0 = t1; }
            if *t0 < 0. { return false; }
            true
        }

    }

}

pub mod render_funcs {
    use std::io::prelude::*;
    use std::fs::File;
    use super::Vector3;
    use super::objects::{Sphere, Material};
    use std::f32;

    pub fn cast_ray(orig: &Vector3<f32>, dir: &Vector3<f32>, spheres: &Vec<Sphere>) -> Vector3<f32> {
        let mut point = &mut Vector3::new(0.,0.,0.);
        let mut N = &mut Vector3::new(0.,0.,0.);
        let mut material = &mut Material::new(Vector3::new(0.,0.,0.));

        if !scene_intersect(orig, dir, spheres, &mut point, &mut N, &mut material) {
            return Vector3::new(0.2, 0.7, 0.8); // background color
        }

        material.diffuse_color
    }

    pub fn scene_intersect(orig: &Vector3<f32>, dir: &Vector3<f32>, spheres: &Vec<Sphere>, hit: &mut Vector3<f32>, N: &mut Vector3<f32>, material: &mut Material) -> bool {
        let mut spheres_dist = f32::MAX;
        for s in spheres.iter() {
            let mut dist_i = 0.;
            if s.ray_intersect(orig, dir, &mut dist_i) && dist_i < spheres_dist {
                spheres_dist = dist_i;
                *hit = orig + dir * dist_i;
                *N = (*hit - s.center).normalize();
                *material = s.material;
            }
        }
        spheres_dist < 1000.
    }

    pub fn save_ppm_image(path: &str, width: usize, height: usize, pixels: Vec<Vector3<f32>>) {
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

        let mut f = File::create(path).unwrap();
        f.write(buffer.as_bytes()).unwrap();
    }
}