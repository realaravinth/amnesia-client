use std::fs::{metadata, read_dir, read_to_string, DirBuilder};
use std::io;
use std::path::Path;

use regex::Regex;

lazy_static! {
    static ref RE: Regex = Regex::new(r"loadQuestion\(*").unwrap();
}

use crate::server::State;
pub async fn upload() -> Result<(), io::Error> {
    // Todo use actix client
    unimplemented!("use actix client");
    Ok(())
}

async fn prepare_payload<'a>() -> Result<String, io::Error> {
    let dir = Path::new("/tmp/amnesia/http-stream");
    metadata(dir).unwrap();
    let contents: String = read_dir(dir)
        .unwrap()
        .map(|file| {
            let x = file.unwrap().file_name();
            let file_name = x.to_str().unwrap();
            let mut contents = String::new();
            if RE.is_match(file_name) {
                contents = read_to_string(dir.join(file_name)).unwrap();
            }
            contents
        })
        .collect();
    Ok(contents)
}

pub async fn message_telegram() -> Result<(), io::Error> {
    unimplemented!(
        "use lib from copier client, also get group ID and API key from environment variable"
    );
    Ok(())
}

//todo!("add path to application state and make custom folders for every capture");
pub async fn setup() -> Result<(), io::Error> {
    DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/http-stream")?;
    DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/payload")?;
    DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/capture")?;

    Ok(())
}

pub async fn cleanup() -> Result<(), io::Error> {
    use std::fs::remove_dir_all;
    remove_dir_all("/tmp/amnesia/")?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future::join_all;
    use std::fs::File;
    use std::io::prelude::*;

    #[actix_rt::test]
    async fn setup_dir_creation() {
        setup().await.unwrap();
        let dirs = [
            Path::new("/tmp/amnesia"),
            Path::new("/tmp/amnesia/capture/"),
            Path::new("/tmp/amnesia/payload"),
            Path::new("/tmp/amnesia/http-stream"),
        ];
        for dir in dirs.iter() {
            assert!(dir.is_dir());
        }
        cleanup().await.unwrap();
    }

    async fn create_files(path: String, content: &[u8]) {
        let mut file = File::create(path).unwrap();
        file.write_all(content).unwrap();
    }

    #[actix_rt::test]
    async fn read_stream_to_string() {
        setup().await.unwrap();
        let mut future_handles = Vec::new();
        let no_files = [1, 2, 3, 4, 5, 6, 7, 8, 9];
        for i in no_files.iter() {
            let file_name = format!("/tmp/amnesia/http-stream/loadQuestion({})", i);
            future_handles.push(create_files(file_name, b"aa"));
        }
        join_all(future_handles).await;
        let content = prepare_payload().await.unwrap();
        print!("{}", content);
        cleanup().await.unwrap();
        assert_eq!("aaaaaaaaaaaaaaaaaa", content);
    }

    #[actix_rt::test]
    async fn cleanup_test() {
        setup().await.unwrap();
        cleanup().await.unwrap();
        let path = Path::new("/tmp/amnesia");
        assert!(!path.is_dir());
    }
}
