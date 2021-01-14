use std::{path::PathBuf, sync::Arc};

use clap::Clap;
use schema::context::Context;
use warp::Filter;

pub mod error;
pub mod recipe;
pub mod review;
pub mod schema;

#[tokio::main]
async fn main() -> Result<(), error::Error> {
    env_logger::init();
    let args = Args::parse();

    let review_dir = Arc::new(args.review_dir);
    let recipe_dir = Arc::new(args.recipe_dir);
    let context = warp::any().map(move || Context::new(&recipe_dir, &review_dir));

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
    let cors = warp::cors()
        .allow_any_origin()
        .allow_methods(vec!["GET", "POST", "OPTIONS"])
        .allow_headers(vec!["content-type", "accept"]);

    warp::serve(
        graphql
            .or(graphiql)
            .with(cors)
            .with(warp::log("warp_server")),
    )
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
