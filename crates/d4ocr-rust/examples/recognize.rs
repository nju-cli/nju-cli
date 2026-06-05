use std::env;

use d4ocr_rust::{ImageSize, TransformationPipeline};

fn main() {
    let args = env::args().nth(1).expect("no image path");

    let image = image::open(args).unwrap().to_luma8();
    let image_size = ImageSize {
        width: (image.width() as f32 * (64_f32 / image.height() as f32)) as usize,
        height: 64,
    };
    let model = TransformationPipeline::new(image_size);

    let result = model.recognize(image).unwrap();
    println!("{result}");
}
