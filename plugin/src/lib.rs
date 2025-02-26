use wasm_minimal_protocol::{initiate_protocol, wasm_func};
mod render_modes;

initiate_protocol!();

fn iterate(c_real: f64, c_imag: f64, max_iter: i32) -> i32 {
    let mut z_real: f64 = 0.0;
    let mut z_imag: f64 = 0.0;

    let mut iter: i32 = 0;

    while z_real * z_real + z_imag * z_imag <= 4.0 && iter < max_iter {
        let temp_real = z_real * z_real - z_imag * z_imag + c_real;
        z_imag = 2.0 * z_real * z_imag + c_imag;
        z_real = temp_real;

        iter = iter + 1;
    }

    return iter;
}

fn parse_u8_slice(data: &[u8]) -> f64 {
    return String::from_utf8(data.to_vec())
        .unwrap()
        .parse::<f64>()
        .unwrap();
}

#[wasm_func]
pub fn mandelbrot(
    res: &[u8],
    max_iter: &[u8],
    x_trans: &[u8],
    y_trans: &[u8],
    zoom: &[u8],
    render_option: &[u8],
) -> Vec<u8> {
    let mut v: Vec<u8> = Vec::new();

    let render_res = parse_u8_slice(res);
    let max_iters = parse_u8_slice(max_iter);
    let zoom_level = parse_u8_slice(zoom);
    let x_translation = parse_u8_slice(x_trans);
    let y_translation = parse_u8_slice(y_trans);
    let render_option = match String::from_utf8(render_option.to_vec()).unwrap().as_str() {
        "rainbow" => render_modes::rainbow,
        "flipflop" => render_modes::flipflop,
        _ => render_modes::greyscale,
    };

    for y in 0..(render_res as u32) {
        for x in 0..(render_res as u32) {
            let x_coord = (4f64 * (x as f64) / render_res - 2f64) * zoom_level + x_translation;
            let y_coord = (4f64 * (y as f64) / render_res - 2f64) * zoom_level + y_translation;

            let iter = iterate(x_coord, y_coord, max_iters as i32);

            let mut greyscale = render_option(iter, max_iters as i32);

            v.append(&mut greyscale);
        }
    }

    return v;
}
