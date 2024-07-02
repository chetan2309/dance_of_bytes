#[cfg(test)]
mod tests {
    use std::{fs::OpenOptions, io::{self, Write}};

    use dance_of_bytes::KeyValue;

    use crate::read_from_file;

    const FILE_PATH: &'static str = "buffer_file_test.txt";
    #[test]
    fn read_single_kv_pair_from_file() -> io::Result<()> {
        let kv1 = KeyValue::new(b"12", b"24", Some(15), false);
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .open(FILE_PATH)?;
        file.write_all(&kv1.to_buffer())?;

        let file_vector = read_from_file(&FILE_PATH).unwrap();
        assert_eq!(file_vector.len(), 1);
        assert_eq!(file_vector[0].key, b"12");
        assert_eq!(file_vector[0].value, b"24");
        assert_ne!(file_vector[0].key, b"13");
        Ok(())
    }
}
