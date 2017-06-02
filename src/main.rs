extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate libsnatch;
extern crate num_cpus;

use ansi_term::Colour::{Green, Yellow, Red};
use clap::{App, Arg};
use libsnatch::download::download_chunks;
use libsnatch::write::OutputFileWriter;
use libsnatch::util::prompt_user;
use libsnatch::filesize::format_filesize;
use libsnatch::cargo_helper::get_cargo_info;
use std::fs::{File, remove_file};
use std::path::Path;
use std::process::exit;

static DEFAULT_FILENAME: &'static str = "index.html";

fn main() {

    // Parse arguments

    let argparse = App::new("Snatch")
        .about("Snatch, a simple, fast and interruptable download accelerator, written in Rust.")
        .version(crate_version!())
        .arg(Arg::with_name("file")
            .long("file")
            .short("f")
            .takes_value(true)
            .help("The local file to save the remote content file"))
        .arg(Arg::with_name("threads")
            .long("threads")
            .short("t")
            .takes_value(true)
            .help("Threads which can use to download"))
        .arg(Arg::with_name("debug")
            .long("debug")
            .short("d")
            .help("Active the debug mode"))
        .arg(Arg::with_name("force")
            .long("force")
            .help("Assume Yes to all queries and do not prompt"))
        .arg(Arg::with_name("url")
            .index(1)
            //.multiple(true)
            .required(true))
        .get_matches();

    // Get informations from arguments

    let url = argparse.value_of("url").unwrap();

    let file = argparse.value_of("file").unwrap_or_else(|| {
                                                            url.split('/')
                                                                .last()
                                                                .unwrap_or(DEFAULT_FILENAME)
                                                        });

    let threads: usize = value_t!(argparse, "threads", usize).unwrap_or(num_cpus::get_physical());

    if argparse.is_present("debug") {
        println!("# [{}] version: {}",
                 Yellow.bold().paint("DEBUG_MODE"),
                 crate_version!());
        println!("# [{}] file: {}", Yellow.bold().paint("DEBUG_MODE"), file);
        println!("# [{}] threads: {}",
                 Yellow.bold().paint("DEBUG_MODE"),
                 threads);
    }

    let local_path = Path::new(&file);

    if local_path.exists() {
        if local_path.is_dir() {
            panic!(Red.bold()
                .paint("[ERROR] The local path to store the remote content is already exists, \
                        and is a directory!"));
        }
        if !argparse.is_present("force") {
            let user_input = prompt_user(Yellow.bold(),
                                         "[WARNING] The path to store the file already exists! \
                                          Do you want to override it? [y/N]");

            if !(user_input == "y" || user_input == "Y") {
                exit(0);
            }
        } else {
            println!("{}",
                     Yellow.bold().paint("[WARNING] The path to store the file already exists! \
                                 It is going to be overriden."));
        }
    }

    let cargo_info = get_cargo_info(&url).expect("fail to parse url");
    println!("# Remote content length: {}",
             format_filesize(cargo_info.content_length));

    let local_file = File::create(local_path).expect("[ERROR] Cannot create a file !");

    local_file.set_len(cargo_info.content_length)
        .expect("[ERROR] Cannot extend file to download size!");
    let out_file = OutputFileWriter::new(local_file);

    if download_chunks(cargo_info, out_file, threads as u64, &url) {
        println!("{} Your download is available in {}",
                Green.bold().paint("Done!"),
                local_path.to_str().unwrap());
    } else {
        // If the file is not ok, delete it from the file system
        print!("{} An error occured - erasing file... ",
                Red.bold().paint("Failed!"));
        match remove_file(local_path) {
            Ok(_) => println!("done !"),
            Err(e) => println!("failed ({}) !", e)
        }
    }

}
