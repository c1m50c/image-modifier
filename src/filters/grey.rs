use super::ImageFilter;

use image::{GenericImageView, GenericImage};
use image::DynamicImage;
use image::Pixel;
use image::Rgba;

use core::ops::Div;


/// Implements an [`ImageFilter`] that converts the [`DynamicImage`] into a greyscaled version of itself.
#[derive(Debug)]
pub struct Greyscale {  }
impl ImageFilter for Greyscale {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let pixel = input.get_pixel(x, y)
                    .to_luma_alpha().to_rgba();
                
                output.put_pixel(x, y, pixel);
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that averages the values of all color channels in the [`DynamicImage`],
/// then sets all color channel values to that average.
#[derive(Debug)]
pub struct GreyscaleAvg {  }
impl ImageFilter for GreyscaleAvg {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let avg = input.get_pixel(x, y).0[0..3]
                    .iter().sum::<u8>().div(3);
                
                output.put_pixel(x, y, Rgba([avg, avg, avg, 255]));
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that converts all color channel values in the [`DynamicImage`] to the red channel's value.
#[derive(Debug)]
pub struct GreyscaleR {  }
impl ImageFilter for GreyscaleR {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[0];
                
                output.put_pixel(x, y, Rgba([value, value, value, 255]));
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that converts all color channel values in the [`DynamicImage`] to the green channel's value.
#[derive(Debug)]
pub struct GreyscaleG {  }
impl ImageFilter for GreyscaleG {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[1];
                
                output.put_pixel(x, y, Rgba([value, value, value, 255]));
            }
        }

        return output;
    }
}


/// Implements an [`ImageFilter`] that converts all color channel values in the [`DynamicImage`] to the blue channel's value.
#[derive(Debug)]
pub struct GreyscaleB {  }
impl ImageFilter for GreyscaleB {
    fn apply(self, input: DynamicImage) -> DynamicImage {
        let mut output = DynamicImage::new_rgb8(
            input.width(), input.height()
        );

        for x in 0 .. output.width() {
            for y in 0 .. output.height() {
                let value = input.get_pixel(x, y).0[2];
                
                output.put_pixel(x, y, Rgba([value, value, value, 255]));
            }
        }

        return output;
    }
}