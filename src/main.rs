use image as img;

use std::vec::Vec;
use std::env;

mod modifiers;


/// Creates a `DynamicImage` from a modified version of the `input`.
fn create_image_from_modifier(modifier: String, input: img::DynamicImage) -> img::DynamicImage {
    return match modifier.to_lowercase().as_str() {
        "greyscale" | "grayscale" | "gs" | "g" => modifiers::greyscale(input),
        "invert" | "inverse" | "inv" | "i" => modifiers::invert(input),
        
        s => panic!(
            "Modifier {:?} is invalid and does not exist.", s
        ),
    };
}


fn main() {
    // Format for the envrionment arguments should be ( [MODIFIER] [INPUT_IMAGE_PATH] [OUTPUT_IMAGE_PATH] ).
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(), 4,
        "Incorrect amount of arguments passed into the application,
        argument order should be ([MODIFER] [INPUT_IMAGE_PATH] [OUTPUT_IMAGE_PATH])."
    );

    // Retrieve the environment arguments.
    let modifier = args[1].clone();
    let input_path = args[2].clone();
    let output_path = args[3].clone();

    // Attempt to open the Image from the specified `input_path`.
    let input_image = img::open(input_path.clone())
        .expect(format!("Cannot open an Image from path {:?}.", input_path).as_str());
    
    // Attempt to modify the Image and save it's modified version to the `output_path`.
    let output_image = create_image_from_modifier(modifier, input_image);
    output_image.save(output_path.clone())
        .expect(format!("Failed to save Image to path {:?}.", output_path).as_str());
}
