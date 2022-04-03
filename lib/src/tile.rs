use image::imageops::rotate270;
use serde::{Deserialize, Serialize};

use crate::{error::Error, image::AtlasImage, prelude::Attributes};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TileOp {
    Rotate180,
    Rotate270,
    Rotate90,
    HFlip,
    VFlip,
}

impl TileOp {
    pub fn apply(
        &self,
        image: &mut AtlasImage,
        pos: (usize, usize),
        size: (usize, usize),
    ) -> Result<(), Error> {
        match self {
            Self::Rotate180 => image.rotate180(pos, size),
            Self::Rotate270 => image.rotate270(pos, size),
            Self::Rotate90 => image.rotate90(pos, size),
            Self::HFlip => image.hflip(pos, size),
            Self::VFlip => image.vflip(pos, size)
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Tile {
    pub id: String,
    pub pos: (usize, usize),

    pub attrs: Vec<Attributes>,
    pub ops: Vec<TileOp>,
}

impl Tile {
    pub fn new(pos: (usize, usize)) -> Self {
        Self {
            pos,
            ..Default::default()
        }
    }

    pub fn add_attr(mut self, attr: Attributes) -> Self {
        self.attrs.push(attr);
        self
    }

    pub fn add_op(mut self, op: TileOp) -> Self {
        self.ops.push(op);
        self
    }

    pub fn apply(&self, image: &mut AtlasImage, tile_size: (usize, usize)) -> Result<(), Error> {
        self.ops
            .iter()
            .try_for_each(|op| op.apply(image, self.pos, tile_size))
    }
}
