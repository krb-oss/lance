extern crate image;

use image::ColorType;
use image::png::PNGEncoder;
use std::fs::File;
use std::io::Result;

/// Write the buffer `pixels`, whose dimensions are given by `bounds`, to the
/// file named `filename`.
pub fn write_image(
    filename: &str,
    pixels: &[u8],
    bounds: (usize, usize)
) -> Result<()> {
    let output = File::create(filename)?;

    let encoder = PNGEncoder::new(output);
    encoder.encode(&pixels,
        bounds.0 as u32, bounds.1 as u32,
        ColorType::Gray(8))?;

    Ok(())
}
