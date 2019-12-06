# jsondb-rs

A self-hosted RESTful JSON storage service.

```
POST  /storage       create a new JSON entry
GET   /storage/{id}  read a JSON entry
PUT   /storage/{id}  update a JSON entry
```

---

To clone and deploy this application on Heroku, follow these steps:

```
$ clone https://github.com/huytd/jsondb-rs
$ cd jsondb-rs
$ heroku apps:create
$ heroku addons:create heroku-postgresql
$ heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust
$ git push heroku master
```