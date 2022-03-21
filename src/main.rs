use indicatif::{ProgressBar, ProgressStyle};
use image as img;

use std::time::Duration;
use std::vec::Vec;
use std::env;

mod modifiers;


/// Creates a `DynamicImage` from a modified version of the `input`.
fn create_image_from_modifier(modifier: String, input: img::DynamicImage) -> img::DynamicImage {
    return match modifier.to_lowercase().as_str() {
        "greyscale" | "grayscale" | "gs" => modifiers::greyscale(input),
        "invert" | "inverse" | "inv" | "i" => modifiers::invert(input),
        "red" | "r" => modifiers::red(input),
        "green" | "g" => modifiers::green(input),
        "blue" | "b" => modifiers::blue(input),
        "alpha" | "a" => modifiers::alpha(input),
        "blur" => modifiers::blur(input),
        "red_grey" | "rg" => modifiers::red_grey(input),
        "green_grey" | "gg" => modifiers::green_grey(input),
        "blue_grey" | "bg" => modifiers::blue_grey(input),
        
        s => panic!(
            "Modifier {:?} is invalid and does not exist.", s
        ),
    };
}


/// Creates a visual progress indicator within the command line to visually show the tool has not crashed.
#[inline]
fn create_progress_indicator(message: String) -> ProgressBar {
    let bar = ProgressBar::new_spinner();
    bar.enable_steady_tick(Duration::from_millis(100).as_millis() as _);
    bar.set_message(message);
    bar.set_style(
        ProgressStyle::default_spinner()
            .template("{spinner:.bold.yellow} {msg:.yellow}")
            .tick_strings(&[
                "▏",
                "▎",
                "▍",
                "▌",
                "▋",
                "▊",
                "▉",
                "▊",
                "▋",
                "▌",
                "▍",
                "▎"
            ])
    );
    
    return bar;
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
    let modifier = args[1].clone(); // The modifier used on the input image.
    let input_path = args[2].clone(); // The path of the input image.
    let output_path = args[3].clone(); // The path where the modified input image should be saved.

    // Attempt to open the Image from the specified `input_path`.
    let input_image = img::open(input_path.clone())
        .expect(format!("Cannot open an Image from path {:?}.", input_path).as_str());
    
    let progress_indicator = create_progress_indicator(
        format!("Processing Modification {:?} on {:?}...", modifier, input_path)
    );
    
    // Attempt to modify the Image and save it's modified version to the `output_path`.
    create_image_from_modifier(modifier, input_image).save(output_path.clone())
        .expect(format!("Failed to save Image to path {:?}.", output_path).as_str());
    
    // Finish with a completion message.
    progress_indicator.set_style( ProgressStyle::default_spinner().template("{msg:.bold.green}") );
    progress_indicator.finish_with_message(format!("▉ Finished modifying the Image, saved to {:?}.", output_path));
}
