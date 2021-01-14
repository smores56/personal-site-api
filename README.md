# Mohr Codes API

The GraphQL API backend for my personal site,
[sam.mohr.codes][personal site]. You can find the API hosted
at [api.mohr.codes][api].


## How it Works

My website hosts all of my movie (and TV show) reviews as well
as my recipes. Rather than storing them in a database like
[SQLite][sqlite], I wanted to store them in a human-readable format.
Thus, all of my records are stored in YAML files that are cloud-synced
between all of my personal machines and the server that this API
is hosted on using [MEGASync][megasync]. Per request, this API
parses every YAML file in each directory and serves them over a
JSON API, exposed with [GraphQL][graphql].

You can see the definitions for all exposed collections of data at the 
[GraphiQL][api graphiql] page for this API.


## Usage

You'll need [Rust installed][install rust] to run this API yourself.

To install it to your system, simply run the following:

```console
$ cargo install --git https://github.com/smores56/mohr-codes-api
```

You can then run the API by providing a port to listen on and
(optionally nested) folders of reviews and recipes like so:

```console
$ mohr-codes-api --port=1234 --review-dir=reviews/ --recipe-dir=recipes/
```


[sqlite]: https://sqlite.org/index.html
[megasync]: https://mega.nz/sync
[graphql]: https://graphql.org/
[api graphiql]: https://api.mohr.codes/graphiql
[personal site]: https://sam.mohr.codes
[api]: https://api.mohr.codes
[install rust]: https://www.rust-lang.org/tools/install
