use {Bytes, Chunk, SChunks};
use client::GetResponse;
use hyper::client::Client;
use hyper::error::Error;
use hyper::header::{ByteRangeSpec, Headers, Range};
use pbr::{MultiBar, Pipe, ProgressBar};
use response::CheckResponseStatus;
use std::cmp::min;
use std::io::Read;
use std::thread;

/// Represents a range between two Bytes types
#[derive(Debug, PartialEq)]
struct RangeBytes(Bytes, Bytes);

/// Function to get the current chunk length, based on the chunk index.
fn get_chunk_length(chunk_index: u64,
                    content_length: Bytes,
                    global_chunk_length: Bytes)
                    -> Option<RangeBytes> {

    if content_length == 0 || global_chunk_length == 0 {
        return None;
    }

    let b_range: Bytes = chunk_index * global_chunk_length;

    if b_range >= (content_length - 1) {
        return None;
    }

    let e_range: Bytes = min(content_length - 1,
                             ((chunk_index + 1) * global_chunk_length) - 1);

    Some(RangeBytes(b_range, e_range))

}

/// Function to get the HTTP header to send to the file server, for a chunk (specified by its index)
fn get_header_from_index(chunk_index: u64,
                         content_length: Bytes,
                         global_chunk_length: Bytes)
                         -> Option<(Headers, Bytes)> {

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
/// This function returns a Result type - Bytes if the content of the header is accessible, an Error type otherwise.
fn download_a_chunk(http_client: &Client,
                    http_header: Headers,
                    chunk_vector: &mut Chunk,
                    url: &str,
                    mpb: &mut ProgressBar<Pipe>)
                    -> Result<Bytes, Error> {

    let mut body = http_client.get_http_response_using_headers(url, http_header).unwrap();
    if !body.check_partialcontent_status() {
        return Err(Error::Status);
    }
    let mut bytes_buffer = [0; 2048];
    let mut sum_bytes = 0;
    while let Ok(n) = body.read(&mut bytes_buffer) {
        if n == 0 {
            return Ok(sum_bytes);
        }
        chunk_vector.extend_from_slice(&bytes_buffer[..n]);
        sum_bytes += n as u64;
        mpb.add(n as u64);
    }
    return Ok(0u64);
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

    let mut mpb = MultiBar::new();
    mpb.println(&format!("Downloading {} chunks: ", nb_chunks));

    for chunk_index in 0..nb_chunks {

        let (http_header, chunk_length) =
            get_header_from_index(chunk_index, content_length, global_chunk_length).unwrap();
        let mut chunk_content = Chunk::with_capacity(chunk_length as usize);
        let hyper_client = Client::new();
        let url_clone = String::from(url);
        let clone_chunks = downloaded_chunks.clone();

        // Progress bar customization
        let mut mp = mpb.create_bar(chunk_length);
        mp.tick_format("▏▎▍▌▋▊▉██▉▊▋▌▍▎▏");
        mp.format("|#--|");
        mp.show_tick = true;
        mp.show_speed = false;
        mp.show_percent = true;
        mp.show_counter = false;
        mp.show_time_left = true;
        mp.message(&format!("Chunk {} ", chunk_index));

        jobs.push(thread::spawn(move || match download_a_chunk(&hyper_client,
                                                                    http_header,
                                                                    &mut chunk_content,
                                                                    &url_clone,
                                                                    &mut mp) {
            Ok(bytes_written) => {
                if bytes_written > 0 {
                    let mut shared_clone_chunks = clone_chunks.lock().unwrap();
                    shared_clone_chunks.insert(chunk_index as usize, chunk_content);
                    mp.finish();
                } else {
                    panic!("The downloaded chunk {} is empty", chunk_index);
                }
            }
            Err(error) => {
                panic!("Canno't download the chunk {}, due to error {}",
                       chunk_index,
                       error);
            }
        }));
    }

    mpb.listen();

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

#[cfg(test)]
mod test_header {

    use super::get_header_from_index;
    use hyper::header::{ByteRangeSpec, Headers, Range};

    #[test]
    fn wrong_chunk_length_should_return_none() {
        assert_eq!(None, get_header_from_index(0, 0, 0));
    }

    #[test]
    fn good_chunk_length_should_return_a_good_header() {
        let mut test_header = Headers::new();
        test_header.set(Range::Bytes(vec![ByteRangeSpec::FromTo(750, 997)]));
        assert_eq!(Some((test_header, 247)), get_header_from_index(3, 998, 250));
    }

}
