# snatch
A simple, fast and interruptable download accelerator, written in Rust

## Features

* Simple: a command line to manage easily your downloads,
* Fast: written in a new exciting programing language,
* Interruptable: you can interrupt and resume easily your downloads.

## Usage

```
Usage:
    snatch [OPTIONS]

A simple, fast and interruptable download accelerator.

optional arguments:
  -h,--help             show this help message and exit
  -V,--version          Show version
  -v,--verbose          Be verbose
  -O,--output_directory OUTPUT_DIRECTORY
                        Path to the output directory
  -o,--output_file OUTPUT_FILE
                        Output file name
  -u,--url URL          The URL to download the file
```

## Build issues

* `fatal error: 'openssl/hmac.h' file not found`  
If you are on macOS, please to install `openssl` and check your OpenSSL configuration:  

      brew install openssl
      export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
      export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
      export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include