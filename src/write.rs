use {SChunks};
use std::io::Error;
use std::fs::File;
use std::io::Write;

/// This function takes as parameter a file name and a vector of chaunks.
/// The function will create a file, and append to the buffer each chunk.
pub fn write_file(name_file: &str, chunks: &SChunks) -> Result<(), Error> {
    
    // Create the file - check if it doesn't exists
    let mut local_file = File::create(name_file).unwrap();

    // Get the access to the chunks
    let chunks_m = chunks.lock().unwrap();

    // For each ones, write it into the file buffer
    for chunk in chunks_m.iter() {
        match local_file.write_all(chunk) {
            Ok(_) => (),
            // Exit if there is an error
            Err(error) => return Err(error),
        }
    }

    // Return a positive result if the remote content has been saved    
    Ok(())
}