use std::{path::PathBuf, time::UNIX_EPOCH};

use anyhow::Result;
use hex::ToHex;

use crate::utils::save_to_disk;

pub fn commit_tree(tree: &str, parent: &str, message: &str) -> Result<String> {
    let mut commit_body = vec![];
    let timestamp = std::time::SystemTime::now()
        .duration_since(UNIX_EPOCH)?
        .as_millis();

    // put the commit into the body
    commit_body.extend(format!("tree {tree}\n").into_bytes());
    commit_body.extend(format!("parent {parent}\n").into_bytes());
    commit_body.extend(
        format!("author Brookzerker <brooks_not_real_address@mailinator.com> {timestamp} -0700\n",)
            .into_bytes(),
    );
    commit_body.extend(
        format!(
            "committer brookzerker<brooks_not_real_address@mailinator.com> {timestamp} -0700\n\n"
        )
        .into_bytes(),
    );
    commit_body.extend(format!("{message}\n").into_bytes());

    let size = commit_body.len();
    let mut commit = format!("commit {size}\0").into_bytes();

    commit.extend(&commit_body);

    let hash = save_to_disk(&commit, PathBuf::new())?;

    Ok(hash.encode_hex())
}
