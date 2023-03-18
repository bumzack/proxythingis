#            

## Install PostGres on Mac M1

## cURL Warp Tokio PostgreSQL (tokio-postgresql is non blocking aka async)

Port: 3050

```
curl -vvv -X POST -d '{ "firstname" : "Max", "lastname": "Musterhabara" }' -H "Content-Type: application/json" http://localhost:3050/person
```

```
curl -vvv  http://localhost:3050/person
```

## cURL Warp Diesel PostgreSQL (Diesel is blocking)

Port: 3060

```
curl -vvv -X POST -d '{ "firstname" : "Max", "lastname": "Musterhabara" }' -H "Content-Type: application/json" http://localhost:3060/person
```

```
curl -vvv  http://localhost:3060/person
```

