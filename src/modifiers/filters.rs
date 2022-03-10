use image::DynamicImage;


/// Returns the `input` as a greyscaled version of its self.
#[inline]
pub fn greyscale(input: DynamicImage) -> DynamicImage {
    return input.grayscale();
}


/// Returns the `input` with all colors equaling their inverse.
#[inline]
pub fn invert(input: DynamicImage) -> DynamicImage {
    let mut img = input.clone();
    img.invert();

    return img;
}