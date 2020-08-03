use async_std::task;
use async_std::fs::File;
use async_std::io::{self, Read};
use async_std::prelude::*;

async fn read_file (path: &str) -> io::Result<String> {
    let mut file = File::open(path).await?;
    let mut buffer = String::new();
    file.read_to_string(&mut buffer).await?;
    Ok(buffer)
}

fn main() {
    task::block_on(async {
        let file =read_file(".gitignore").await;
        println!("{}", file.unwrap());
    })
}