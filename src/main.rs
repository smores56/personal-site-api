use std::{path::PathBuf, sync::Arc};

use clap::Clap;
use recipe::Recipe;
use review::Reviews;
use schema::context::Context;
use warp::Filter;
use watcher::CollectionWatcher;

pub mod error;
pub mod recipe;
pub mod review;
pub mod schema;
pub mod watcher;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    env_logger::init();
    let args = Args::parse();

    let recipe_watcher = Arc::new(CollectionWatcher::<Recipe>::new(args.recipe_dir));
    let review_watcher = Arc::new(CollectionWatcher::<Reviews>::new(args.review_dir));

    recipe_watcher.clone().watch()?;
    review_watcher.clone().watch()?;

    let context = warp::any().map(move || Context {
        recipes: recipe_watcher.clone(),
        reviews: review_watcher.clone(),
    });

    log::info!("Listening on 127.0.0.1:{}", args.port);

    let graphiql = warp::get()
        .and(warp::path("graphiql"))
        .and(juniper_warp::graphiql_filter("/", None));
    let graphql = warp::post()
        .and(warp::path::end())
        .and(juniper_warp::make_graphql_filter(
            schema::schema(),
            context.boxed(),
        ));

    warp::serve(graphql.or(graphiql).with(warp::log("warp_server")))
        .run(([127, 0, 0, 1], args.port))
        .await;

    Ok(())
}

#[derive(Debug, Clap)]
pub struct Args {
    #[clap(long, env)]
    pub recipe_dir: PathBuf,
    #[clap(long, env)]
    pub review_dir: PathBuf,
    #[clap(long, env)]
    pub port: u16,
}
