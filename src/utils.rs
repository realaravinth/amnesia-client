use std::fs;
use std::io;

pub async fn upload() {
    // Todo use actix client
}

pub async fn setup() -> Result<(), io::Error> {
    fs::DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/http-stream")?;
    Ok(())
}

pub async fn cleanup() -> Result<(), io::Error> {
    use std::fs::remove_dir_all;
    remove_dir_all("/tmp/amnesia/")?;
    Ok(())
}
