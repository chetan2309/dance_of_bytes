mod main_test;

use dance_of_bytes::{read_from_file, KeyValue};
use std::{
    fs::OpenOptions,
    io::{self, Write},
};

const FILE_PATH: &'static str = "buffer_file.txt";

fn main() -> io::Result<()> {
    let kv1 = KeyValue::new(b"12", b"24", Some(15), false);
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .open(FILE_PATH)?;
    file.write_all(&kv1.to_buffer())?;

    let kv2 = KeyValue::new(b"13", b"26", Some(20), false);
    file.write_all(&kv2.to_buffer())?;

    read_from_file(FILE_PATH)?;
    Ok(())
}
