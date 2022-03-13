use bollard::image::ListImagesOptions;
use bollard::Docker;

use std::default::Default;

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let images = &docker
        .list_images(Some(ListImagesOptions::<String> {
            all: true,
            ..Default::default()
        }))
        .await
        .unwrap();

    for image in images {
        println!("{image:?}")
    }
}
