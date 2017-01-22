use std::fs::File;
use std::io::Write;
use std::io::{Seek, SeekFrom};
use std::sync::{Arc, Mutex};

/// Structure that contains a shared file instance
pub struct OutputFileWriter {
    file: Arc<Mutex<File>>,
}

/// Structure that contains a shared file instance and the current
/// offset of this file
pub struct OutputChunkWriter {
    output: OutputFileWriter,
    offset: u64,
}

impl Clone for OutputFileWriter {
    fn clone(&self) -> OutputFileWriter {
        OutputFileWriter { file: self.file.clone() }
    }
}

impl OutputFileWriter {
    pub fn write(&mut self, offset: u64, buf: &[u8]) {
        let mut out_file = self.file.lock().unwrap();
        out_file.seek(SeekFrom::Start(offset)).expect("Error while seeking in file.");
        out_file.write_all(buf).expect("Error while writing to file.");
    }

    pub fn get_chunk_writer(&mut self, offset: u64) -> OutputChunkWriter {
        OutputChunkWriter {
            output: self.clone(),
            offset: offset,
        }
    }

    pub fn new(file: File) -> OutputFileWriter {
        OutputFileWriter { file: Arc::new(Mutex::new(file)) }
    }
}

impl OutputChunkWriter {
    pub fn write(&mut self, done_offset: u64, buf: &[u8]) {
        self.output.write(self.offset + done_offset, buf)
    }
}
