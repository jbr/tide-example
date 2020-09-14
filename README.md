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

# Run the example
This example-project uses a local sqlite database.
To connect to this database the `$DATABASE_URL` environment variable needs
to be set. This project also uses the `$TIDE_SECRET` variable for the
session middleware.
It can be useful to set these variables on your development environment
using a `.env` file. An example `.env.example` file is supplied with a
`$DATABASE_URL` value that will let you create a local database file with
the name `database.db` in your project folder. A `$TIDE_SECRET` value
still needs to be supplied.

To set up the database and run the migrations in the `migrations` folder
the `sqlx-cli` tool can be used. This tool is still in beta but works
fine. It can be installed using the followig command.

```
> cargo install sqlx-cli --version 0.1.0-beta.1 --no-default-features --features sqlite
```

The version parameter is neccesary for now because without it cargo
install will try to install version 0.0.1 which does not work. The features
are there to make sure that you install this tool for use with a sqlite database.

To create the database and run the migrations you can run the following commands
from the project root;
```
> sqlx database create
```
and 
```
> sqlx migrate run
````

The sqlx tool will automatically pick up the environment variables in the
`.env` file.

To start the tide server you can run
```
> source .env
```
to set up the environment and
```
> cargo run
```
to start the application.


# Contributing

Pull requests and fork variants very welcome. Any variants that aren't
merged into this repo will be linked here along with decriptions of
the libraries, design decisons, and tradeoffs.

# Tide version

This application will always represent the most recent crate release
    of tide: <img
    src="https://img.shields.io/crates/v/tide.svg?style=flat-square"
    alt="Crates.io version" />
