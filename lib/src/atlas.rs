use std::{collections::HashMap, path::PathBuf};

use serde::{Serialize, Deserialize};

use crate::prelude::*;

/// This module describes a texture atlas
#[derive(Clone, Debug, Serialize, Deserialize, Default)]
pub struct Atlas {
    pub id: String,

    pub size: (usize, usize),

    pub tiles: Vec<Tile>,
    pub attrs: Vec<Attributes>,

    pub output: Option<PathBuf>,

    #[serde(skip)]
    image: AtlasImage,
}

impl Atlas {
}
