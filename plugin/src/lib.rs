use wasm_minimal_protocol::{initiate_protocol, wasm_func};

initiate_protocol!();

fn iterate(c_real: f64, c_imag: f64, max_iter: u32) -> u32 {
    let mut z_real: f64 = 0.0;
    let mut z_imag: f64 = 0.0;

    let mut iter = 0;

    while z_real * z_real + z_imag * z_imag <= 4.0 && iter < max_iter {
        let temp_real = z_real * z_real - z_imag * z_imag + c_real;
        z_imag = 2.0 * z_real * z_imag + c_imag;
        z_real = temp_real;

        iter = iter + 1;
    }

    return iter;
}

fn parse_u8_slice(data: &[u8]) -> u32 {
    return String::from_utf8(data.to_vec())
        .unwrap()
        .parse::<u32>()
        .unwrap();
}

const X_MIN: f64 = -2.0;
const X_MAX: f64 = 1.0;
const Y_MIN: f64 = -1.5;
const Y_MAX: f64 = 1.5;

#[wasm_func]
pub fn mandelbrot(res: &[u8], max_iter: &[u8]) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    let render_res = parse_u8_slice(res);
    let max_iters = parse_u8_slice(max_iter);

    let dx: f64 = (X_MAX - X_MIN) / render_res as f64;
    let dy: f64 = (Y_MAX - Y_MIN) / render_res as f64;

    for y in 0..render_res {
        for x in 0..render_res {
            let c_real = X_MIN + (x as f64) * dx;
            let c_imag = Y_MAX - (y as f64) * dy;

            let iter = iterate(c_real, c_imag, max_iters);

            let intensity: u8 = (255 * iter / max_iters) as u8;

            v.push(intensity);
            v.push(0);
            v.push(intensity);
        }
    }

    return v;
}
