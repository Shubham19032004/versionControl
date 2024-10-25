use crate::hash::Hash;
use crate::{hash_object::hash_object, utils::save_to_disk};
use anyhow::{Context, Result};
use ignore::WalkBuilder;
use std::{fmt::Display, path::PathBuf};

// Unix-specific permissions handling
#[cfg(unix)]
use std::os::unix::prelude::PermissionsExt;

pub fn write_tree() -> Result<String> {
    // files and folders in the current directory
    let path = PathBuf::new().join(".");
    let checksum = write_tree_object(&path)?.expect("getting checksum after writing all trees");

    Ok(hex::encode(checksum))
}
fn write_tree_object(path: &PathBuf) -> Result<Option<Hash>> {
    let mut objects = vec![];
    for object in WalkBuilder::new(&path)
        .hidden(false)
        .max_depth(Some(1))
        .build()
        .skip(1)
    {
        let dir_object = object?;
        let file_path = dir_object.path();
        let metadata = dir_object
            .metadata()
            .context("error getting directory metadata")?;
        let name = dir_object
            .file_name()
            .to_str()
            .context("Could not get file name from object")?
            .to_owned();

        // Handle Unix-specific permissions
        #[cfg(unix)]
        let mode = metadata.permissions().mode();

        // Provide an alternative on non-Unix platforms
        #[cfg(not(unix))]
        let mode = {
            // Default mode or alternative handling
            0o644 // Example default mode
        };

        let file_object = if metadata.is_file() {
            let checksum = hash_object(true, file_path.to_path_buf())?;
            Some(TreeObject::new(true, checksum, name, mode))
        } else {
            if name == ".vc" {
                continue;
            }

            if let Some(checksum) = write_tree_object(&file_path.to_path_buf())? {
                Some(TreeObject::new(false, checksum, name, mode))
            } else {
                None
            }
        };

        objects.extend(file_object);
    }

    objects.sort_unstable_by_key(|object| object.name.clone());
    if objects.is_empty() {
        Ok(None)
    } else {
        let tree_file = create_tree_file(&objects);
        let path = PathBuf::new();
        let hash = save_to_disk(&tree_file, path)?;

        Ok(Some(hash))
    }
}


#[derive(Debug)]
struct TreeObject {
    mode: String,
    checksum: Hash,
    name: String,
}

impl TreeObject {
    pub fn new(is_file: bool, checksum: Hash, name: String, mode: u32) -> Self {
        let object_type = TreeObjectType::new(is_file, mode);
        let mode = object_type.mode();

        Self {
            mode,
            checksum,
            name,
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = vec![];

        let mode = format!("{} ", &self.mode);
        bytes.extend(mode.as_bytes());

        let name = format!("{}\0", &self.name);
        bytes.extend(name.as_bytes());

        bytes.extend(&self.checksum);

        bytes
    }
}

impl Display for TreeObject {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {}\0{:?}", &self.mode, &self.name, &self.checksum)
    }
}

#[derive(Debug)]
enum TreeObjectType {
    Blob(&'static str),
    Tree(&'static str),
}

impl TreeObjectType {
    pub fn new(is_file: bool, mode: u32) -> Self {
        if is_file {
            let mode = if mode & 0o100 == 0o100 {
                "100755"
            } else {
                "100644"
            };
            Self::Blob(mode)
        } else {
            Self::Tree("40000")
        }
    }

    pub fn mode(&self) -> String {
        match self {
            Self::Blob(mode) => mode,
            Self::Tree(mode) => mode,
        }
        .to_string()
    }
}

impl Display for TreeObjectType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name = match self {
            Self::Blob(_) => "blob",
            Self::Tree(_) => "tree",
        };

        write!(f, "{name}")
    }
}

fn create_tree_file(objects: &[TreeObject]) -> Vec<u8> {
    let mut tree_file = vec![];

    let objects = objects
        .iter()
        .map(|object| object.as_bytes())
        .collect::<Vec<Vec<u8>>>();
    let size = objects.iter().fold(0, |acc, object| acc + object.len());

    tree_file.extend(format!("tree {size}\0").as_bytes());
    for object in objects {
        tree_file.extend(object)
    }

    tree_file
}

// fn write_tree_to_file(content: &str, hash: &str) -> Result<()> {

// }
