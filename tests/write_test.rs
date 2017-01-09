extern crate snatch;

#[cfg(test)]
mod test_http_versions {
    use snatch::write::write_file;
    use snatch::SChunks;

    use std::fs::File;
    use std::sync::{Arc, Mutex};

    #[test]
    fn empty_chunks_should_only_create_a_file() {
        let file_name = "tests/test_files/write_test.txt";
        let mut test_file : File = File::create(file_name).unwrap();
        let empty_chunk : SChunks = Arc::new(Mutex::new(vec![]));
        
        assert!(write_file(&mut test_file, &empty_chunk).is_ok());
        assert!(test_file.metadata().unwrap().len() == 0);
    }

    #[test]
    fn simple_hello_world_msg_should_pass() {
        let file_name = "tests/test_files/write_test.txt";
        let mut test_file : File = File::create(file_name).unwrap();
        let hello_world_msg : Vec<u8> = String::from("Hello World!").into_bytes();
        let msg_chunk : SChunks = Arc::new(Mutex::new(vec![hello_world_msg.clone()]));

        assert!(write_file(&mut test_file, &msg_chunk).is_ok());
        assert!(test_file.metadata().unwrap().len() == hello_world_msg.len() as u64);
    }

    #[test]
    fn complex_msg_should_pass() {
        let file_name = "tests/test_files/write_test.txt";
        let mut test_file : File = File::create(file_name).unwrap();
        let fst_msg : Vec<u8> = String::from("This mess").into_bytes();
        let snd_msg : Vec<u8> = String::from("age is ").into_bytes();
        let trd_msg : Vec<u8> = String::from("complex\n").into_bytes();
        let msg_chunk : SChunks = Arc::new(Mutex::new(vec![fst_msg.clone(), snd_msg.clone(), trd_msg.clone()]));

        assert!(write_file(&mut test_file, &msg_chunk).is_ok());
        assert!(test_file.metadata().unwrap().len() == (fst_msg.len() + snd_msg.len() + trd_msg.len()) as u64);
    }

}