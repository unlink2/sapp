use std::path::PathBuf;

use image::{imageops, DynamicImage, GenericImage, SubImage, GenericImageView};

use crate::error::Error;

/// A wrapper around image processing
#[derive(Clone, Debug, Default)]
pub struct AtlasImage {
    image: DynamicImage,
}

impl AtlasImage {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            image: match image::open(&path) {
                Ok(image) => image,
                Err(_) => return Err(Error::ImageError(path)),
            },
        })
    }

    pub fn save(path: PathBuf) -> Result<Self, Error> {
        todo!()
    }

    pub fn is_in_bounds(pos: (usize, usize), size: (usize, usize)) -> bool {
        false
    }

    /// Copy a tile from x to y
    pub fn copy_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error> {
        todo!()
    }

    /// Move a tile from x to y
    pub fn move_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error> {
        todo!()
    }

    /// Rotate a tile
    pub fn rotate180(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        let rotated = imageops::rotate180(&self.crop(pos, size)?.to_image());
        // TODO convert error 
        self.image.copy_from(&rotated, pos.0 as u32, pos.1 as u32);
        todo!()
    }

    pub fn rotate270(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    pub fn rotate90(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    pub fn hflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    pub fn vflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    fn crop(
        &self,
        pos: (usize, usize),
        size: (usize, usize),
    ) -> Result<SubImage<&DynamicImage>, Error> {
        Ok(self.image.view(
            pos.0 as u32,
            pos.1 as u32,
            size.0 as u32,
            size.1 as u32,
        ))
    }
}
