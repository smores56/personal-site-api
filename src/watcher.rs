use std::ffi::OsStr;
use std::path::{Path, PathBuf};
use std::sync::Arc;
use std::sync::{RwLock, RwLockReadGuard};
use std::{collections::HashMap, fs::File};

use crate::error::Error;
use log::{error, info};
use notify::{Event, EventKind, INotifyWatcher, RecursiveMode, Watcher};
use serde::de::DeserializeOwned;

pub struct CollectionWatcher<T> {
    directory: PathBuf,
    files: RwLock<HashMap<PathBuf, T>>,
}

impl<T: DeserializeOwned + Send + Sync + 'static> CollectionWatcher<T> {
    pub fn new(directory: PathBuf) -> Self {
        Self {
            directory,
            files: RwLock::new(HashMap::new()),
        }
    }

    pub fn read_collection<'c>(&'c self) -> RwLockReadGuard<'c, HashMap<PathBuf, T>> {
        self.files.read().unwrap()
    }

    fn all_files(&self) -> impl Iterator<Item = PathBuf> {
        ignore::Walk::new(&self.directory).filter_map(|entry| match entry {
            Ok(entry) => {
                let extension = entry.path().extension().and_then(OsStr::to_str);
                if extension == Some("yml") {
                    Some(entry.path().to_owned())
                } else {
                    None
                }
            }
            Err(_error) => None,
        })
    }

    fn read_file(path: &Path) -> Result<T, Error> {
        let file = File::open(path).map_err(Error::FileReadError)?;
        serde_yaml::from_reader(file).map_err(|error| Error::YamlError {
            path: path.to_owned(),
            error,
        })
    }

    pub fn update_all_files(&self) -> Result<(), Error> {
        let mut files = self.files.write().unwrap();
        for path in self.all_files() {
            let data = Self::read_file(&path)?;
            files.insert(path.to_owned(), data);
        }
        Ok(())
    }

    pub fn watch(self: Arc<Self>) -> Result<(), Error> {
        self.update_all_files()?;

        let directory = self.directory.clone();

        let mut watcher = INotifyWatcher::new_immediate(move |event| {
            info!("File event detected: {:?}", event);
            match event {
                Ok(event) => {
                    if let Err(error) = self.clone().handle(event) {
                        error!("Error occurred while handling directory event: {}", error);
                    }
                }
                Err(error) => error!("Error occurred while watching directory: {}", error),
            }
        })
        .map_err(Error::NotifyError)?;

        watcher
            .watch(&directory, RecursiveMode::Recursive)
            .map_err(Error::NotifyError)?;

        Ok(())
    }

    fn handle(&self, event: Event) -> Result<(), Error> {
        if matches!(
            event.kind,
            EventKind::Access(_) | EventKind::Modify(_) | EventKind::Remove(_)
        ) {
            self.update_all_files()
        } else {
            Ok(())
        }
    }
}
