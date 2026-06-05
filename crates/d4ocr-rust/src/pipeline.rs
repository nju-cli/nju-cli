use std::path::Path;

use image::imageops::FilterType;
use image::GrayImage;
use tract_onnx::prelude::Tensor;
use tract_onnx::prelude::*;

use super::MODEL_PATH;
use super::{GenericTransform, ImageTransform};
use super::{ImageSize, ResizeGrayImage, ToTensor};
use super::{ImageTransformResult, ToArray};

type TractSimplePlan = SimplePlan<TypedFact, Box<dyn TypedOp>, Graph<TypedFact, Box<dyn TypedOp>>>;

pub struct TransformationPipeline {
    steps: Vec<ImageTransform>,
    model: TractSimplePlan,
}

impl TransformationPipeline {
    pub fn new(image_size: ImageSize) -> Self {
        TransformationPipeline {
            steps: vec![
                ResizeGrayImage {
                    image_size: ImageSize {
                        width: image_size.width,
                        height: image_size.height,
                    },
                    filter: FilterType::CatmullRom,
                }
                .into(),
                ToArray {}.into(),
                ToTensor {}.into(),
            ],
            model: TransformationPipeline::load_model(&image_size),
        }
    }

    fn load_model(image_size: &ImageSize) -> TractSimplePlan {
        if !Path::new(MODEL_PATH).exists() {
            panic!("{MODEL_PATH} is not find");
        }
        let input_shape = tvec!(1, 1, image_size.height, image_size.width);
        let model = tract_onnx::onnx()
            .model_for_path(MODEL_PATH)
            .expect("Cannot read model")
            .with_input_fact(0, InferenceFact::dt_shape(f32::datum_type(), input_shape))
            .unwrap()
            .with_output_fact(
                0,
                InferenceFact::dt_shape(f32::datum_type(), tvec!(22, 1, 8210)),
            )
            .unwrap();
        model.into_optimized().unwrap().into_runnable().unwrap()
    }

    fn transform_image(&self, image: GrayImage) -> Result<Tensor, &'static str> {
        let mut result = ImageTransformResult::GrayImage(image);

        for step in &self.steps {
            result = step.transform(result)?;
        }

        let to_tensor = ToTensor {};
        result = to_tensor.transform(result)?;

        match result {
            ImageTransformResult::Tensor(t) => Ok(t),
            _ => Err("Should be converted to tensor already"),
        }
    }

    fn extract_features(&self, image: GrayImage) -> Result<Vec<i64>, String> {
        let image_tensor = self.transform_image(image).expect("Cannot transform image");
        let result = self
            .model
            .run(tvec!(image_tensor.into()))
            .expect("Cannot run model");
        let logits = result[0]
            .to_array_view::<f32>()
            .expect("Cannot extract logits");
        let shape = logits.shape();
        let class_count = *shape
            .last()
            .ok_or_else(|| "Empty logits shape".to_string())?;
        let features = logits
            .as_slice()
            .ok_or_else(|| "Logits are not contiguous".to_string())?
            .chunks(class_count)
            .map(|scores| {
                scores
                    .iter()
                    .enumerate()
                    .max_by(|(_, a), (_, b)| a.total_cmp(b))
                    .map(|(index, _)| index as i64)
                    .unwrap_or(0)
            })
            .collect();
        Ok(features)
    }

    pub fn recognize(&self, image: GrayImage) -> Result<String, String> {
        let vec = self.extract_features(image)?;

        let mut result = String::from("");
        let mut last_item: i64 = 0;
        for i in vec {
            if i == last_item {
                continue;
            } else {
                last_item = i
            }
            result.push_str(super::CHARSET[i as usize])
        }
        Ok(result)
    }
}
