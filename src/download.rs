use std::cmp::min;

use Byte;

/// Represents a range between two Byte types
#[derive(Debug, PartialEq)]
struct RangeBytes(Byte, Byte);

/// Function to get the current chunk length, based on the chunk index.
fn get_chunk_length(chunk_index: Byte,
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