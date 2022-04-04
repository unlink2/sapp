use serde::{Deserialize, Serialize};

use crate::{error::Error, image::GenericAtlasImage, prelude::Attributes};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TileOp {
    Copy((u32, u32)),
    Rotate180,
    Rotate270,
    Rotate90,
    HFlip,
    VFlip,
}

impl TileOp {
    pub fn apply(
        &self,
        image: &mut dyn GenericAtlasImage,
        pos: (u32, u32),
        size: (u32, u32),
    ) -> Result<(), Error> {
        match self {
            Self::Copy(origin) => image.copy_to(*origin, pos, size),
            Self::Rotate180 => image.rotate180(pos, size),
            Self::Rotate270 => image.rotate270(pos, size),
            Self::Rotate90 => image.rotate90(pos, size),
            Self::HFlip => image.hflip(pos, size),
            Self::VFlip => image.vflip(pos, size),
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Tile {
    pub id: String,
    pub pos: (u32, u32),

    pub attrs: Vec<Attributes>,
    pub ops: Vec<TileOp>,
}

impl Tile {
    pub fn new(pos: (u32, u32)) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn add_attr(&mut self, attr: Attributes) {
        self.attrs.push(attr);
    }

    pub fn add_op(&mut self, op: TileOp) {
        self.ops.push(op);
    }

    pub fn apply(
        &self,
        image: &mut dyn GenericAtlasImage,
        tile_size: (u32, u32),
    ) -> Result<(), Error> {
        self.ops
            .iter()
            .try_for_each(|op| op.apply(image, self.pos, tile_size))
    }
}

#[cfg(test)]
mod test {
    use crate::image::AtlasImage;

    use super::*;

    #[test]
    fn it_should_apply_all_operations() {
        let mut image = AtlasImage::new("./assets/source.png").unwrap();
        let expected = AtlasImage::new("./assets/it_should_rotate270.png").unwrap();

        let mut tile = Tile::new((8, 8));
        tile.add_op(TileOp::Rotate90);
        tile.add_op(TileOp::Rotate180);
        tile.apply(&mut image, (8, 8)).unwrap();
        assert_eq!(expected, image);
    }
}
