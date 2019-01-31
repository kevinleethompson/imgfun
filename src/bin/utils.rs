pub mod utils {

    pub use nalgebra::Vector3;

    #[derive(Clone, Copy)]
    pub struct Sphere {
        center: Vector3<f32>,
        radius: f32
    }

    impl Sphere {

        pub fn new(c: Vector3<f32>, r: f32) -> Sphere {
            Sphere {center: c, radius: r}
        }

        pub fn ray_intersect(&self, orig: Vector3<f32>, dir: Vector3<f32>, t0: &f32) -> bool {
            let mut t0 = *t0;
            let L = self.center - orig;
            let tca: f32 = L.dot(&dir);
            let d2: f32 = L.dot(&L) - tca * tca;
            if d2 > self.radius * self.radius { return false; }
            let thc: f32 = (self.radius * self.radius - d2).sqrt();
            t0 = tca - thc;
            let t1: f32 = tca + thc;
            if t0 < 0_f32 { t0 = t1; }
            if t0 < 0_f32 { return false; }
            true
        }

    }

}