use noise::{NoiseFn, Perlin};
use rand::{random, thread_rng, Rng};

fn grey(t: u8) -> [u8; 3] {
    return [(255 - t), (255 - t), (255 - t)];
}

fn generate_bad_noise() {
    let imgx = 255;
    let imgy = 255;
    let mut imgbuf = image::ImageBuffer::new(imgx, imgy);
    for x in 0..imgx {
        for y in 0..imgy {
            let pixel = imgbuf.get_pixel_mut(x, y);
            let t = random::<u8>();
            *pixel = image::Rgb(grey(t));
        }
    }
    imgbuf.save("bad-noise.png").unwrap();
}

fn generate_porlin_noise() {
    let perlin = Perlin::new(2);
    let imgx = 256;
    let imgy = 256;
    let gridx = 32;
    let gridy = 32;
    let imgbuf = image::ImageBuffer::from_fn(imgx, imgy, |x, y| {
        let xx = (x as f32) / (gridx as f32);
        let yy = (y as f32) / (gridy as f32);
        let mut t = perlin.get([xx as f64, yy as f64]) as f32;
        t = (t + 1.0) / 2.0;
        // dbg!(t);
        image::Rgb([
            (255.0 * (1.0 - t)) as u8,
            (255.0 * (1.0 - t)) as u8,
            (255.0 * (1.0 - t)) as u8,
        ])
    });
    imgbuf.save("perlin-noise.png").unwrap();
}

fn generate_voronoi_map() {
    let imgx = 256usize;
    let imgy = 256usize;
    let gridx = 32usize;
    // let gridy = 32usize;
    let mut rng = thread_rng();
    let mut root_points: Vec<Vec<Vec<u32>>> = vec![vec![vec![0u32; 2]; 8]; 8];
    let mut root_colors: Vec<Vec<Vec<u8>>> = vec![vec![vec![0u8; 3]; 8]; 8];
    for i in 0..8 {
        for j in 0..8 {
            let x = rng.gen_range(0..gridx) + i * gridx;
            let y = rng.gen_range(0..gridx) + j * gridx;
            root_points[i as usize][j as usize] = vec![x as u32, y as u32];
            let r: u8 = rng.gen();
            let g: u8 = rng.gen();
            let b: u8 = rng.gen();
            root_colors[i as usize][j as usize] = vec![r, g, b];
        }
    }
    // dbg!(&root_points);
    // dbg!(&root_colors);
    let distance = |a: &Vec<u32>, b: &Vec<u32>| -> f32 {
        let c = a[0] as f32 - b[0] as f32;
        let d = a[1] as f32 - b[1] as f32;
        (c * c + d * d).sqrt()
    };
    let mut imgbuf = image::ImageBuffer::new(imgx as u32, imgy as u32);
    for x in 0..imgx {
        for y in 0..imgy {
            let a = vec![x as u32, y as u32];
            let mut dis = distance(&a, &root_points[0][0]);
            let mut color = &root_colors[0][0];
            for i in 0..8 {
                for j in 1..8 {
                    let current = distance(&a, &root_points[i][j]);
                    if current < dis {
                        color = &root_colors[i][j];
                        dis = current;
                    }
                }
            }
            let pixel = imgbuf.get_pixel_mut(x as u32, y as u32);
            *pixel = image::Rgb([color[0], color[1], color[2]]);
        }
    }
    imgbuf.save("voronoi.png").unwrap();
}

fn main() {
    generate_bad_noise();
    generate_porlin_noise();
    generate_voronoi_map();
}
