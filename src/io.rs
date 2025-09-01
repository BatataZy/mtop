use std::{fs, io::Read, path};

#[must_use]
pub fn read(file_path: &str, start: usize, end: usize) -> String {
    let mut buffer: String = String::new();

    let mut file = file_open(file_path);

    let read = file.read_to_string(&mut buffer);

    match read {
        Ok(_) => buffer[start..buffer.len() - 1 - end].to_owned(),
        Err(_) => ("0").to_owned(),
    }
}

/// # Panics
///
/// Will panic if a file that doesn't exists tries to get opened.
/// Will also panic if a file cannot be opened for some reason.
#[must_use]
pub fn file_open(path: &str) -> fs::File {
    let os_path: &path::Path = path::Path::new(path); //::from_str(path).expect("valid path");

    assert!(
        os_path.exists(),
        "Tried to open file '{path}' that doesn't exist."
    );

    fs::File::
        // .truncate(truncate)
        open(os_path)
    .unwrap_or_else(|_| panic!("Failed to open file: '{path}'"))
}
