use crate::helper::errors::SentenceSplitterError;

use cached_path::{Cache, Options, ProgressBar};
use lazy_static::lazy_static;
use std::env;
use std::path::PathBuf;

extern crate dirs;

#[derive(PartialEq, Clone)]
pub enum Resource {
    Local(LocalResource),
    Remote(RemoteResource),
}

impl Resource {
    pub fn get_local_path(&self) -> Result<PathBuf, SentenceSplitterError> {
        match self {
            Resource::Local(resource) => Ok(resource.local_path.clone()),
            Resource::Remote(resource) => {
                let cached_path = CACHE.cached_path_with_options(
                    &resource.url,
                    &Options::default().subdir(&resource.cache_subdir),
                )?;
                Ok(cached_path)
            }
        }
    }
}

#[derive(PartialEq, Clone)]
pub struct LocalResource {
    pub local_path: PathBuf,
}

#[derive(PartialEq, Clone)]
pub struct RemoteResource {
    pub url: String,
    pub cache_subdir: String,
}

impl RemoteResource {
    pub fn new(url: &str, cache_subdir: &str) -> RemoteResource {
        RemoteResource {
            url: url.to_string(),
            cache_subdir: cache_subdir.to_string(),
        }
    }
}

lazy_static! {
    #[derive(Copy, Clone, Debug)]
    pub static ref CACHE: Cache = Cache::builder()
        .dir(_get_cache_directory())
        .progress_bar(Some(ProgressBar::Light))
        .build().unwrap();
}

fn _get_cache_directory() -> PathBuf {
    match env::var("SENTENCE_SPLITTER_CACHE") {
        Ok(value) => PathBuf::from(value),
        Err(_) => {
            let mut home = dirs::home_dir().unwrap();
            home.push(".cache");
            home.push(".sentence-splitter");
            home
        }
    }
}
