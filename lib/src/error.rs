use std::{path::PathBuf};

use image::ImageError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Image coordinates out of bounds")]
    CoordinateOutOfBounds((usize, usize)),

    #[error("Image error {source}")]
    ImageError {
        #[source]
        source: image::ImageError
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Self::ImageError {
            source:err
        }
    }
}
