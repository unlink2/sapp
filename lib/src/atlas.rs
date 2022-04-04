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
    pub fn apply(&self, image: &mut dyn GenericAtlasImage) -> Result<(), Error> {
        self.tiles
            .iter()
            .try_for_each(|t| t.apply(image, self.size))
    }
}
