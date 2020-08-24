use std::io;

pub async fn upload() {
    // Todo use actix client
}

pub async fn setup() -> Result<(), io::Error> {
    use std::fs::{self, DirBuilder};
    DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/http-stream")?;
    fs::File::create("/tmp/amnesia/amnesia.pcapng").unwrap();
    Ok(())
}

pub async fn cleanup() -> Result<(), io::Error> {
    use std::fs::remove_dir_all;
    remove_dir_all("/tmp/amnesia/")?;
    Ok(())
}
