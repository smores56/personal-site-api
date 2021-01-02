use std::collections::HashMap;

use juniper::GraphQLObject;
use serde::Deserialize;

#[derive(Deserialize, Clone)]
pub struct Recipe {
    pub name: String,
    #[serde(default)]
    pub notes: Option<String>,
    pub tags: Vec<String>,
    #[serde(default)]
    pub image: Option<String>,
    #[serde(default)]
    pub links: Vec<String>,
    pub steps: Vec<String>,
    pub ingredients: Ingredients,
    #[serde(default)]
    pub nutrition: Option<Nutrition>,
}

/// A recipe that I've kept around
#[juniper::graphql_object]
impl Recipe {
    /// The name of the dish
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Tags to quickly sort recipes
    pub fn tags(&self) -> &Vec<String> {
        &self.tags
    }

    /// A link to an image of the finished dish
    pub fn image(&self) -> &Option<String> {
        &self.image
    }

    /// Links to the original recipe
    pub fn links(&self) -> &Vec<String> {
        &self.links
    }

    /// Steps to cook the dish
    pub fn steps(&self) -> &Vec<String> {
        &self.steps
    }

    /// The necessary ingredients
    pub fn ingredients(&self) -> Vec<IngredientsForComponent> {
        match &self.ingredients {
            Ingredients::PlainList(ingredients) => vec![IngredientsForComponent {
                component: self.name.clone(),
                ingredients: ingredients.clone(),
            }],
            Ingredients::Components(components) => components
                .iter()
                .map(|(component, ingredients)| IngredientsForComponent {
                    component: component.clone(),
                    ingredients: ingredients.clone(),
                })
                .collect(),
        }
    }

    /// The nutritional info for the recipe
    pub fn nutrition(&self) -> &Option<Nutrition> {
        &self.nutrition
    }
}

#[derive(Deserialize, Clone)]
#[serde(untagged)]
pub enum Ingredients {
    PlainList(Vec<Ingredient>),
    Components(HashMap<String, Vec<Ingredient>>),
}

/// The ingredients for a component of a recipe
#[derive(Deserialize, GraphQLObject, Clone)]
pub struct IngredientsForComponent {
    /// The name of the component
    pub component: String,
    /// The ingredients for the component
    pub ingredients: Vec<Ingredient>,
}

/// An ingredient for a recipe
#[derive(Deserialize, Clone, GraphQLObject)]
pub struct Ingredient {
    /// The name of the item
    pub item: String,
    /// How much of the item is called for
    #[serde(default)]
    pub quantity: Option<String>,
    /// Additional notes about this ingredient
    #[serde(default)]
    pub notes: Option<String>,
    /// Potential substitutes if you don't have this ingredient handy
    #[serde(default)]
    pub substitutes: Vec<String>,
    /// Whether this ingredient is optional
    #[serde(default)]
    pub optional: bool,
}

/// The nutritional info for a recipe
#[derive(Deserialize, Clone, GraphQLObject)]
pub struct Nutrition {
    /// The number of servings it makes
    pub servings: Option<i32>,
    /// The size of each serving
    #[serde(rename = "serving-size")]
    pub serving_size: Option<String>,
    /// The calories (in grams) in each serving
    pub calories: Option<f64>,
    /// The fat (in grams) in each serving
    pub fat: Option<f64>,
    /// The carbohydrates (in grams) in each serving
    pub carbs: Option<f64>,
    /// The NET carbohydrates (in grams) in each serving
    #[serde(rename = "net-carbs")]
    pub net_carbs: Option<f64>,
    /// The protein (in grams) in each serving
    pub protein: Option<f64>,
    /// The fiber (in grams) in each serving
    pub fiber: Option<f64>,
}
