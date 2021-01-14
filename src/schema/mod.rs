use juniper::{EmptyMutation, EmptySubscription, RootNode};

use self::context::Context;
use self::query::Query;

pub mod context;
pub mod query;

pub type Schema = RootNode<'static, Query, EmptyMutation<Context>, EmptySubscription<Context>>;

pub fn schema() -> Schema {
    Schema::new(
        Query,
        EmptyMutation::default(),
        EmptySubscription::default(),
    )
}
