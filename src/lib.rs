use std::{
    fs::OpenOptions,
    io::{self, Read},
};

pub fn read_from_file(file_path: &str) -> io::Result<Vec<(Vec<u8>, Vec<u8>)>> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut records = Vec::new();
    loop {
        let mut key_len_buf = [0; 1];
        if file.read_exact(&mut key_len_buf).is_err() {
            break;
        }
        let key_len = key_len_buf[0] as usize;

        let mut key_buf = vec![0; key_len];
        if file.read_exact(&mut key_buf).is_err() {
            break;
        }

        // Read value's length
        let mut value_len_buffer = [0; 1];
        if file.read_exact(&mut value_len_buffer).is_err() {
            break;
        }
        let value_length = value_len_buffer[0] as usize;

        let mut value_buf = vec![0; value_length];
        if file.read_exact(&mut value_buf).is_err() {
            break;
        }

        println!("(u8) Key: {:?}", String::from_utf8_lossy(&key_buf));
        println!("(u8) Value: {:?}", String::from_utf8_lossy(&value_buf));

        records.push((key_buf, value_buf));
    }

    Ok(records)
}