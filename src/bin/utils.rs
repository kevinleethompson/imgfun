pub use nalgebra::{Vector3, Vector4};

pub mod objects {
    use super::{Vector3, Vector4};

    #[derive(Clone, Copy)]
    pub struct Matte { pub diffuse_color: Vector3<f32> }

    #[derive(Clone, Copy)]
    pub struct Smooth {
        pub albedo: Vector4<f32>,
        pub refractive_index: f32,
        pub diffuse_color: Vector3<f32>,
        pub specular_exponent: f32
    }

    #[derive(Clone, Copy)]
    pub enum Material {
        Matte(Matte),
        Smooth(Smooth)
    }

    impl Material {
        pub fn matte(color: Vector3<f32>) -> Material {
            Material::Matte( Matte { diffuse_color: color } )
        }

        pub fn smooth(a: Vector4<f32>, r: f32, color: Vector3<f32>, spec: f32) -> Material {
            Material::Smooth( Smooth { albedo: a, refractive_index: r, diffuse_color: color, specular_exponent: spec } )
        }
    }

    #[derive(Clone, Copy)]
    pub struct Light {
        pub position: Vector3<f32>,
        pub intensity: f32
    }

    impl Light {
        pub fn new(p: &Vector3<f32>, i: &f32) -> Light {
            Light { position: *p, intensity: *i }
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
    use super::{Vector3};
    use super::objects::{Sphere, Material, Light, Smooth, Matte};
    use std::f32;

    fn reflect(I: &Vector3<f32>, N: &Vector3<f32>) -> Vector3<f32> {
        return I - N * 2. * (I.dot(N));
    }

    fn refract(I: &Vector3<f32>, N: &Vector3<f32>, eta_t: &f32, eta_i: &f32) -> Vector3<f32> { // Snell's law
        let mut cosi: f32 = -(-1f32).max(1f32.min(I.dot(N)));
        if cosi < 0. { // if the ray is inside the object, swap the indices and invert the normal to get the correct result
            return refract(I, &-N, eta_i, eta_t);
        }
        let eta: f32 = eta_i / eta_t;
        let k: f32 = 1. - eta * eta * (1. - cosi * cosi);
        if k < 0. { Vector3::new(1.,0.,0.) } else { I * eta + N * (eta * cosi - k.sqrt()) }
    }

    pub fn cast_ray(orig: &Vector3<f32>, dir: &Vector3<f32>, spheres: &Vec<Sphere>, lights: Option<&Vec<Light>>, depth: Option<i32>) -> Vector3<f32> {
        let mut point = Vector3::new(0.,0.,0.);
        let mut N = Vector3::new(0.,0.,0.);
        let mut material = Material::matte(Vector3::new(0.,0.,0.));
        let depth = depth.unwrap_or(0);
        let mut diffuse_light_intensity = 0.;

        if depth > 4 || !scene_intersect(orig, dir, spheres, &mut point, &mut N, &mut material) {
            return Vector3::new(0.2, 0.7, 0.8); // background color
        }

        match material {
            Material::Matte(Matte { diffuse_color }) => {
                if let Some(light_vec) = lights {
                    for l in light_vec {
                        let light_dir: Vector3<f32> = (l.position - point).normalize();
                        diffuse_light_intensity += l.intensity * 0f32.max(light_dir.dot(&N));
                    }
                    diffuse_color * diffuse_light_intensity
                } else {
                    diffuse_color
                }
            },
            Material::Smooth(Smooth {albedo, refractive_index, diffuse_color, specular_exponent }) => {
                let mut specular_light_intesity = 0.;

                let reflect_dir = reflect(dir, &N);
                let reflect_orig = if reflect_dir.dot(&N) < 0. { point - N*1e-3 } else { point + N*1e-3 };
                let reflect_color = cast_ray(&reflect_orig, &reflect_dir, spheres, lights, Some(depth + 1));

                let refract_dir = refract(dir, &N, &refractive_index, &1.).normalize();
                let refract_orig = if refract_dir.dot(&N) < 0. { point - N*1e-3 } else { point + N*1e-3 };
                let refract_color = cast_ray(&refract_orig, &refract_dir, spheres, lights, Some(depth + 1));

                if let Some(light_vec) = lights {
                    for l in light_vec {
                        let light_dir: Vector3<f32> = (l.position - point).normalize();
                        let light_distance: f32 = (l.position - point).norm();

                        let shadow_orig: Vector3<f32> = if light_dir.dot(&N) < 0. { point - N*1e-3 } else { point + N*1e-3 }; // checking if the point lies in the shadow of the lights[i]
                        let mut shadow_point = Vector3::new(0.,0.,0.);
                        let mut shadow_N = Vector3::new(0.,0.,0.);
                        let mut tmp_material = Material::matte(Vector3::new(0.,0.,0.));

                        if scene_intersect(&shadow_orig, &light_dir, spheres, &mut shadow_point, &mut shadow_N, &mut tmp_material) && (shadow_point-shadow_orig).norm() < light_distance {
                            continue;
                        }

                        diffuse_light_intensity += l.intensity * 0f32.max(light_dir.dot(&N));
                        specular_light_intesity += 0f32.max(-reflect(&-light_dir, &N).dot(dir)).powf(specular_exponent) * l.intensity;
                    }
                    diffuse_color * diffuse_light_intensity * albedo[0] + Vector3::new(1.,1.,1.) * specular_light_intesity * albedo[1] + reflect_color * albedo[2] + refract_color * albedo[3]
                } else {
                    diffuse_color
                }
            }
        }
    }

    fn scene_intersect(orig: &Vector3<f32>, dir: &Vector3<f32>, spheres: &Vec<Sphere>, hit: &mut Vector3<f32>, N: &mut Vector3<f32>, material: &mut Material) -> bool {
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
            let mut c = pixels[x];
            let max = c[0].max(c[1].max(c[2]));
            if max > 1. { c = c * max/1.; }
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