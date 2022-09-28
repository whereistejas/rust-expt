use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions, LogOutput,
    RemoveContainerOptions, StartContainerOptions,
};
use bollard::image::{CreateImageOptions, ListImagesOptions};
use bollard::Docker;

use futures::StreamExt;
use std::collections::HashMap;
use std::default::Default;

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();
    let mut images = Vec::new();

    let pull_options = Some(CreateImageOptions {
        from_image: "airbyte/source-amazon-ads:0.1.19",
        ..Default::default()
    });
    let mut response = docker.create_image(pull_options, None, None);

    for result in response.next().await {
        match result {
            Ok(_) => break,
            Err(_) => continue,
        }
    }

    let config = Config {
        cmd: Some(vec!["spec"]),
        image: Some("airbyte/source-amazon-ads:0.1.19"),
        ..Default::default()
    };

    let create_options = Some(CreateContainerOptions {
        name: "some-container",
    });

    let mut filters = HashMap::new();
    filters.insert("reference", vec!["airbyte/source-amazon-ads:0.1.19"]);

    images = docker
        .list_images(Some(ListImagesOptions {
            filters: filters.clone(),
            ..Default::default()
        }))
        .await
        .unwrap();

    while images.is_empty() {
        images = docker
            .list_images(Some(ListImagesOptions {
                filters: filters.clone(),
                ..Default::default()
            }))
            .await
            .unwrap();
    }

    let _response = docker
        .create_container(create_options, config)
        .await
        .unwrap();

    let attach_options = Some(AttachContainerOptions::<String> {
        stdin: Some(true),
        stdout: Some(true),
        stderr: Some(true),
        stream: Some(true),
        logs: Some(true),
        ..Default::default()
    });

    let AttachContainerResults { mut output, input } = docker
        .attach_container("some-container", attach_options)
        .await
        .unwrap();

    docker
        .start_container("some-container", None::<StartContainerOptions<String>>)
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

    let remove_options = Some(RemoveContainerOptions {
        force: true,
        ..Default::default()
    });
    let _ = docker
        .remove_container("some-container", remove_options)
        .await;
}
