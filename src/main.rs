use std::env;


fn main() {
    let mut args = env::args().skip(1);

    let input_img_path = args.next()
        .expect("Expected an Input Image File Path argument.");
    let output_img_path = args.next()
        .expect("Expected an Output Image File Path argument.");
    let _filter_names = args.collect::<Vec<_>>();

    let img = image::open(input_img_path)
        .unwrap_or_else(|err| {
            panic!("Failed to open Image from `input_img_path` | Error {:?}", err)
        });
    
    img.save(output_img_path)
        .unwrap_or_else(|err| {
            panic!("Failed to save Image from `output_img_path` | Error {:?}", err)
        });
}