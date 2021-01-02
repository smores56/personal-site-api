use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error("Error occurred while reading a file: {0}")]
    FileReadError(std::io::Error),
    #[error("Unable to start watching directory: {0}")]
    NotifyError(notify::Error),
    #[error("Failed to read YAML document in file {path}: {error}")]
    YamlError {
        path: PathBuf,
        #[source]
        error: serde_yaml::Error,
    },
    #[error("Could not find a recipe named '{0}'")]
    NoRecipeNamed(String),
    #[error("Could not find a review for a movie titled '{0}'")]
    NoReviewTitled(String),
    #[error(
        "Multiple movies found with the title '{title}', from years {}",
        .years.iter().map(ToString::to_string).collect::<Vec<_>>().join(", ")
    )]
    MultipleMoviesTitled { title: String, years: Vec<i32> },
}
