# ProxyThingis

```
ulimit -a
```

### add to your bashrc oder zshrc

```
ulimit -n 100000
```

```
sudo lsof -n | cut -f1 -d' ' | uniq -c | sort | tail
```

## ubuntu postgresql install

sudo -u postgres psql
create database bumzack;
create user bumzack with encrypted password 'bumzack';
grant all privileges on database bumzack to bumzack;

sudo -u bumzack psql bumzack

insert sql script

## start as service

Reverse Proxy API GateWay Load Balancer - a laymans implemention attempt using rust

## postgresql

add path to .zshrc

## simple "testing"

```
wrk -t12 -c400 -d30s http://127.0.0.1:3036/hallo
```

```
curl -H "Authorization: Bearer with some JWT Token" http://localhost:3036/post_json?parameter1=has_aa_value&param2=val2 
```

```
curl -d '{"ksdsdsdsdey1":"vsdsdsdadsadaadlue1", "kasdadadey2":"vasdadadasdasdalue2"}' -H "Content-Type: application/json" -X POST http://localhost:3036/post_json?parameter1=has_aa_value&param2=val2 | jq
```

```
curl -d '{"ksdsdsdsdey1":"vsdsdsdadsadaadlue1", "kasdadadey2":"vasdadadasdasdalue2"}' -H "Authorization: Bearer with some JWT Token" -H "Content-Type: application/json" -X POST http://localhost:3036/post_json\?parameter1\=has_aa_value\&param2\=val2 | jq
```

## warp-proxy-v5

### add new source server

```
curl -d '{"description":"new server", "path_starts_with":"/api/person", "method": "GET"}'   -H "Content-Type: application/json" -X POST http://localhost:3036/proxythingi/server/source | jq
```

### add new target server

```
curl -d '{"description":"new target for new server", "schema":"http", "host": "localhost", "port": 1234, "path": "/api/person", "method": "GET", "active": true, "source": 4}'   -H "Content-Type: application/json" -X POST http://localhost:3036/proxythingi/server/target | jq
```

### list servers

```
curl http://localhost:3036/proxythingi/server | jq
```

### Activate target server

```
curl http://localhost:3036/proxythingi/server/activate/2 | jq

curl http://localhost:3036/proxythingi/server/activate/3 | jq
```

### Dectivate target server

```
curl http://localhost:3036/proxythingi/server/deactivate/2 | jq

curl http://localhost:3036/proxythingi/server/deactivate/3 | jq
```

### prod

```
curl http://proxy.proxythingi.at/proxythingi/server/deactivate/8 | jq
```

## Stats

### get stats (currently whole server config)

```
curl http://localhost:3036/proxythingi/stats | jq
```

### store stats in DB

```
curl -X POST http://localhost:3036/proxythingi/stats | jq
```

### reset stats in memory

```
curl -X DELETE http://localhost:3036/proxythingi/stats | jq
```
