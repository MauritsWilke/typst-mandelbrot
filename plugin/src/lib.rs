use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

const MAX_ITERS: u32 = 500;

pub fn iterate(c_real: f64, c_imag: f64) -> u32 {
    let mut z_real: f64 = 0.0;
    let mut z_imag: f64 = 0.0;

    let mut iter = 0;

    while z_real * z_real + z_imag * z_imag <= 4.0 && iter < MAX_ITERS {
        let temp_real = z_real * z_real - z_imag * z_imag + c_real;
        z_imag = 2.0 * z_real * z_imag + c_imag;
        z_real = temp_real;

        iter = iter + 1;
    }

    return iter;
}

const X_MIN: f64 = -2.0;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

#[wasm_func]
pub fn mandelbrot(res: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    let stringy = String::from_utf8(res.to_vec()).unwrap();
    let render_res = stringy.parse::<i32>().unwrap();

    let dx: f64 = (X_MAX - X_MIN) / render_res as f64;
    let dy: f64 = (Y_MAX - Y_MIN) / render_res as f64;

    for y in 0..render_res {
        for x in 0..render_res {
            let c_real = X_MIN + (x as f64) * dx;
            let c_imag = Y_MAX - (y as f64) * dy;

            let iter = iterate(c_real, c_imag);

            let intensity: u8 = match iter {
                MAX_ITERS => 0,
                _ => (255 * iter / MAX_ITERS) as u8,
            };

            v.push(intensity);
            v.push(0);
            v.push(intensity);
        }
    }

    return v;
}
