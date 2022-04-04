use image::{imageops, DynamicImage, GenericImage, GenericImageView, ImageBuffer, Rgba, SubImage};

use crate::error::Error;

pub trait GenericAtlasImage {
    fn save(&self, path: &str) -> Result<(), Error>;

    /// Copy a tile from x to y
    fn copy_to(
        &mut self,
        origin: (u32, u32),
        dest: (u32, u32),
        size: (u32, u32),
    ) -> Result<(), Error>;

    /// Rotate a tile
    fn rotate180(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error>;

    fn rotate270(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error>;

    fn rotate90(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error>;

    fn hflip(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error>;

    fn vflip(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error>;
}

/// A wrapper around image processing
#[derive(Clone, Debug, Default, PartialEq)]
pub struct AtlasImage {
    image: DynamicImage,
}

impl AtlasImage {
    pub fn new(path: &str) -> Result<Self, Error> {
        Ok(Self {
            image: image::open(&path)?,
        })
    }

    fn try_view(
        &self,
        pos: (u32, u32),
        size: (u32, u32),
    ) -> Result<SubImage<&DynamicImage>, Error> {
        Ok(self
            .image
            .view(pos.0 as u32, pos.1 as u32, size.0 as u32, size.1 as u32))
    }

    fn try_copy(
        &mut self,
        pos: (u32, u32),
        buffer: &ImageBuffer<Rgba<u8>, Vec<u8>>,
    ) -> Result<(), Error> {
        self.image.copy_from(buffer, pos.0, pos.1)?;
        Ok(())
    }
}
impl GenericAtlasImage for AtlasImage {
    fn save(&self, path: &str) -> Result<(), Error> {
        self.image.save(path)?;
        Ok(())
    }

    fn copy_to(
        &mut self,
        origin: (u32, u32),
        dest: (u32, u32),
        size: (u32, u32),
    ) -> Result<(), Error> {
        self.try_copy(dest, &self.try_view(origin, size)?.to_image())
    }

    fn rotate180(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error> {
        self.try_copy(
            pos,
            &imageops::rotate180(&self.try_view(pos, size)?.to_image()),
        )
    }

    fn rotate270(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error> {
        self.try_copy(
            pos,
            &imageops::rotate270(&self.try_view(pos, size)?.to_image()),
        )
    }

    fn rotate90(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error> {
        self.try_copy(
            pos,
            &imageops::rotate90(&self.try_view(pos, size)?.to_image()),
        )
    }

    fn hflip(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error> {
        self.try_copy(
            pos,
            &imageops::flip_horizontal(&self.try_view(pos, size)?.to_image()),
        )
    }

    fn vflip(&mut self, pos: (u32, u32), size: (u32, u32)) -> Result<(), Error> {
        self.try_copy(
            pos,
            &imageops::flip_vertical(&self.try_view(pos, size)?.to_image()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TILE_SIZE: (u32, u32) = (8, 8);

    /// loads the test source image
    fn test_image() -> AtlasImage {
        AtlasImage::new("./assets/source.png").unwrap()
    }

    /// loads the expected result as a GenericAtlasImage
    fn expected_image(path: &str) -> AtlasImage {
        AtlasImage::new(path).unwrap()
    }

    #[test]
    fn it_should_copy_to() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_copy.png");

        assert_eq!(Ok(()), image.copy_to((8, 8), (8, 16), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    fn it_should_hflip() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_hflip.png");

        assert_eq!(Ok(()), image.hflip((8, 8), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    fn it_should_vflip() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_vflip.png");

        assert_eq!(Ok(()), image.vflip((8, 8), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    fn it_should_rotate90() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_rotate90.png");

        assert_eq!(Ok(()), image.rotate90((8, 8), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    fn it_should_rotate180() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_rotate180.png");

        assert_eq!(Ok(()), image.rotate180((8, 8), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    fn it_should_rotate270() {
        let mut image = test_image();
        let expected = expected_image("./assets/it_should_rotate270.png");

        assert_eq!(Ok(()), image.rotate270((8, 8), TILE_SIZE));
        assert_eq!(expected, image);
    }

    #[test]
    #[should_panic]
    fn it_should_fail_if_destination_out_of_bounds() {
        let mut image = test_image();

        image.copy_to((0, 0), (30, 30), TILE_SIZE).unwrap();
    }
}
