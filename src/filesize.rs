use std::fmt;

#[derive(Debug)]
enum FileSize {
    B,
    KB,
    MB,
    GB,
    TB,
}

impl fmt::Display for FileSize {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl FileSize {
    fn value(&self) -> u64 {
        match *self {
            FileSize::B => 1,
            FileSize::KB => 1_024,
            FileSize::MB => 1_048_576,
            FileSize::GB => 1_073_741_824,
            FileSize::TB => 1_099_511_627_776,
        }
    }
}

pub fn format_filesize(size: u64) -> String {

    let (file_size, unit) = match size {
        0...999 => (size as f64, FileSize::B),
        1_000...999_999 => (size as f64 / FileSize::KB.value() as f64, FileSize::KB),
        1_000_000...999_999_999 => (size as f64 / FileSize::MB.value() as f64, FileSize::MB),
        1_000_000_000...999_999_999_999 => {
            (size as f64 / FileSize::GB.value() as f64, FileSize::GB)
        }
        _ => (size as f64 / FileSize::TB.value() as f64, FileSize::TB),
    };

    format!("{:.2} {}", file_size, unit)
}
