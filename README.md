<p align="center"><img src="logo.svg" width="100px;" height="100px" /></p>
<p align="center"><b>jsondb-rs</b></p>

# Usage

JSONdb is a RESTful JSON storage service that you can
use to quickly develop your mobile and web app, without
the need of any backend.

```
POST  /storage       create a new JSON entry
GET   /storage/{id}  read a JSON entry
PUT   /storage/{id}  update a JSON entry
```

# Deploy to Heroku

To deploy this application on Heroku, your have two options:

**Option 1: Click to deploy:**

[![Deploy](https://www.herokucdn.com/deploy/button.svg)](https://heroku.com/deploy)

**Option 2: Deploy manually:**

```
$ clone https://github.com/huytd/jsondb-rs
$ cd jsondb-rs
$ heroku apps:create
$ heroku addons:create heroku-postgresql
$ heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust
$ git push heroku master
```

**Note:** If you're using the 1-core Heroku free dyno, please expect
the build process to be 10-20 minutes at worst.

# Run on your own server

To run on your own server, assuming you already have these installed:

- [Rust](https://rustup.rs/) (stable edition)
- [PostgreSQL](https://www.postgresql.org/)
- [Diesel CLI](https://github.com/diesel-rs/diesel/tree/master/diesel_cli)

You can install `diesel-cli` using the following command:

```
$ cargo install diesel_cli --no-default-features --features postgres
```

First, clone the repo:

```
$ clone https://github.com/huytd/jsondb-rs
$ cd jsondb-rs
```

Create the `.env` file to config your `DATABASE_URL`, in the project's root:

```
$ echo DATABASE_URL=postgres://<username>:<password>@127.0.0.1/jsondb > .env
```

Initialize the database:

```
$ diesel setup
```

Run the migration to create the data tables:

```
$ diesel migration run
```

Run the app server:

```
$ cargo run --release
```

If everything is running, you can start [setup a systemd service](https://www.steadylearner.com/blog/read/How-to-deploy-Rust-Web-App#5-create-systemd-service-to-serve-your-web-application) to serve your app
and put it behind nginx, or so.
