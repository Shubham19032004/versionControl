use anyhow::{bail, Context, Result};
use flate2::read::ZlibDecoder;
use flate2::write::ZlibEncoder;
use flate2::Compression;
use sha1::{Digest, Sha1};
use std::{
    io::{Read, Write},
    path::PathBuf,
};

use crate::hash::Hash;

pub fn get_object_directory_name(hash: &str) -> String {
    hash[0..2].to_owned()
}

pub fn get_object_file_name(hash: &str) -> String {
    hash[2..].to_owned()
}

pub fn decompress(bytes: &[u8]) -> Vec<u8> {
    let mut decoder = ZlibDecoder::new(bytes);
    let mut result = vec![];

    decoder
        .read_to_end(&mut result)
        .expect("error decompressing");

    result
}

pub fn next_chunk(bytes: &[u8], offset_nulls: usize) -> Option<&[u8]> {
    let mut result = bytes.split(|&bytes| bytes == b'\0');

    result.nth(offset_nulls)
}

pub fn compress(content: &[u8]) -> Result<Vec<u8>> {
    let mut encoded = Vec::new();
    let mut encoder = ZlibEncoder::new(&mut encoded, Compression::default());

    encoder
        .write_all(content)
        .context("error write all compress")?;

    Ok(encoder.finish()?.to_owned())
}

pub fn get_hash(content: &[u8]) -> Result<Hash> {
    let mut hasher = Sha1::new();

    hasher.update(content);

    Ok(Hash::new(hasher.finalize().into()))
}

pub fn save_to_disk(content: &[u8], mut path: PathBuf) -> Result<Hash> {
    let hash = get_hash(content)?;
    let compressed = compress(content)?;
    let hash_utf8 = hex::encode(&hash);
    let directory_name = get_object_directory_name(&hash_utf8);
    let file_name = get_object_file_name(&hash_utf8);

    path = path.join(".vc").join("objects").join(directory_name);

    let Ok(directory_exists) = path.try_exists() else {
        bail!("Error checking if directory exists");
    };

    if !directory_exists {
        std::fs::DirBuilder::new().create(&path)?;
    }

    path = path.join(&file_name);

    if path.exists() {
        return Ok(hash);
    }

    std::fs::write(path, &compressed)?;

    Ok(hash)
}

pub fn create_directory(path: &PathBuf) -> Result<()> {
    if !path.exists() {
        std::fs::DirBuilder::new().create(path)?;
    }

    Ok(())
}

pub fn remove_header(bytes: &[u8]) -> Result<&[u8]> {
    let index = bytes
        .iter()
        .position(|&byte| byte == b'\0')
        .context("finding index of null to remove header")?;
    Ok(&bytes[index + 1..])
}

#[cfg(test)]
mod tests {
    use std::io::Write;

    use flate2::{write::ZlibEncoder, Compression};

    use super::*;

    #[test]
    fn should_get_object_directory_name_from_hash() {
        let hash = "8515244e62a6f01fea3d4866a4e075782b81a05e";
        let expected_name = "85";
        let name = get_object_directory_name(hash);

        assert_eq!(name, expected_name);
    }

    #[test]
    fn should_get_object_file_name() {
        let hash = "8515244e62a6f01fea3d4866a4e075782b81a05e";
        let expected_name = "15244e62a6f01fea3d4866a4e075782b81a05e";
        let name = get_object_file_name(hash);

        assert_eq!(name, expected_name);
    }

    #[test]
    fn should_decompress() {
        let de_compressed_string = "8515244e62a6f01fea3d4866a4e075782b81a05e";
        let mut encoder = ZlibEncoder::new(vec![], Compression::default());

        encoder.write_all(de_compressed_string.as_bytes()).unwrap();

        let compressed = encoder.finish().unwrap();
        let de_compressed = decompress(&compressed);

        assert_eq!(de_compressed, de_compressed_string.as_bytes());
    }

    #[test]
    fn should_return_chunk_before_first_null() {
        let string = "eanfphensrtduyfj\0rsiueaptyrafupgdreif\0";
        let expected_value = "eanfphensrtduyfj".as_bytes();
        let result = next_chunk(&string.as_bytes(), 0).unwrap();

        assert_eq!(result, expected_value);
    }

    #[test]
    fn should_return_index_of_second_null() {
        let string = "eanfphensrtduyfj\0rsiueaptyrafupgdreif\0aoiresth";
        let string_bytes = string.as_bytes();
        let expected_result = "rsiueaptyrafupgdreif".as_bytes();
        let result = next_chunk(string_bytes, 1).unwrap();

        assert_eq!(result, expected_result);
    }

    #[test]
    fn should_remove_header() -> Result<()> {
        let string = "blob 324\0fup8ljhshrne";
        let string_bytes = string.as_bytes();
        let expected_result = "fup8ljhshrne".as_bytes();
        let result = remove_header(string_bytes)?;

        assert_eq!(expected_result, result);
        Ok(())
    }
}
