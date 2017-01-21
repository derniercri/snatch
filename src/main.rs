extern crate ansi_term;
extern crate argparse;
extern crate hyper;
extern crate libsnatch;

use ansi_term::Colour::{Green, Yellow, Red};
use argparse::{ArgumentParser, Store, StoreTrue};
use hyper::client::Client;
use libsnatch::{Bytes, Chunks};
use libsnatch::client::GetResponse;
use libsnatch::contentlength::GetContentLength;
use libsnatch::download::download_chunks;
use libsnatch::http_version::ValidateHttpVersion;
use libsnatch::write::write_file;
use std::fs::File;
use std::io;
use std::sync::{Arc, Mutex};
use std::path::Path;
use std::process::exit;

fn main() {

    let mut file = String::from("");
    let mut threads: usize = 4;
    let mut url = String::from("");
    let mut verbose = false;

    {
        let mut argparse = ArgumentParser::new();
        argparse.set_description("Snatch, a simple, fast and interruptable download accelerator, \
                                  written in Rust.");
        argparse.refer(&mut file)
            .add_option(&["-f", "--file"],
                        Store,
                        "The local file to save the remote content file");
        argparse.refer(&mut threads)
            .add_option(&["-t", "--threads"],
                        Store,
                        "Number of threads available to download");
        argparse.refer(&mut url)
            .add_option(&["-u", "--url"], Store, "Remote content URL to download")
            .required();
        argparse.refer(&mut verbose)
            .add_option(&["-v", "--verbose"], StoreTrue, "Verbose mode");
        argparse.parse_args_or_exit();
    }

    let hyper_client = Client::new();

    // Get the first response from the server
    let client_response = hyper_client.get_head_response(&url).unwrap();

    print!("# Waiting a response from the remote server... ");

    if !client_response.version.greater_than_http_11() {
        println!("{}",
                 Yellow.bold()
                     .paint("OK (HTTP version <= 1.0 detected)"));
    } else {
        println!("{}", Green.bold().paint("OK !"));
    }

    // If no filename has been given, infer it
    if file == "" {
        file = match url.split('/').last() {
            Some(filename) => String::from(filename),
            None => String::from("index.html"),
        }
    }

    let local_path = Path::new(&file);

    if local_path.exists() {
        if local_path.is_dir() {
            panic!(Red.bold()
                .paint("[ERROR] The local path to store the remote content is already exists, \
                        and is a directory!"));
        }
        println!("{}",
                 Red.bold()
                     .paint("[WARNING] The path to store the file already exists! Do you want \
                             to override it? [y/N]"));
        {
            let mut user_input = String::new();
            io::stdin()
                .read_line(&mut user_input)
                .ok()
                .expect("[ERROR] Couldn't read line!");
            user_input = String::from(user_input.trim());
            if !(user_input == "y" || user_input == "Y") {
                exit(0);
            }
        }
    }

    let remote_content_length = match client_response.headers.get_content_length() {
        Some(remote_content_length) => remote_content_length,
        None => {
            println!("{}",
                     Red.bold()
                         .paint("[ERROR] Cannot get the content length of the remote content, \
                                 from the server."));
            exit(1);
        }
    };

    println!("# Remote content length: {:?} MB",
             (remote_content_length / 1000000) as Bytes);

    let mut core_chunks = Chunks::with_capacity(threads);

    for _ in 0..threads {
        core_chunks.push(Vec::new());
    }

    let mut shared_chunks = Arc::new(Mutex::new(core_chunks));

    download_chunks(remote_content_length,
                    &mut shared_chunks,
                    threads as u64,
                    &url);

    let mut local_file = File::create(local_path).expect("[ERROR] Cannot create a file !");

    match write_file(&mut local_file, &shared_chunks) {
        Ok(()) => println!("{}", Green.bold().paint("Chunks have been successfuly saved!")),
        Err(error) => println!("[ERROR] {}", error),
    }

}
