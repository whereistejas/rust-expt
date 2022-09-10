use bollard::container::{
    AttachContainerOptions, AttachContainerResults, Config, CreateContainerOptions, LogOutput,
    RemoveContainerOptions, StartContainerOptions,
};
use bollard::Docker;

use futures::StreamExt;
use std::default::Default;

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let config = Config {
        cmd: Some(vec!["spec"]),
        image: Some("airbyte/source-amazon-ads:0.1.19"),
        ..Default::default()
    };

    let create_options = Some(CreateContainerOptions {
        name: "some-container",
    });

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

    let _ = docker
        .remove_container("some-container", None::<RemoveContainerOptions>)
        .await;
}
