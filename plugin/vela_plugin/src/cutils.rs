use color_thief::{get_color, ColorFormat};
use cxx_qt::QObject;
use image::{self, imageops::FilterType, GenericImageView};
use qt6_core::{QColor, QUrl};

#[derive(QObject, Default)]
pub struct CUtils;

impl CUtils {
    /// We will copy a file from the source to our target. This returns true on sucess.
    #[qinvokable(cpp_name = "copyFile")]
    pub fn copy_file(&self, source: &QUrl, target: &QUrl, overwrite: bool) -> bool {
        use std::fs;
        let src = source.to_string();
        let dst = target.to_string();
        if overwrite {
            let _ = std::fs::remove_file(&dst);
        }
        fs::copy(src, dst).is_ok()
    }

    /// Deletes the specified file at a given path.
    #[qinvokable(cpp_name = "deleteFile")]
    pub fn delete_file(&self, path: &QUrl) -> bool {
        std::fs::remove_file(path.to_string()).is_ok()
    }

    /// Converts a QUrl to a local file path (removes the "file://" prefix).
    #[qinvokable(cpp_name = "toLocalFile")]
    pub fn to_local_file(&self, url: &QUrl) -> String {
        let s = url.to_string();
        s.strip_prefix("file://").unwrap_or(&s).to_string()
    }

    /// Grabs the dominant color of a wallpaper. If `rescale_size` is None,
    /// this defaults to 128
    #[qinvokable(cpp_name = "getDominantColor")]
    pub fn get_dominant_color(&self, path: &str, rescale_size: Option<i32>) -> QColor {
        let size = rescale_size.unwrap_or(128).max(1) as u32;

        match image::open(path) {
            Ok(img) => {
                /// Downscaling for performance.
                /// Triangle here is a decent middle of the road quality/speed tradeoff.
                let resized = img.resize(size, size, FilterType::Triangle);

                /// Converting RGB8 (drop alpha)
                /// ColorThief works with raw RGB/RGBA buffers.
                let rgb = resized.to_rgb8();

                /// Sampling every Nth pixel,
                /// with 10 being a good speed/quality default.
                let quality = 10usize;

                match get_color(rgb.as_raw(), ColorFormat::Rgb, quality) {
                    Ok(c) => QColor::from_rgb(c.r as i32, c.g as i32, c.b as i32),
                    Err(_) => QColor::from_rgb(0, 0, 0),
                }
            }
            Err(_) => QColor::from_rgb(0, 0, 0),
        }
    }

    /// Calculating the average luminance of the wallpaper. Downscales using Triangle
    /// for efficiency then computes luminance with the standard formula:
    /// 0.2126*R + 0.7152*G + 0.0722*B
    #[qinvokable(cpp_name = "getAverageLuminance")]
    pub fn get_average_luminance(&self, path: &str, rescale_size: Option<i32>) -> f64 {
        let size = rescale_size.unwrap_or(128).max(1) as u32;
        match image::open(path) {
            Ok(img) => {
                let resized = img.resize(size, size, FilterType::Triangle);
                let mut total_luma = 0f64;
                let mut count = 0u64;
                for pixel in resized.pixels() {
                    let channels = pixel.2.to_rgb();
                    let r = channels[0] as f64 / 255.0;
                    let g = channels[1] as f64 / 255.0;
                    let b = channels[2] as f64 / 255.0;
                    // Luminance formula
                    total_luma += 0.2126 * r + 0.7152 * g + 0.0722 * b;
                    count += 1;
                }
                if count > 0 {
                    total_luma / (count as f64)
                } else {
                    0.0
                }
            }
            Err(_) => 0.0,
        }
    }
}

/// Register CUtils with the QML engine
pub fn registe() {
    cxx_qt::qml_register_type::<CUtils>("Vela", 1, 0, "CUtils");
}
