use anyhow::Context;
use anyhow::Result;
use std::path::PathBuf;

use crate::tree::Tree;
use crate::{clone::GitObjects, hash::Hash};

pub fn checkout(path: PathBuf, git_objects: &GitObjects, commit_hash: Hash) -> Result<()> {
    let commit = git_objects
        .get(&commit_hash)
        .expect("cannot find commit in git objects");
    let tree_hash: Hash = commit[5..45]
        .to_vec()
        .try_into()
        .context("converting commit bytes to hash")?;

    process_tree(path, git_objects, tree_hash).context("processing tree")?;

    Ok(())
}

fn process_tree(path: PathBuf, git_objects: &GitObjects, tree_hash: Hash) -> Result<()> {
    let tree_bytes = git_objects
        .get(&tree_hash)
        .expect("missing tree referenced in commit");
    let tree = Tree::from(tree_bytes.as_slice());

    for tree_object in &tree.tree_objects {
        match tree_object.object_type {
            crate::tree::TreeObjectType::Blob => {
                let mut path = path.clone();
                let git_object = git_objects
                    .get(&tree_object.checksum)
                    .context("getting git object to write to disk")?;

                path.push(&tree_object.filename);
                write_blob_to_file(path, git_object).context("writing blob to file")?;
            }
            crate::tree::TreeObjectType::Tree => {
                let mut path = path.clone();

                path.push(&tree_object.filename);
                std::fs::create_dir(&path).context("creating directory")?;
                process_tree(path, git_objects, tree_object.checksum.clone())
                    .context("recursively processing tree")?;
            }
        }
    }

    Ok(())
}

fn write_blob_to_file(path: PathBuf, bytes: &[u8]) -> Result<()> {
    std::fs::write(path, bytes)?;

    Ok(())
}
