use image::imageops::FilterType;
use image::GrayImage;
use serde::{Deserialize, Serialize};
use serde_with::serde_as;
use tract_onnx::prelude::{tract_ndarray::Array4, *};

use charset::CHARSET;
pub use pipeline::TransformationPipeline;
use transformer::{GenericTransform, ImageTransform};

mod charset;
mod pipeline;
mod transformer;

const MODEL_PATH: &str = "common.onnx";

#[derive(Serialize, Deserialize)]
#[serde(remote = "FilterType")]
enum FilterOption {
    Nearest,
    Triangle,
    CatmullRom,
    Gaussian,
    Lanczos3,
}

#[derive(Clone, Serialize, Deserialize, Debug)]
pub struct ImageSize {
    pub width: usize,
    pub height: usize,
}

#[serde_as]
#[derive(Clone, Serialize, Deserialize)]
pub struct ResizeGrayImage {
    image_size: ImageSize,
    #[serde(with = "FilterOption")]
    filter: FilterType,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToTensor {}

#[derive(Clone, Serialize, Deserialize)]
pub struct ToArray {}

pub enum ImageTransformResult {
    GrayImage(GrayImage),
    Array4(Array4<f32>),
    Tensor(Tensor),
}

impl From<GrayImage> for ImageTransformResult {
    fn from(rgb_image: GrayImage) -> Self {
        ImageTransformResult::GrayImage(rgb_image)
    }
}

impl From<Tensor> for ImageTransformResult {
    fn from(tensor: Tensor) -> Self {
        ImageTransformResult::Tensor(tensor)
    }
}
