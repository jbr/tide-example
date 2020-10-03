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

# Running it

You need to add two environment variables to try this out:

``` bash
export DATABASE_URL="sqlite::memory"
export TIDE_SECRET="..."
```

The `TIDE_SECRET` needs to be a 32byte key, an easy way to generate it is:

``` bash
export TIDE_SECRET=`date|md5`
```

After doing this, you can do:

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
