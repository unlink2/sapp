use image::ImageError;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Image coordinates out of bounds")]
    CoordinateOutOfBounds((u32, u32)),

    #[error("Image error {source}")]
    ImageError {
        #[source]
        source: image::ImageError,
    },

    #[error("IO Error {source}")]
    IOError {
        #[source]
        source: std::io::Error
    },
    
    #[error("JSON Error {source}")]
    JSONError {
        #[source]
        source: serde_json::Error
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::CoordinateOutOfBounds(l0), Self::CoordinateOutOfBounds(r0)) => l0 == r0,
            _ => false,
        }
    }
}

impl From<ImageError> for Error {
    fn from(err: ImageError) -> Self {
        Self::ImageError { source: err }
    }
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Self::IOError { source: err }
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Self::JSONError { source: err }
    }
}

