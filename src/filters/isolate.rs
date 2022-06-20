use super::submodule_prelude::*;


fn isolate_red(input: DynamicImage) -> DynamicImage {
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


fn isolate_green(input: DynamicImage) -> DynamicImage {
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


fn isolate_blue(input: DynamicImage) -> DynamicImage {
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


/// Function handling the registration of this submodule's [`Filter`] functions.
#[doc(hidden)]
pub fn register_filters() {
    register_filter(String::from("-iso_red"), isolate_red);
    register_filter(String::from("-iso_green"), isolate_green);
    register_filter(String::from("-iso_blue"), isolate_blue);
}