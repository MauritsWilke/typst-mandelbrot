pub fn greyscale(iterations: i32, max_iterations: i32) -> Vec<u8> {
    return vec![
        (255i32 * iterations / (max_iterations as i32)) as u8,
        (255i32 * iterations / (max_iterations as i32)) as u8,
        (255i32 * iterations / (max_iterations as i32)) as u8,
    ];
}

pub fn flipflop(iterations: i32, _max_iterations: i32) -> Vec<u8> {
    if iterations % 2 == 0 {
        return vec![0, 0, 0];
    } else {
        return vec![255, 255, 255];
    }
}

pub fn rainbow(iterations: i32, _max_iterations: i32) -> Vec<u8> {
    let hue = 8i32 * iterations % 1530i32;

    let colour: Vec<u8> = match hue {
        0..256 => vec![255, (hue % 255) as u8, 0],
        256..512 => vec![(255 - (hue.saturating_sub(256)) % 255) as u8, 255, 0],
        512..768 => vec![0, 255, (hue.saturating_sub(512) % 255) as u8],
        768..1024 => vec![0, 255 - (hue.saturating_sub(768) % 255) as u8, 255],
        1024..1280 => vec![(hue.saturating_sub(1024) % 255) as u8, 0, 255],
        _ => vec![255, 0, (255 - hue.saturating_sub(1280) % 255) as u8],
    };

    return colour;
}
