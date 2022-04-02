use std::collections::HashMap;

use crate::prelude::*;

/// This module describes a texture atlas

pub struct Atlas {
    pub id: String,
    pub width: usize,
    pub height: usize,
    pub tiles: HashMap<String, Tile>,
    pub attrs: Vec<Attributes>,

    image: AtlasImage,
}

impl Atlas {}
