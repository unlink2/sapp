use std::fs;

use clap::Parser;

use crate::{
    error::Error,
    prelude::{Atlas, AtlasImage, GenericAtlasImage},
};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
pub struct Args {
    pub image_path: String,

    #[clap(short, long, default_value = "./config.json")]
    pub config_path: String,

    #[clap(short, long, default_value = "./out.png")]
    pub output_path: String,
}

pub fn exec(args: &[String]) -> Result<(), Error> {
    let args = Args::parse_from(args);

    let cfg_str = fs::read_to_string(args.config_path)?;

    let atlas: Atlas = serde_json::from_str(&cfg_str)?;
    let mut atlas_image = AtlasImage::new(&args.image_path)?;

    atlas.apply(&mut atlas_image)?;

    atlas_image.save(&args.output_path)?;
    Ok(())
}
