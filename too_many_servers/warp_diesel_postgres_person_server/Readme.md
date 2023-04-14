# Diesel Postgres

## install

```
cargo install diesel_cli --no-default-features --features postgres
```

diesel migration run

## do stuff

```
 curl -vvv -X POST -d '{ "firstname" : "Max", "lastname": "Musterhabara" }' -H "Content-Type: application/json" http://localhost:3050/person
```

```
curl -vvv  http://localhost:3050/person
```

brew install postgresql@15 libpq
