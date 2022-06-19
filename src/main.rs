use indicatif::{ProgressBar, ProgressStyle};
use image as img;

use std::time::Duration;
use std::env;

pub mod filters;

mod flags;

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
    let mut args = env::args().skip(1);
    filters::register_filters();
    
    let input_path = args.next()
        .expect("Expected an Input Path parameter at position 1.");
    
    let output_path = args.next()
        .expect("Expected an Output Path parameter at position 2.");

    let flags = flags::get_flags(args.collect());

    let input_image = img::open(input_path)
        .unwrap_or_else(|err| {
            panic!("Failed to open image from `input_path`, error {:?}", err)
        });

    let progress_indicator = create_progress_indicator(
        String::from("ProgressIndicator")
    );

    
    progress_indicator.set_style( ProgressStyle::default_spinner().template("{msg:.bold.green}") );
    progress_indicator.finish_with_message("Finished!");
}
