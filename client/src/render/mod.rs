#[cfg(target_family = "wasm")]
use wasm_bindgen::prelude::wasm_bindgen;

/// Struct used for storing colors (usually in linear srgb)
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub struct Color {
    /// The color red
    /// 0 (no red) to 1 (fully red)
    pub red: f64,

    /// The color green
    /// 0 (no green) to 1 (fully green)
    pub green: f64,

    /// The color blue
    /// 0 (no blue) to 1 (fully blue)
    pub blue: f64,

    /// The transparency
    /// 0 (fully transparent) to 1 (fullly opaque)
    pub alpha: f64,

    /// The color space
    /// Linear is more accurate
    pub color_space: ColorSpace,
}

/// The color space a color is in
#[repr(C)]
#[derive(Debug, Clone, Copy, PartialEq)]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub enum ColorSpace {
    /// Linear Color Space
    LINEAR,

    /// Gamma Color Space
    GAMMA,
}

// https://www.colorspaceconverter.com/converter/rgb-to-srgb-linear
// https://en.wikipedia.org/wiki/SRGB
// https://registry.khronos.org/OpenGL/extensions/EXT/EXT_texture_sRGB_decode.txt
/// Converts Standard RGB Color Space to Linear Standard RGB Color Space
///
/// Outputs f64 instead of f32 for use in `wgpu::Color`
/// TODO: Research Wide Gamut
#[no_mangle]
#[must_use]
#[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn srgb_to_linear_srgb(red: u8, green: u8, blue: u8) -> Color {
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

    Color {
        red: convert_color(r),
        green: convert_color(g),
        blue: convert_color(b),
        alpha: 1.0,
        color_space: ColorSpace::LINEAR,
    }
}

/// Converts out color struct to WGPU's color struct
#[no_mangle]
#[must_use]
// #[cfg_attr(target_family = "wasm", wasm_bindgen)]
pub extern "C" fn get_wgpu_color_from_ce_color(color: Color) -> wgpu::Color {
    wgpu::Color {
        r: color.red,
        g: color.green,
        b: color.blue,
        a: color.alpha,
    }
}

#[cfg(test)]
mod tests {
    use crate::render::{Color, ColorSpace};

    #[test]
    fn test_srgb_to_linear_srgb() {
        // ~(0.14, 0.06, 0.27)
        let expected_result: Color = Color {
            red: 0.138_431_615_032_451_83,
            green: 0.063_010_017_653_167_67,
            blue: 0.266_355_604_802_862_47,
            alpha: 1.0,
            color_space: ColorSpace::LINEAR,
        };

        assert_eq!(super::srgb_to_linear_srgb(104, 71, 141), expected_result);
    }
}
