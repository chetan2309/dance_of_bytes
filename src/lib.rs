use std::{
    fs::OpenOptions,
    io::{self, Read}
};
use crc32fast::Hasher;

pub struct KeyValue {
    pub key: Vec<u8>,
    pub value: Vec<u8>,
    pub timestamp: Option<u64>,
    pub tombstone: bool,
    pub checksum: u32
}

impl KeyValue {
    pub fn new(key: &[u8], val: &[u8], expiry: Option<u64>, tombstone: bool, checksum: u32) -> Self {
        let mut kv = KeyValue {
            key: key.to_vec(),
            value: val.to_vec(),
            timestamp: expiry,
            tombstone,
            checksum
        };
        kv.checksum =  kv.calculate_checksum();
        kv
    }

    fn calculate_checksum(&self) -> u32 {
        let mut hasher = Hasher::new();
        hasher.update(&self.key);
        hasher.update(&self.value);
        hasher.update(&self.timestamp.unwrap_or(0).to_le_bytes());
        hasher.update(&[self.tombstone as u8]);
        hasher.finalize()
    }

    pub fn to_buffer(&self) -> Vec<u8> {
        let mut buffer = Vec::new();
        buffer.push(self.key.len() as u8);
        buffer.push(self.value.len() as u8);

        buffer.extend_from_slice(&self.key);
        buffer.extend_from_slice(&self.value);

        buffer.extend_from_slice(&self.timestamp.unwrap_or(0).to_le_bytes());
        buffer.push(self.tombstone as u8);

        buffer.extend_from_slice(&self.checksum.to_le_bytes());

        buffer
    }
}

pub fn read_from_file(file_path: &str) -> io::Result<Vec<KeyValue>> {
    let mut file = OpenOptions::new().read(true).open(file_path)?;
    let mut records = Vec::new();
    loop {
        // Read key length (u8)
        let mut key_len_buf = [0u8; 1];
        if file.read_exact(&mut key_len_buf).is_err() {
            break;
        }
        let key_len = key_len_buf[0] as usize;

        // Read value length (u8)
        let mut value_len_buf = [0u8; 1];
        file.read_exact(&mut value_len_buf)?;
        let value_len = value_len_buf[0] as usize;

        // Read key
        let mut key_buf = vec![0; key_len];
        file.read_exact(&mut key_buf)?;

        // Read value
        let mut value_buf = vec![0; value_len];
        file.read_exact(&mut value_buf)?;

        // Read timestamp
        let mut timestamp_buffer = [0u8; 8];
        file.read_exact(&mut timestamp_buffer)?;
        let timestamp = Some(u64::from_le_bytes(timestamp_buffer));

        // Read tombstone
        let mut tombstone_buffer = [0u8; 1];
        file.read_exact(&mut tombstone_buffer)?;
        let tombstone = tombstone_buffer[0] != 0;

        // Read checksum
        let mut checksum_buffer = [0u8; 4];
        file.read_exact(&mut checksum_buffer)?;
        let checksum_from_file = u32::from_le_bytes(checksum_buffer);

        records.push(KeyValue {
            key: key_buf,
            value: value_buf,
            timestamp,
            tombstone,
            checksum: checksum_from_file
        });
    }

    Ok(records)
}
