<p align="center"><img src="logo.svg" width="100px;" height="100px" /></p>
<p align="center"><b>jsondb-rs</b></p>

---

JSONdb is a RESTful JSON storage service that you can
use to quickly develop your mobile and web app, without
the need of any backend.

```
POST  /storage       create a new JSON entry
GET   /storage/{id}  read a JSON entry
PUT   /storage/{id}  update a JSON entry
```

---

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