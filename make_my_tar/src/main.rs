use futures::StreamExt;
use tokio::fs::File;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("file1.txt").await?;
    let mut entries = tokio_tar::Archive::new(file).entries().unwrap();

    while let Some(entry) = entries.next().await {
        println!("{:?}", entry)
    }

    Ok(())
}
