use std::io;

pub async fn upload_to_comand() -> Result<(), io::Error> {
    // Todo use actix client
    unimplemented!("use actix client");
    Ok(())
}

pub async fn process_capture() -> Result<(), io::Error> {
    unimplemented!(
        "not yet implemented, thinking either tokio::io or something based off of acixt"
    );
    Ok(())
}

pub async fn message_telegram() -> Result<(), io::Error> {
    unimplemented!(
        "use lib from copier client, also get group ID and API key from environment variable"
    );
    Ok(())
}

pub async fn setup() -> Result<(), io::Error> {
    use std::fs::{self, DirBuilder};
    DirBuilder::new()
        .recursive(true)
        .create("/tmp/amnesia/http-stream")?;
    Ok(())
}

pub async fn cleanup() -> Result<(), io::Error> {
    use std::fs::remove_dir_all;
    remove_dir_all("/tmp/amnesia/")?;
    Ok(())
}
