// https://www.colorspaceconverter.com/converter/rgb-to-srgb-linear

// https://en.wikipedia.org/wiki/SRGB
// https://registry.khronos.org/OpenGL/extensions/EXT/EXT_texture_sRGB_decode.txt
/// Converts Standard RGB Color Space to Linear Standard RGB Color Space
///
/// Outputs f64 instead of f32 for use in wgpu::Color
// TODO (BIND): Implement `#[wasm_bindgen]` and `extern "C"`
pub fn srgb_to_linear_srgb(red: u8, green: u8, blue: u8) -> (f64, f64, f64) {
    let r: f64 = f64::from(red) / 255.0;
    let g: f64 = f64::from(green) / 255.0;
    let b: f64 = f64::from(blue) / 255.0;

    let convert_color = |value: f64| -> f64 {
        if 0.04045 >= value {
            value / 12.92
        } else {
            ((value + 0.055) / 1.055).powf(2.4)
        }
    };

    (convert_color(r), convert_color(g), convert_color(b))
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_srgb_to_linear_srgb() -> () {
        // ~(0.14, 0.06, 0.27)
        assert_eq!(
            super::srgb_to_linear_srgb(104, 71, 141),
            (
                0.13843161503245183,
                0.06301001765316767,
                0.26635560480286247
            )
        );
    }
}
