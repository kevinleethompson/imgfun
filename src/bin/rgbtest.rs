mod utils;
use utils::Vector3;
use utils::render_funcs::save_ppm_image;

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

    save_ppm_image("test.ppm", WIDTH, HEIGHT, pix_vec);
}