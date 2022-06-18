use image::DynamicImage;

pub use isolate::*;
pub mod isolate;


pub trait ImageFilter {
    /// Applys the [`ImageFilter`] to the `input` image, returing a modified version of `input`.
    /// 
    /// # Example
    /// ```rust
    /// use imgmod::filters::IsolateRed;
    /// ```
    fn apply(self, input: DynamicImage) -> DynamicImage;
}


#[derive(Debug)]
pub enum Filters {
    IsolateRed(isolate::IsolateRed),
    IsolateGreen(isolate::IsolateGreen),
    IsolateBlue(isolate::IsolateBlue),
}