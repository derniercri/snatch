# snatch
A simple, fast and interruptable download accelerator, written in Rust

## Features

* Simple: a command line to manage easily your downloads,
* Fast: written in a new exciting programing language,
* Interruptable: you can interrupt and resume easily your downloads.

## Options

* `-c`: resume a download
* `-d`: delete file(s) after download (usefull when executing a script)
* `-l`: maximum recursion depth
* `-h`: help command
* `-m`: maximum of threads to download the file(s)
* `-o`: the output file
* `-O`: the standard output
* `-r`: number of retry if a problem is occuring during the download
* `-v`: verbose mode (critical)
* `-vv`: verbose mode (error)
* `-vvv`: verbose mode (warning)
* `-vvvv`: verbose mode (info)
* `-x`: exclude some files (regex, for example '*.DS_Store')