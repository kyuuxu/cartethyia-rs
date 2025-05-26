use std::io::{Cursor, Read, Write};
use std::path::Path;

use git2::{FetchOptions, RemoteCallbacks, Repository};
use git2::build::RepoBuilder;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("UReq error: {0}")]
    UReq(#[from] ureq::Error),
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("ZipExtract error: {0}")]
    ZipExtract(#[from] zip_extract::ZipExtractError),
    #[error("Git error: {0}")]
    Git(#[from] git2::Error),
    #[error("Option conversion error")]
    Option(),
}

pub fn update_from_releases<T: AsRef<str> + std::fmt::Display>(object_url: T,
                                                               target_folder: T,
                                                               max_buffer: usize) -> Result<(), Error> {
    let path = Path::new(target_folder.as_ref());
    std::fs::create_dir_all(path)?;
    let version = path.join("version.txt");
    let new_version = object_url.as_ref();
    if version.exists() {
        tracing::debug!("Version marker exists, checking version");
        let old_version = std::fs::read_to_string(&version)?;
        if old_version.eq(new_version) {
            tracing::debug!("The version to download is the current one, skipping ...");
            return Ok(())
        } else {
            tracing::debug!("The old version is: {old_version}, new version: {new_version}")
        }

    } else {
        tracing::debug!("Version marker doesn't exist, fetching resources for first time")
    }

    tracing::debug!("Starting the download of static assets from {object_url}");
    let mut bytes: Vec<u8> = Vec::with_capacity(max_buffer);
    let agent = ureq::config::Config::builder()
        .timeout_send_body(Some(std::time::Duration::from_secs(60)))
        .timeout_recv_body(Some(std::time::Duration::from_secs(60)))
        .build()
        .new_agent();

    let response = agent.get(new_version).call()?;
    response.into_body().as_reader().read_to_end(&mut bytes)?;
    tracing::debug!("Downloading assets from: {object_url} finished, extracting into {target_folder}");
    zip_extract::extract(Cursor::new(bytes), path, true)?;
    tracing::debug!("Resources extracted");

    let mut file = std::fs::File::create(&version)?;
    file.write_all(new_version.as_bytes())?;

    Ok(())
}

pub fn clone_or_update_repository<T: AsRef<str> + std::fmt::Display>(repository_url: T,
                                                                     repository_path: T,
                                                                     ref_name: T,
                                                                     discard_local_changes: bool) -> Result<(), Error> {
    // Open existing repository or clone if not existing
    let path = Path::new(repository_path.as_ref());
    let repository = match path.exists() {
        true => {
            tracing::debug!("Path {repository_path} exists, opening repository");
            Repository::open(repository_path.as_ref())
        }
        false => clone_repository(repository_url.as_ref(), path)
    }?;
    // TODO: Fetch remote updates????
    // Get reference
    let (object, reference) = repository.revparse_ext(ref_name.as_ref())?;
    tracing::debug!("Reference {ref_name} found in repository");
    let update_needed = match &reference {
        Some(reference) => repository.head()?.eq(reference),
        None => repository.head()?.target().ok_or(Error::Option())?.eq(&object.id()),
    };
    // If we are already in commit, no need to modify anything
    if update_needed {
        // Discard all local changes
        if discard_local_changes {
            tracing::debug!("discard_local_changes was set to true, changes will be discarded");
            repository.checkout_head(None)?;
        } else {
            tracing::warn!("discard_local_changes was set to false, changes will be kept");
        }
        // Checkout required remote
        repository.checkout_tree(&object, None)?;
        // Checkout the needed commit
        match reference {
            Some(repo_reference) => repository.set_head(repo_reference.name().unwrap())?,
            None => repository.set_head_detached(object.id())?,
        }
        tracing::debug!("Repository has been updated to {ref_name}");
    } else {
        tracing::debug!("Repository is already at the desired reference, no update needed");
    };
    Ok(())
}

fn clone_repository(repository_url: &str, path: &Path) -> Result<Repository, git2::Error> {
    tracing::debug!(
        "Path {path:?} doesn't exists, cloning repository at {repository_url}"
    );

    let mut remote_callbacks = RemoteCallbacks::new();
    remote_callbacks.sideband_progress(|data| {
        tracing::debug!("remote: {}", std::str::from_utf8(data).unwrap());
        std::io::stdout().flush().unwrap();
        true
    });
    remote_callbacks.transfer_progress(|stats| {
        if stats.received_objects() == stats.total_objects() {
            tracing::debug!(
                "Resolving deltas {}/{}\r",
                stats.indexed_deltas(),
                stats.total_deltas()
            );
        } else if stats.total_objects() > 0 {
            tracing::debug!(
                "Received {}/{} objects ({}) in {} bytes\r",
                stats.received_objects(),
                stats.total_objects(),
                stats.indexed_objects(),
                stats.received_bytes()
            );
        }
        std::io::stdout().flush().unwrap();
        true
    });

    let mut fetch_options = FetchOptions::new();
    fetch_options.remote_callbacks(remote_callbacks);

    let result = RepoBuilder::new()
        .fetch_options(fetch_options)
        .clone(repository_url, path);

    result
}