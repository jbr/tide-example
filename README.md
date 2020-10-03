# Tide Example Application

This application is intended to eventually represent best practices in
developing a tide application. The actual features are modeled closely
after [the ruby on rails getting
started](https://guides.rubyonrails.org/getting_started.html)
tutorial, with the intent that when this application is succinct and
ergonomic enough, a similar tutorial will be a useful introduction.

Current crate choices include:
* [sqlx](https://github.com/launchbadge/sqlx) with sqlite as a datastore
* [askama](https://github.com/djc/askama) as a html template engine

## Running this repository locally

You need to add two environment variables to run this application,
`DATABASE_URL` and `TIDE_SECRET`.

### Database setup

This application is built on the [sqlite
database](https://www.sqlite.org/), and accepts database urls of the
form `"sqlite:///path/to/sqlite.db"`.

This repository does yet not use migrations. In order to set up the database, run
```bash
$ sqlite3 db/sqlite3.db < db/schema.sql
```

You will also need to add `DATABASE_URL` to your environment. We
recommend using [`direnv`](https://direnv.net/), and include an
example .envrc.

``` bash
export DATABASE_URL="sqlite://./db/sqlite3.db"
```

### Sessions
The `TIDE_SECRET` needs to be a cryptographically random key of at
least 32 bytes in your execution environment. An easy way to generate
it is:

``` bash
$ openssl rand -base64 64
```

### Running the app

``` bash
cargo run
```

# Contributing

Pull requests and fork variants very welcome. Any variants that aren't
merged into this repo will be linked here along with decriptions of
the libraries, design decisons, and tradeoffs.

# Tide version

This application will always represent the most recent crate release
    of tide: <img
    src="https://img.shields.io/crates/v/tide.svg?style=flat-square"
    alt="Crates.io version" />
