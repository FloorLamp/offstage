use git2::{Oid, Repository, StashFlags};
use std::fs;
use std::io::ErrorKind::NotFound;
use std::path::Path;

pub fn open_repository() -> Repository {
    Repository::open_from_env().expect("Repository could not be opened.")
}

pub fn save_snapshot_stash(repository: &mut Repository) -> Oid {
    // If we are in the middle of a merge, save the merge status, because we
    // will run `git stash`, and that clears it.
    let merge_status = save_merge_status(repository);

    // TODO: Friendly error message here.
    let signature = repository.signature().expect("No signature.");

    // TODO: Friendly error message here.
    let stash = repository
        .stash_save(
            &signature,
            "offstage backup",
            Some(StashFlags::INCLUDE_UNTRACKED | StashFlags::KEEP_INDEX),
        )
        .unwrap();

    restore_merge_status(repository, &merge_status);

    stash
}

fn save_merge_status(repository: &Repository) -> MergeStatus {
    MergeStatus {
        merge_head: maybe_read_file(repository.path().join("MERGE_HEAD")),
        merge_mode: maybe_read_file(repository.path().join("MERGE_MODE")),
        merge_msg: maybe_read_file(repository.path().join("MERGE_MSG")),
    }
}

fn restore_merge_status(repository: &Repository, merge_status: &MergeStatus) {
    if let Some(merge_head) = &merge_status.merge_head {
        fs::write(repository.path().join("MERGE_HEAD"), merge_head)
            .expect("TODO: Friendly error message");
    }

    if let Some(merge_mode) = &merge_status.merge_mode {
        fs::write(repository.path().join("MERGE_MODE"), merge_mode)
            .expect("TODO: Friendly error message");
    }

    if let Some(merge_msg) = &merge_status.merge_msg {
        fs::write(repository.path().join("MERGE_MSG"), merge_msg)
            .expect("TODO: Friendly error message");
    }
}

fn maybe_read_file<P: AsRef<Path>>(file: P) -> Option<String> {
    match fs::read_to_string(file) {
        Ok(contents) => Some(contents),
        Err(error) if error.kind() == NotFound => None,
        // Figure out how to print the file name here...
        Err(error) => panic!("Problem opening file: {:?}", error),
    }
}

#[derive(Debug)]
struct MergeStatus {
    merge_head: Option<String>,
    merge_mode: Option<String>,
    merge_msg: Option<String>,
}