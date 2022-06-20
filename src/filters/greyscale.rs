use super::submodule_prelude::*;
use image::Pixel;


fn greyscale(input: DynamicImage) -> DynamicImage {
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


/// Function handling the registration of this submodule's [`Filter`] functions.
#[doc(hidden)]
pub fn register_filters() {
    register_filter(String::from("-greyscale"), greyscale);
}