use std::collections::HashSet;

use juniper::FieldResult;

use super::context::Context;
use crate::error::Error;
use crate::recipe::Recipe;
use crate::review::Review;

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Find a recipe by name
    pub fn recipe(name: String, context: &Context) -> FieldResult<&Recipe> {
        context
            .recipes
            .iter()
            .find(|recipe| recipe.name == name)
            .ok_or_else(|| Error::NoRecipeNamed(name).into())
    }

    /// List all recipes with one of the given tags. If no tags are given, returns all recipes.
    pub fn recipes(tags: Option<Vec<String>>, context: &Context) -> Vec<&Recipe> {
        let tags = tags.unwrap_or_default();

        context
            .recipes
            .iter()
            .filter(|recipe| tags.is_empty() || tags.iter().any(|tag| recipe.tags.contains(tag)))
            .collect()
    }

    /// List all tags found in any recipe
    pub fn recipe_tags(context: &Context) -> Vec<&String> {
        let all_tags: HashSet<&String> = context
            .recipes
            .iter()
            .flat_map(|recipe| &recipe.tags)
            .collect();
        let mut tag_list: Vec<&String> = all_tags.into_iter().collect();
        tag_list.sort();

        tag_list
    }

    /// Find a review by movie title (and optionally the year)
    pub fn review(title: String, year: Option<i32>, context: &Context) -> FieldResult<Review> {
        let mut reviews = context
            .reviews
            .iter()
            .filter(|review| {
                review.title == title
                    && year
                        .map(|y| review.year.map(|rt| rt == y).unwrap_or(false))
                        .unwrap_or(true)
            })
            .peekable();

        match reviews.next() {
            None => Err(Error::NoReviewTitled(title).into()),
            Some(review) => {
                if reviews.peek().is_none() {
                    Ok(review.clone())
                } else {
                    let mut years = reviews
                        .flat_map(|r| r.year.clone())
                        .chain(review.year)
                        .collect::<Vec<i32>>();
                    years.sort();

                    Err(Error::MultipleMoviesTitled { title, years }.into())
                }
            }
        }
    }

    /// List all movie reviews
    pub fn reviews(context: &Context) -> &Vec<Review> {
        &context.reviews
    }

    /// List all movies with no review yet
    pub fn unreviewed_films(context: &Context) -> Vec<&Review> {
        context
            .reviews
            .iter()
            .filter(|review| review.rating.is_none() && review.review.is_none())
            .collect()
    }
}
