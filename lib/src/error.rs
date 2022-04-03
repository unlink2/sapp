use std::path::PathBuf;


#[derive(Debug, Clone)]
pub enum Error {
    CoordinateOutOfBounds((usize, usize)),
    ImageError(PathBuf) 
}


