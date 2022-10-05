use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions, LogOutput,
    RemoveContainerOptions, StartContainerOptions,
};
use bollard::image::{CreateImageOptions, ListImagesOptions};
use bollard::Docker;

use bollard::service::ImageSummary;
use futures::StreamExt;
use std::collections::HashMap;
use std::default::Default;

async fn pull_image(image_name: &str) -> Result<(), bollard::errors::Error> {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let pull_options = CreateImageOptions {
        from_image: image_name,
        ..Default::default()
    };
    let mut response = docker.create_image(Some(pull_options), None, None);

    while let Some(output) = response.next().await {
        println!("{:?}", output?);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let image_name = "airbyte/source-amazon-ads:0.1.20";
    let name = format!("test-container-{}", rand::random::<u32>());

    let mut filters = HashMap::new();
    filters.insert("reference", vec![image_name]);

    let mut images_list = docker
        .list_images(Some(ListImagesOptions {
            filters: filters.clone(),
            ..Default::default()
        }))
        .await
        .unwrap();

    let is_image_present = |images: &Vec<ImageSummary>, image_name: &str| -> bool {
        images
            .iter()
            .any(|image| image.repo_tags.iter().any(|tag| tag == image_name))
    };

    let mut download_image = true;
    while !is_image_present(&images_list, image_name) || images_list.is_empty() {
        println!("Image is not present. Pulling image...");
        if download_image {
            pull_image(image_name).await.unwrap();
            download_image = false;
        }

        images_list = docker
            .list_images(Some(ListImagesOptions {
                filters: filters.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();
    }

    let config = Config {
        cmd: Some(vec!["spec"]),
        image: Some(image_name),
        ..Default::default()
    };

    let create_options = CreateContainerOptions {
        name: name.as_str(),
    };

    let _response = docker
        .create_container(Some(create_options), config)
        .await
        .unwrap();

    let attach_options = AttachContainerOptions::<String> {
        stdin: Some(true),
        stdout: Some(true),
        stderr: Some(true),
        stream: Some(true),
        logs: Some(true),
        ..Default::default()
    };

    let AttachContainerResults { mut output, .. } = docker
        .attach_container(&name, Some(attach_options))
        .await
        .unwrap();

    docker
        .start_container(&name, None::<StartContainerOptions<String>>)
        .await
        .unwrap();

    for line in output.next().await {
        match line.unwrap() {
            LogOutput::StdOut { message }
            | LogOutput::StdErr { message }
            | LogOutput::StdIn { message }
            | LogOutput::Console { message } => {
                println!("{}", std::str::from_utf8(&message).unwrap())
            }
        }
    }

    let remove_options = RemoveContainerOptions {
        force: true,
        ..Default::default()
    };
    let _ = docker.remove_container(&name, Some(remove_options)).await;
}
