pub type Filter = fn(image::DynamicImage) -> image::DynamicImage;
pub static mut FILTERS: Vec<(String, Filter)> = Vec::new();


/// Takes a [`Filter`]'s `name` and `function` and pushes it to [`FILTERS`].
/// 
/// # Example
/// ```rust
/// use imgmod::filters::register_filter;
/// 
/// use image::{GenericImageView, GenericImage};
/// use image::DynamicImage;
/// use image::Rgba;
/// 
/// fn isolate_red(input: DynamicImage) -> DynamicImage {
///     let mut output = DynamicImage::new_rgb8(
///         input.width(), input.height()
///     );
///     
///     for x in 0 .. output.width() {
///         for y in 0 .. output.height() {
///             let value = input.get_pixel(x, y).0[0];
///             output.put_pixel(x, y, Rgba([value, 0, 0, 255]));
///         }
///     }
///     
///     return output;
/// }
/// 
/// fn register_filters() {
///     register_filter(String::from("isolate_red", isolate_red));
/// }
/// ```
pub fn register_filter(name: String, function: Filter) {
    unsafe { 
        FILTERS.push(
            (name, function)
        );
    }
}


/// Handles calling the `register_filters` functions within each submodule,
/// meant to only be called from the `main` function.
/// 
/// # Example
/// ```rust
/// use imgmod::filters::register_filters;
/// 
/// fn main() {
///     // Ensure this is called before any reference to `FILTERS`.
///     register_filters();
/// }
/// ```
#[doc(hidden)]
pub fn register_filters() {
    isolate::register_filters();
}


mod isolate;