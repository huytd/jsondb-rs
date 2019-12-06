# jsondb-rs

A self-hosted RESTful JSON storage service.

```
POST  /storage       create a new JSON entry
GET   /storage/{id}  read a JSON entry
PUT   /storage/{id}  update a JSON entry
```

---

To deploy this application on Heroku, follow these steps:

```
$ heroku buildpacks:set https://github.com/emk/heroku-buildpack-rust
$ git push heroku master
```