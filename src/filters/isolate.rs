use super::ImageFilter;

use image::{GenericImageView, GenericImage};
use image::DynamicImage;
use image::Rgba;


/// Implements an [`ImageFilter`] that isolates the red channel in the given [`DynamicImage`].
#[derive(Debug)]
pub struct IsolateRed {  }
impl ImageFilter for IsolateRed {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[0];
                output.put_pixel(x, y, Rgba([value, 0, 0, 255]));
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that isolates the green channel in the given [`DynamicImage`].
#[derive(Debug)]
pub struct IsolateGreen {  }
impl ImageFilter for IsolateGreen {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[1];
                output.put_pixel(x, y, Rgba([0, value, 0, 255]));
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that isolates the blue channel in the given [`DynamicImage`].
#[derive(Debug)]
pub struct IsolateBlue {  }
impl ImageFilter for IsolateBlue {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[2];
                output.put_pixel(x, y, Rgba([0, 0, value, 255]));
            }
        }

        return output;
    }
}