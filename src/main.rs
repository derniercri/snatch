extern crate ansi_term;
#[macro_use]
extern crate clap;
extern crate hyper;
extern crate libsnatch;
extern crate num_cpus;

use clap::{App, Arg};
use libsnatch::cargo_helper::get_cargo_info;
use libsnatch::download::download_chunks;
use libsnatch::filesize::format_filesize;
#[macro_use]
mod logs;
use libsnatch::util::prompt_user;
use libsnatch::write::OutputFileWriter;
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

    let file = argparse
        .value_of("file")
        .unwrap_or_else(|| url.split('/').last().unwrap_or(DEFAULT_FILENAME));

    // Check if multi-threaded download is possible
    let mut threads: usize = value_t!(argparse, "threads", usize)
        .and_then(|v| if v != 0 {
                      Ok(v)
                  } else {
                      Err(clap::Error::with_description("Cannot download a file using 0 thread",
                                                        clap::ErrorKind::InvalidValue))
                  })
        .unwrap_or(num_cpus::get_physical());

    if argparse.is_present("debug") {
        info!(&format!("version: {}", crate_version!()));
        info!(&format!("file: {}", file));
        info!(&format!("threads: {}", threads));
    }

    let local_path = Path::new(&file);

    if local_path.exists() {
        if local_path.is_dir() {
            epanic!("The local path to store the remote content is already exists, \
                        and is a directory!");
        }
        if !argparse.is_present("force") {
            let user_input = prompt_user("The path to store the file already exists! \
                                          Do you want to override it? [y/N]");
            if !(user_input == "y" || user_input == "Y") {
                exit(0);
            }
        } else {
            warning!("The path to store the file already exists! \
                                 It is going to be overriden.");
        }
    }

    let cargo_info = get_cargo_info(&url).expect("fail to parse url");
    info!(&format!("# Remote content length: {}",
                   format_filesize(cargo_info.content_length)));

    let local_file = File::create(local_path).expect("[ERROR] Cannot create a file !");

    local_file
        .set_len(cargo_info.content_length)
        .expect("Cannot extend file to download size!");
    let out_file = OutputFileWriter::new(local_file);

    // If the server does not accept PartialContent status, download the remote file
    // using only one thread
    if !cargo_info.accept_partialcontent {
        warning!("The remote server does not accept PartialContent status! \
                             Downloading the remote file using one thread.");
        threads = 1;
    }

    if download_chunks(cargo_info, out_file, threads as u64, &url) {
        ok!(&format!("Your download is available in {}",
                     local_path.to_str().unwrap()));
    } else {
        // If the file is not ok, delete it from the file system
        error!("Download failed! An error occured - erasing file... ");
        if remove_file(local_path).is_err() {
            error!("Cannot remove downloaded file!");
        }
    }

}
