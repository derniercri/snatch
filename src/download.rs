use {Byte, Chunk, SChunks};
use client::GetResponse;
use hyper::client::Client;
use hyper::error::Error;
use hyper::header::{ByteRangeSpec, Headers, Range};
use std::cmp::min;
use std::io::Read;
use std::thread;

/// Represents a range between two Byte types
#[derive(Debug, PartialEq)]
struct RangeBytes(Byte, Byte);

/// Function to get the current chunk length, based on the chunk index.
fn get_chunk_length(chunk_index: u64,
                    content_length: Byte,
                    global_chunk_length: Byte)
                    -> Option<RangeBytes> {

    if content_length == 0 || global_chunk_length == 0 {
        return None;
    }

    let b_range: Byte = chunk_index * global_chunk_length;

    if b_range >= (content_length - 1) {
        return None;
    }

    let e_range: Byte = min(content_length - 1,
                            ((chunk_index + 1) * global_chunk_length) - 1);

    Some(RangeBytes(b_range, e_range))

}

/// Function to get the HTTP header to send to the file server, for a chunk (specified by its index)
fn get_header_from_index(chunk_index: u64,
                         content_length: Byte,
                         global_chunk_length: Byte)
                         -> Option<(Headers, Byte)> {

    match get_chunk_length(chunk_index, content_length, global_chunk_length) {
        Some(range) => {
            let mut header = Headers::new();
            header.set(Range::Bytes(vec![ByteRangeSpec::FromTo(range.0, range.1)]));
            Some((header, range.1 - range.0))
        }
        None => None,
    }

}

/// Function to get from the server the content of a chunk.
/// This function returns a Result type - Byte if the content of the header is accessible, an Error type otherwise.
fn download_a_chunk(http_client: &Client,
                    http_header: Headers,
                    chunk_vector: &mut Chunk,
                    url: &str)
                    -> Result<Byte, Error> {

    match http_client.get_http_response_using_headers(url, http_header) {
        Ok(mut body) => {
            match body.read_to_end(chunk_vector) {
                Ok(nb_bytes) => Ok(nb_bytes as u64),
                Err(_) => Ok(0u64),
            }
        }
        Err(http_error) => Err(http_error),
    }

}

/// Function to download each chunk of a remote content (given by its URL).
/// This function takes as parameters:
/// * the remote content length,
/// * a mutable reference to share between threads, which contains each chunk,
/// * the number of chunks that contains the remote content,
/// * the URL of the remote content server.
pub fn download_chunks(content_length: u64,
                       downloaded_chunks: &SChunks,
                       nb_chunks: u64,
                       url: &str) {

    // let mut downloaded_chunks: Arc<Mutex<Chunks>> =
    //     Arc::new(Mutex::new(Chunks::with_capacity(nb_chunks as usize)));
    let global_chunk_length: u64 = (content_length / nb_chunks) + 1;
    let mut jobs = vec![];

    for chunk_index in 0..nb_chunks {

        let (http_header, chunk_length) =
            get_header_from_index(chunk_index, content_length, global_chunk_length).unwrap();
        let mut chunk_content = Chunk::with_capacity(chunk_length as usize);
        let hyper_client = Client::new();
        let url_clone = String::from(url);
        let clone_chunks = downloaded_chunks.clone();

        jobs.push(thread::spawn(move || {
            match download_a_chunk(&hyper_client, http_header, &mut chunk_content, &url_clone) {
                Ok(bytes_written) => {
                    if bytes_written > 0 {
                        let mut shared_clone_chunks = clone_chunks.lock().unwrap();
                        shared_clone_chunks.insert(chunk_index as usize, chunk_content);
                    } else {
                        panic!("The downloaded chunk {} is empty", chunk_index);
                    }
                }
                Err(error) => {
                    panic!("Canno't download the chunk {}, due to error {}",
                           chunk_index,
                           error);
                }
            }
        }));
    }

    for child in jobs {
        let _ = child.join();
    }

}

#[cfg(test)]
mod test_chunk_length {
    use super::get_chunk_length;
    use super::RangeBytes;

    #[test]
    fn wrong_content_length_parameter_should_return_none() {
        assert_eq!(None, get_chunk_length(0, 15, 0));
    }

    #[test]
    fn wrong_global_chunk_length_parameter_should_return_none() {
        assert_eq!(None, get_chunk_length(0, 0, 15));
    }

    #[test]
    fn wrong_length_parameters_should_return_none() {
        assert_eq!(None, get_chunk_length(0, 0, 0));
    }

    #[test]
    fn get_the_first_range_in_chunk() {
        assert_eq!(Some(RangeBytes(0, 249)), get_chunk_length(0, 1000, 250));
    }

    #[test]
    fn get_the_last_range_in_chunk() {
        assert_eq!(Some(RangeBytes(750, 999)), get_chunk_length(3, 1000, 250));
    }

    #[test]
    fn get_the_last_range_in_shorten_chunk() {
        assert_eq!(Some(RangeBytes(750, 997)), get_chunk_length(3, 998, 250));
    }

    #[test]
    fn wrong_index_parameter_should_return_none() {
        assert_eq!(None, get_chunk_length(4, 1000, 250));
    }

}