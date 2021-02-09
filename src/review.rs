use juniper::GraphQLObject;
use serde::Deserialize;

/// My thoughts on a film
#[derive(Deserialize, Clone, GraphQLObject)]
pub struct Review {
    /// The title of the film
    pub title: String,
    /// When the movie was released
    pub year: Option<i32>,
    /// My rating of the film (empty if I haven't seen it yet)
    pub rating: Option<f64>,
    /// Additional thoughts on the film
    pub review: Option<String>,
    /// A link to the movie page (ex. IMDb, Wikipedia, etc.)
    pub link: Option<String>,
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum Reviews {
    Single(Review),
    List(Vec<Review>),
}
