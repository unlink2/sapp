use std::path::PathBuf;

use image::{imageops, DynamicImage, GenericImage, GenericImageView, SubImage};

use crate::error::Error;

pub trait GenericAtlasImage {
    fn save(&self, path: PathBuf) -> Result<(), Error>;

    fn is_in_bounds(&self, pos: (usize, usize), size: (usize, usize)) -> bool;

    /// Copy a tile from x to y
    fn copy_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error>;

    /// Move a tile from x to y
    fn move_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error>;

    /// Rotate a tile
    fn rotate180(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error>;

    fn rotate270(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error>;

    fn rotate90(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error>;

    fn hflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error>;

    fn vflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error>;
}

/// A wrapper around image processing
#[derive(Clone, Debug, Default)]
pub struct AtlasImage {
    image: DynamicImage,
}

impl AtlasImage {
    pub fn new(path: PathBuf) -> Result<Self, Error> {
        Ok(Self {
            image: image::open(&path)?,
        })
    }

    fn crop(
        &self,
        pos: (usize, usize),
        size: (usize, usize),
    ) -> Result<SubImage<&DynamicImage>, Error> {
        Ok(self
            .image
            .view(pos.0 as u32, pos.1 as u32, size.0 as u32, size.1 as u32))
    }
}
impl GenericAtlasImage for AtlasImage {
    fn save(&self, path: PathBuf) -> Result<(), Error> {
        todo!()
    }

    fn is_in_bounds(&self, pos: (usize, usize), size: (usize, usize)) -> bool {
        false
    }

    /// Copy a tile from x to y
    fn copy_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error> {
        todo!()
    }

    /// Move a tile from x to y
    fn move_to(
        &mut self,
        origin: (usize, usize),
        dest: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error> {
        todo!()
    }

    /// Rotate a tile
    fn rotate180(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        let rotated = imageops::rotate180(&self.crop(pos, size)?.to_image());
        self.image.copy_from(&rotated, pos.0 as u32, pos.1 as u32)?;
        Ok(())
    }

    fn rotate270(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    fn rotate90(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    fn hflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }

    fn vflip(&mut self, pos: (usize, usize), size: (usize, usize)) -> Result<(), Error> {
        todo!()
    }
}
