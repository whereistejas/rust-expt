use std::cmp::Ordering;

use bollard::{
    image::{ListImagesOptions, RemoveImageOptions},
    Docker,
};

#[tokio::main]
async fn main() {
    let docker = Docker::connect_with_local_defaults().unwrap();

    let all_images = docker
        .list_images(None::<ListImagesOptions<String>>)
        .await
        .unwrap();

    // name, version, id
    let mut airbye_images = all_images
        .iter()
        .filter_map(|summary| {
            summary
                .repo_tags
                .first()
                .and_then(|image_name| {
                    image_name.split_once(":").and_then(|(name, version)| {
                        name.starts_with("airbyte")
                            .then_some((name.clone(), version.clone()))
                    })
                })
                .map(|(name, ver)| (name, ver, summary.id.clone()))
        })
        .collect::<Vec<_>>();

    airbye_images.sort_by(|a, b| match a.0.cmp(b.0) {
        Ordering::Equal => a.1.cmp(b.1),
        ord => ord,
    });

    // do not delete these images.
    let images_to_keep = airbye_images.iter().fold(
        vec![],
        |mut acc, (current_name, current_version, current_id)| {
            if let Some(idx) = acc.iter().position(|(name, _, _)| name == current_name) {
                let (_, prev_version, _) = acc.get(idx).unwrap();
                if current_version > prev_version {
                    acc[idx] = (current_name.clone(), current_version.clone(), current_id);
                }
            } else {
                acc.push((current_name.clone(), current_version.clone(), current_id))
            }

            acc
        },
    );

    for image in all_images
        .iter()
        .filter(|summary| images_to_keep.iter().all(|(_, _, id)| summary.id != **id))
    {
        let image_name = &image.repo_tags.first().unwrap();
        docker
            .remove_image(
                image_name,
                Some(RemoveImageOptions {
                    force: true,
                    ..Default::default()
                }),
                None,
            )
            .await
            .unwrap();
        println!("Removed image: {image_name}");
    }
}
