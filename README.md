# snatch
A simple, fast and interruptable download accelerator, written in Rust

![Snatch logo](./img/snatch-horizontal.png)

(A special thanks to @frankirito for this awesome logo !)

## Features

* **Simple**: a command line to manage easily your downloads ;
* **Fast**: written in a new exciting programing language ;
* **Interruptable**: you can interrupt and resume easily your downloads (_**SOON**_).

**NOTE**: _Snatch_ is on _alpha_ version. This version runs well on remote contents that lenght is known **before** the download (by the `content-length` header from the server response) - also, the _Interruptable_ feature is not implemented yet.

## Installation

1. Please to install Rust and Cargo using [rustup](https://www.rustup.rs/) ;
2. Install _Snatch_: `cargo install --git https://github.com/derniercri/snatch.git` ;
3. Enjoy !

## Usage

```
Usage:
    snatch [OPTIONS]

Snatch, a simple, fast and interruptable download accelerator, written in Rust.

optional arguments:
  -h,--help             show this help message and exit
  -f,--file FILE        The local file to save the remote content file
  -t,--threads THREADS  Number of threads available to download
  -u,--url URL          Remote content URL to download
  -v,--verbose          Verbose mode
```

## Screenshot
 
![example](./img/snatch-screenshot.png)

## File examples

* [a simple PDF file](http://www.cbu.edu.zm/downloads/pdf-sample.pdf) ;
* [Big Bukk Bunny](http://distribution.bbb3d.renderfarming.net/video/mp4/bbb_sunflower_1080p_60fps_stereo_abl.mp4), a big free mp4 file ;
* [the cat DNA](http://hgdownload.cse.ucsc.edu/goldenPath/felCat8/bigZips/felCat8.fa.gz), a big .gz file ;
* [a big PDF file from Princeton](http://scholar.princeton.edu/sites/default/files/oversize_pdf_test_0.pdf).


## Build issues

* `fatal error: 'openssl/hmac.h' file not found`  
If you are on macOS, please to install `openssl` and check your OpenSSL configuration:  

```
brew install openssl
export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include
```
