use std::ffi::OsStr;
use std::fs::File;
use std::iter::{empty, once};
use std::path::{Path, PathBuf};

use log::error;
use serde::de::DeserializeOwned;

use crate::review::{Review, Reviews};
use crate::{error::Error, recipe::Recipe};

pub struct Context {
    pub reviews: Vec<Review>,
    pub recipes: Vec<Recipe>,
}

impl Context {
    pub fn new(recipe_dir: &Path, review_dir: &Path) -> Self {
        let mut recipes = Self::walk_files(&recipe_dir)
            .filter_map(|path| match Self::read_file(&path) {
                Ok(recipe) => Some(recipe),
                Err(read_err) => {
                    error!("File read error: {}", read_err);
                    None
                }
            })
            .collect::<Vec<Recipe>>();
        let mut reviews = Self::walk_files(&review_dir)
            .flat_map(|path| match Self::read_file(&path) {
                Ok(reviews) => match reviews {
                    Reviews::Single(review) => {
                        Box::new(once(review)) as Box<dyn Iterator<Item = Review>>
                    }
                    Reviews::List(list) => Box::new(list.into_iter()),
                },
                Err(read_err) => {
                    error!("File read error: {}", read_err);
                    Box::new(empty())
                }
            })
            .collect::<Vec<Review>>();

        recipes.sort_by_key(|recipe| recipe.name.clone());
        reviews.sort_by_key(|review| (review.title.clone(), review.year));

        Self { recipes, reviews }
    }

    fn read_file<T: DeserializeOwned>(path: &Path) -> Result<T, Error> {
        let file = File::open(path).map_err(Error::FileReadError)?;
        serde_yaml::from_reader(file).map_err(|error| Error::YamlError {
            error,
            path: path.to_owned(),
        })
    }

    fn walk_files(dir: &Path) -> impl Iterator<Item = PathBuf> {
        ignore::Walk::new(&dir).filter_map(|entry| match entry {
            Ok(entry) => {
                let extension = entry.path().extension().and_then(OsStr::to_str);
                if extension == Some("yml") {
                    Some(entry.path().to_owned())
                } else {
                    None
                }
            }
            Err(err) => {
                error!("Invalid entry in directory: {}", err);
                None
            }
        })
    }
}
