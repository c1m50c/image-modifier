use image::{DynamicImage, GenericImageView};
use image::{RgbImage, Rgb};


/// Returns the `input` as a greyscaled version of its self.
#[inline]
pub fn greyscale(input: DynamicImage) -> DynamicImage {
    return input.grayscale();
}


/// Returns the `input` with all colors equaling their inverse.
#[inline]
pub fn invert(input: DynamicImage) -> DynamicImage {
    let mut img = input.clone(); img.invert();
    return img;
}


/// Excludes all colors within the `input` besides the red channel.
#[inline]
pub fn red(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let red_value = input.get_pixel(x, y).0[0];
            output.put_pixel(x, y, Rgb([ red_value, 0, 0 ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}


/// Excludes all colors within the `input` besides the green channel.
#[inline]
pub fn green(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let green_value = input.get_pixel(x, y).0[1];
            output.put_pixel(x, y, Rgb([ 0, green_value, 0 ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}


/// Excludes all colors within the `input` besides the blue channel.
#[inline]
pub fn blue(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let blue_value = input.get_pixel(x, y).0[2];
            output.put_pixel(x, y, Rgb([ 0, 0, blue_value ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}