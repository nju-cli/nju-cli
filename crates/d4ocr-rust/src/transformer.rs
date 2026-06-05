use enum_dispatch::enum_dispatch;
use image::imageops::{crop, resize, FilterType};
use serde::{Deserialize, Serialize};
use tract_onnx::prelude::{tract_ndarray, tract_ndarray::Ix4, Tensor};
use tract_onnx::tract_core::ndarray::Array;

use super::ImageTransformResult;
use super::{FilterOption, ImageSize, ResizeGrayImage, ToArray, ToTensor};

#[enum_dispatch]
#[derive(Clone, Serialize, Deserialize)]
pub enum ImageTransform {
    ResizeGrayImage(ResizeGrayImage),
    ResizeGrayImageAspectRatio(ResizeGrayImageAspectRatio),
    CenterCrop(CenterCrop),
    Normalization(Normalization),
    Transpose(Transpose),
    ToArray(ToArray),
    ToTensor(ToTensor),
}

#[enum_dispatch(ImageTransform)]
pub trait GenericTransform {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str>;
}

impl GenericTransform for ResizeGrayImage {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(image) => Ok(resize(
                &image,
                self.image_size.width as u32,
                self.image_size.height as u32,
                FilterType::CatmullRom,
            )
            .into()),
            ImageTransformResult::Tensor(_) => Err("Image resize not implemented for Tensor"),
            ImageTransformResult::Array4(_) => Err("Image resize not implemented for Array4"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct ResizeGrayImageAspectRatio {
    image_size: ImageSize,
    scale: f32,
    #[serde(with = "FilterOption")]
    filter: FilterType,
}

impl GenericTransform for ResizeGrayImageAspectRatio {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(image) => {
                let (height, width) = image.dimensions();
                let height = height as f32;
                let width = width as f32;
                let new_height = 100.0 * (self.image_size.height as f32) / self.scale;
                let new_width = 100.0 * (self.image_size.width as f32) / self.scale;

                let (final_height, final_width) = if height > width {
                    (new_width, new_height * height / width)
                } else {
                    (new_width * width / height, new_width)
                };

                Ok(resize(&image, final_width as u32, final_height as u32, self.filter).into())
            }
            ImageTransformResult::Tensor(_) => Err("Image resize not implemented for Tensor"),
            ImageTransformResult::Array4(_) => Err("Image resize not implemented for Array4"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct CenterCrop {
    crop_size: ImageSize,
}

impl GenericTransform for CenterCrop {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(image) => {
                let (height, width) = image.dimensions();
                let left = (width - self.crop_size.width as u32) / 2;
                let top = (height - self.crop_size.height as u32) / 2;
                let mut image_cropped = image;
                let image_cropped_new = crop(
                    &mut image_cropped,
                    top,
                    left,
                    self.crop_size.width as u32,
                    self.crop_size.height as u32,
                );
                Ok(image_cropped_new.to_image().into())
            }
            ImageTransformResult::Tensor(_) => Err("Image resize not implemented for Tensor"),
            ImageTransformResult::Array4(_) => Err("Image resize not implemented for Array4"),
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Normalization {
    sub: [f32; 3],
    div: [f32; 3],
    zeroone: bool,
}

impl GenericTransform for Normalization {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(_) => Err("Not implemented"),
            ImageTransformResult::Tensor(_) => Err("Not implemented"),
            ImageTransformResult::Array4(arr) => {
                let sub = Array::from_shape_vec((1, 3, 1, 1), self.sub.to_vec())
                    .expect("Wrong conversion to array");
                let div = Array::from_shape_vec((1, 3, 1, 1), self.div.to_vec())
                    .expect("Wrong conversion to array");
                let new_arr = if self.zeroone {
                    (arr / 255.0 - sub) / div
                } else {
                    (arr - sub) / div
                };
                Ok(ImageTransformResult::Array4(new_arr))
            }
        }
    }
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Transpose {
    axes: [usize; 4],
}

impl GenericTransform for Transpose {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(_) => Err("Not implemented"),
            ImageTransformResult::Array4(arr) => {
                let arr = arr.permuted_axes(self.axes);
                Ok(ImageTransformResult::Array4(arr))
            }
            ImageTransformResult::Tensor(tensor) => {
                // note that the same operation on Tensor is not safe as it is on Array4
                let tensor = tensor
                    .permute_axes(&self.axes)
                    .expect("Transpose should match the shape of the tensor");
                Ok(ImageTransformResult::Tensor(tensor))
            }
        }
    }
}

impl GenericTransform for ToArray {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(image) => {
                let shape = image.dimensions();
                let arr = tract_ndarray::Array4::from_shape_fn(
                    (1_usize, 1_usize, shape.1 as usize, shape.0 as usize),
                    |(_, c, y, x)| ((image[(x as _, y as _)][c] as f32 / 255.) - 0.5) / 0.5,
                );
                Ok(ImageTransformResult::Array4(arr))
            }
            ImageTransformResult::Tensor(tensor) => {
                let dyn_arr = tensor
                    .into_array::<f32>()
                    .expect("Cannot convert tensor to Array4");
                let arr4 = dyn_arr
                    .into_dimensionality::<Ix4>()
                    .expect("Cannot convert dynamic Array to Array4");
                Ok(ImageTransformResult::Array4(arr4))
            }
            ImageTransformResult::Array4(arr4) => {
                // already an array
                Ok(ImageTransformResult::Tensor(arr4.into()))
            }
        }
    }
}

impl GenericTransform for ToTensor {
    fn transform(&self, input: ImageTransformResult) -> Result<ImageTransformResult, &'static str> {
        match input {
            ImageTransformResult::GrayImage(image) => {
                let shape = image.dimensions();
                let tensor: Tensor = tract_ndarray::Array4::from_shape_fn(
                    (1_usize, 1_usize, shape.0 as usize, shape.1 as usize),
                    |(_, c, y, x)| image[(x as _, y as _)][c] as f32,
                )
                .into();
                Ok(ImageTransformResult::Tensor(tensor))
            }
            ImageTransformResult::Tensor(tensor) => {
                // already a tensor
                Ok(ImageTransformResult::Tensor(tensor))
            }
            ImageTransformResult::Array4(arr4) => Ok(ImageTransformResult::Tensor(arr4.into())),
        }
    }
}
