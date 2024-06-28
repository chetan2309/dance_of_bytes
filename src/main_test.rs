#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::Write};

    use crate::read_from_file;

    const FILE_PATH: &'static str = "buffer_file.txt";
    #[test]
    fn read_single_kv_pair_from_file() {
        let key = b"12";
        let key_len = key.len() as u8;
        let value = b"24";
        let value_len = value.len() as u8;

        let mut buffer = Vec::new();
        buffer.push(key_len);
        buffer.extend_from_slice(key);
        buffer.push(value_len);
        buffer.extend_from_slice(value);

        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILE_PATH)
            .unwrap();
        file.write_all(&buffer).unwrap();

        let file_vector = read_from_file(&FILE_PATH).unwrap();
        assert_eq!(file_vector.len(), 1);
        assert_eq!(file_vector[0].0, key);
        assert_eq!(file_vector[0].1, value);
        assert_ne!(file_vector[0].0, b"13");
    }
}
