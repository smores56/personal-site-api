use std::{collections::HashSet, iter::once};

use juniper::{EmptyMutation, EmptySubscription, FieldResult, RootNode};

use self::context::Context;
use crate::review::{Review, Reviews};
use crate::{error::Error, recipe::Recipe};

pub mod context;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
}

pub struct Query;

#[juniper::graphql_object(context = Context)]
impl Query {
    /// Find a recipe by name
    pub fn recipe(name: String, context: &Context) -> FieldResult<Recipe> {
        let recipes = context.recipes.read_collection();
        let recipe = recipes
            .values()
            .find(|recipe| recipe.name == name)
            .ok_or_else(|| Error::NoRecipeNamed(name))?;

        Ok(recipe.clone())
    }

    /// List all recipes with one of the given tags. If no tags are given, returns all recipes.
    pub fn recipes(tags: Option<Vec<String>>, context: &Context) -> Vec<Recipe> {
        let tags = tags.unwrap_or_default();

        context
            .recipes
            .read_collection()
            .values()
            .filter(|recipe| tags.is_empty() || tags.iter().any(|tag| recipe.tags.contains(tag)))
            .cloned()
            .collect()
    }

    /// List all tags found in any recipe
    pub fn recipe_tags(context: &Context) -> Vec<String> {
        let all_tags: HashSet<String> = context
            .recipes
            .read_collection()
            .values()
            .flat_map(|recipe| recipe.tags.iter().cloned())
            .collect();

        all_tags.into_iter().collect()
    }

    /// Find a review by movie title (and optionally the year)
    pub fn review(title: String, year: Option<i32>, context: &Context) -> FieldResult<Review> {
        let reviews = context.reviews.read_collection();
        let reviews = reviews
            .values()
            .flat_map(|reviews| match reviews {
                Reviews::Single(review) => {
                    Box::new(once(review)) as Box<dyn Iterator<Item = &'_ Review>>
                }
                Reviews::List(review_list) => Box::new(review_list.iter()),
            })
            .filter(|review| {
                review.title == title
                    && year
                        .map(|y| review.year.map(|rt| rt == y).unwrap_or(false))
                        .unwrap_or(true)
            })
            .collect::<Vec<_>>();

        match &reviews[..] {
            &[] => Err(Error::NoReviewTitled(title).into()),
            &[review] => Ok(review.clone()),
            multiple => {
                let mut years = multiple
                    .iter()
                    .flat_map(|r| r.year.clone())
                    .collect::<Vec<i32>>();
                years.sort();
                Err(Error::MultipleMoviesTitled { title, years }.into())
            }
        }
    }

    /// List all movie reviews
    pub fn reviews(context: &Context) -> Vec<Review> {
        context
            .reviews
            .read_collection()
            .values()
            .flat_map(|reviews| match reviews {
                Reviews::Single(review) => {
                    Box::new(once(review)) as Box<dyn Iterator<Item = &'_ Review>>
                }
                Reviews::List(review_list) => Box::new(review_list.iter()),
            })
            .cloned()
            .collect()
    }
}
