# snatch
A simple, fast and interruptable download accelerator, written in Rust

## Features

* Simple: a command line to manage easily your downloads,
* Fast: written in a new exciting programing language,
* Interruptable: you can interrupt and resume easily your downloads (SOON).

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

## Build issues

* `fatal error: 'openssl/hmac.h' file not found`  
If you are on macOS, please to install `openssl` and check your OpenSSL configuration:  

      brew install openssl
      export OPENSSL_INCLUDE_DIR=`brew --prefix openssl`/include
      export OPENSSL_LIB_DIR=`brew --prefix openssl`/lib
      export DEP_OPENSSL_INCLUDE=`brew --prefix openssl`/include
