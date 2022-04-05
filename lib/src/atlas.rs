use crate::{error::Error, prelude::*};
use serde::{Deserialize, Serialize};

/// This module describes a texture atlas
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Atlas {
    pub id: String,

    pub size: (u32, u32),

    pub tiles: Vec<Tile>,
    pub attrs: Vec<Attributes>,
}

impl Atlas {
    pub fn new(size: (u32, u32)) -> Self {
        Self {
            size,
            ..Default::default()
        }
    }

    pub fn apply(&self, image: &mut dyn GenericAtlasImage) -> Result<(), Error> {
        self.tiles
            .iter()
            .try_for_each(|t| t.apply(image, self.size))
    }

    pub fn add_tile(&mut self, tile: Tile) {
        self.tiles.push(tile);
    }

    pub fn add_attr(&mut self, attr: Attributes) {
        self.attrs.push(attr);
    }
}


#[cfg(test)] 
mod test {
    use super::*;
    use crate::{image::AtlasImage, tile::{Tile, TileOp}};

    #[test]
    fn it_should_apply_all_tiles() {
        let mut image = AtlasImage::new("./assets/source.png").unwrap();
        let expected = AtlasImage::new("./assets/it_should_apply_all.png").unwrap();

        let mut tile1 = Tile::new((1, 1));
        tile1.add_op(TileOp::Rotate90);
        tile1.add_op(TileOp::Rotate180);

        let mut tile2 = Tile::new((1, 2));
        tile2.add_op(TileOp::Copy((8, 8)));

        let mut atlas = Atlas::new((8, 8));
        atlas.add_tile(tile1);
        atlas.add_tile(tile2);

        atlas.apply(&mut image).unwrap();

        assert_eq!(expected, image);
    }
}
