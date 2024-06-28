mod main_test;

use dance_of_bytes::read_from_file;
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

const FILE_PATH: &'static str = "buffer_file.txt";

fn main() -> io::Result<()> {
    let key = b"12";
    let key_len = key.len() as u8;
    let value = b"24";
    let value_len = value.len() as u8;
    // let key_len_bytes = [key_len];

    let mut buffer = Vec::new();
    buffer.push(key_len);
    buffer.extend_from_slice(key);
    buffer.push(value_len);
    buffer.extend_from_slice(value);

    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)?;
    file.write_all(&buffer)?;

    let key = b"13";
    let key_len = key.len() as u8;
    let value = b"26";
    // let key_len_bytes = [key_len];

    let mut buffer = Vec::new();
    buffer.push(key_len);
    buffer.extend_from_slice(key);
    let value_len = value.len() as u8;
    buffer.push(value_len);
    buffer.extend_from_slice(value);

    file.write_all(&buffer)?;

    read_from_file(FILE_PATH)?;
    Ok(())
}
