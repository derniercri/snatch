use SChunks;
use std::fs::File;
use std::io::{Error, Write};

/// This function fills the buffer of a given local file, with the content of chunks.
pub fn write_file(local_file_buf: &mut File, chunks: &SChunks) -> Result<(), Error> {

    // Get the access to the chunks
    let chunks_m = chunks.lock().unwrap();

    // For each ones, write it into the file buffer
    for chunk in chunks_m.iter() {
        match local_file_buf.write_all(chunk) {
            Ok(_) => (),
            Err(error) => return Err(error),
        }
    }

    // Return a positive result if the remote content has been saved
    Ok(())
}
