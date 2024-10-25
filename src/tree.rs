use crate::hash::Hash;
use anyhow::{bail, Context, Result};
use std::fmt::Display;

#[derive(Debug)]
pub struct Tree {
    pub tree_objects: Vec<TreeObject>,
}

impl Tree {
    pub fn filenames(&self) -> Vec<&str> {
        self.tree_objects
            .iter()
            .map(|tree_object| tree_object.filename.as_str())
            .collect()
    }
}

impl From<&[u8]> for Tree {
    fn from(value: &[u8]) -> Self {
        let mut tree_objects = vec![];
        let value_iter = value.iter();
        let mut values = value_iter.copied().peekable();

        loop {
            if let None = values.peek() {
                break;
            };

            let mut tree_object = TreeObject::default();

            tree_object
                .extract_mode(&mut values)
                .expect("Error extracting mode while creating tree");

            tree_object.set_object_type();

            tree_object
                .extract_filename(&mut values)
                .expect("Error extracting filename while creating tree");

            // tree_object
            //     .parse_mode_and_filename(lines.next())
            //     .expect("error parseing mode and filename");

            tree_object
                .parse_hash(&mut values)
                .expect("error parseing hash");

            tree_objects.push(tree_object);
        }

        Self { tree_objects }
    }
}

#[derive(Default, Debug)]
pub struct TreeObject {
    mode: u32,
    pub object_type: TreeObjectType,
    pub filename: String,
    pub checksum: Hash,
}

impl TreeObject {
    pub fn extract_mode(&mut self, bytes: &mut impl Iterator<Item = u8>) -> Result<()> {
        let mut mode_bytes = Vec::new();

        for byte in bytes {
            if byte == b' ' {
                break;
            }

            mode_bytes.push(byte);
        }

        self.mode = String::from_utf8(mode_bytes)?.parse()?;

        Ok(())
    }

    pub fn set_object_type(&mut self) {
        self.object_type = TreeObjectType::from(self.mode);
    }

    pub fn extract_filename(&mut self, bytes: &mut impl Iterator<Item = u8>) -> Result<()> {
        let mut filename_bytes = vec![];

        for byte in bytes.take_while(|&byte| byte != b'\0') {
            filename_bytes.push(byte);
        }

        self.filename = String::from_utf8(filename_bytes).context("extracting filename")?;

        Ok(())
    }

    pub fn parse_mode_and_filename(&mut self, bytes: Option<&[u8]>) -> Result<()> {
        let Some(bytes) = bytes else {
            bail!("missing bytes")
        };
        let mut split_bytes = bytes.split(|&byte| byte == b' ');
        let mode_as_bytes = split_bytes
            .next()
            .expect("missing mode when parseing mode and filename");
        let mode = std::str::from_utf8(mode_as_bytes)?.parse()?;
        let filename_as_bytes = split_bytes
            .next()
            .expect("missing filename when parseing mode and filename");
        let filename = std::str::from_utf8(filename_as_bytes)?.to_owned();

        self.mode = mode;
        self.filename = filename;

        Ok(())
    }

    pub fn parse_hash(&mut self, bytes: &mut impl Iterator<Item = u8>) -> Result<()> {
        let mut hash_bytes = [0; 20];

        for index in 0..20 {
            let byte = bytes
                .next()
                .context("missing byte when extracting the hash")?;

            hash_bytes[index] = byte;
        }

        let hash = Hash::new(hash_bytes);

        self.checksum = hash;

        Ok(())
    }
}

impl Display for TreeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {:?} {}\t{}",
            self.mode, self.object_type, self.checksum, &self.filename
        )
    }
}

#[derive(Default, Debug)]
pub enum TreeObjectType {
    #[default]
    Blob,
    Tree,
}

impl From<u32> for TreeObjectType {
    fn from(value: u32) -> Self {
        match value {
            100644 | 644 | 755 | 100755 => Self::Blob,
            40000 => Self::Tree,
            _ => unreachable!("attempting to extract tree object type, but not one of the types"),
        }
    }
}
