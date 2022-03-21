use image::{DynamicImage, GenericImageView};
use image::{RgbaImage, Rgba};
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


/// Isolates the red channel with the `input`.
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


/// Isolates the green channel with the `input`.
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


/// Isolates the blue channel with the `input`.
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


/// Converts the value of all color channels to the value of the alpha channel within the `input`.
#[inline]
pub fn alpha(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbaImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let alpha_value = input.get_pixel(x, y).0[3];
            output.put_pixel(x, y, Rgba([alpha_value, alpha_value, alpha_value, 255]));
        }
    }

    return DynamicImage::ImageRgba8(output);
}


/// Blurs the `input` image.
// TODO: Add an additional envrionment argument to control filter values.
#[inline]
pub fn blur(input: DynamicImage) -> DynamicImage {
    return input.blur(16.0);
}


/// Fills all color channels with the value of the red channel.
#[inline]
pub fn red_grey(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let red_value = input.get_pixel(x, y).0[0];
            output.put_pixel(x, y, Rgb([ red_value, red_value, red_value ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}


/// Fills all color channels with the value of the green channel.
#[inline]
pub fn green_grey(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let green_value = input.get_pixel(x, y).0[1];
            output.put_pixel(x, y, Rgb([ green_value, green_value, green_value ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}


/// Fills all color channels with the value of the blue channel.
#[inline]
pub fn blue_grey(input: DynamicImage) -> DynamicImage {
    let (width, height) = input.dimensions();
    let mut output = RgbImage::new(width, height);

    for x in 0 .. width {
        for y in 0 .. height {
            let blue_value = input.get_pixel(x, y).0[2];
            output.put_pixel(x, y, Rgb([ blue_value, blue_value, blue_value ]));
        }
    }

    return DynamicImage::ImageRgb8(output);
}