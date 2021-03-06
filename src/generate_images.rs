use std::path::PathBuf;

use structopt::StructOpt;
use thiserror::Error;

use crate::converter;

mod renderer;
use renderer::Renderer;

#[derive(StructOpt)]
#[structopt()]
pub struct GenerateImagesOpts {
    #[structopt(short, long, default_value = ".")]
    output: PathBuf,
}

#[derive(Error, Debug)]
pub enum GenerateImagesError {}

pub async fn generate_images(opt: GenerateImagesOpts) -> Result<(), GenerateImagesError> {
    let renderer = renderer::new_cairo();
    for transform in converter::TransformList::new().transforms() {
        debug!("rendering image for {}", transform.full_name);

        let image = renderer.render_image(transform.as_ref());
        std::fs::write(
            opt.output
                .clone()
                .join(PathBuf::from(transform.short_name.clone() + ".jpg")),
            &image[..],
        )
        .unwrap();
    }

    Ok(())
}
