# ProxyThingis

Reverse Proxy API GateWay Load Balancer - a laymans implemention attempt using rust

## postgresql

add path to .zshrc

## simple "testing"

```
wrk -t12 -c400 -d30s http://127.0.0.1:3031/hallo
```

```
curl -H "Authorization: Bearer with some JWT Token" http://localhost:3031/post_json?parameter1=has_aa_value&param2=val2 
```

```
curl -d '{"ksdsdsdsdey1":"vsdsdsdadsadaadlue1", "kasdadadey2":"vasdadadasdasdalue2"}' -H "Content-Type: application/json" -X POST http://localhost:3031/post_json?parameter1=has_aa_value&param2=val2 | jq
```

```
curl -d '{"ksdsdsdsdey1":"vsdsdsdadsadaadlue1", "kasdadadey2":"vasdadadasdasdalue2"}' -H "Authorization: Bearer with some JWT Token" -H "Content-Type: application/json" -X POST http://localhost:3031/post_json\?parameter1\=has_aa_value\&param2\=val2 | jq
```

## warp-proxy-v4

### add new source server

```
curl -d '{"description":"new server", "path_starts_with":"/api/person", "method": "GET"}'   -H "Content-Type: application/json" -X POST http://localhost:3034/proxythingi/server/source | jq
```

### add new target server

```
curl -d '{"description":"new target for new server", "schema":"http", "host": "localhost", "port": 1234, "path": "/api/person", "method": "GET", "active": true, "source": 4}'   -H "Content-Type: application/json" -X POST http://localhost:3034/proxythingi/server/target | jq
```

### list servers

```
curl http://localhost:3034/proxythingi/server | jq
```


## Stats

### get stats (currently whole server config)
```
curl http://localhost:3034/proxythingi/stats | jq
```

### store stats in DB
```
curl -X POST http://localhost:3034/proxythingi/stats | jq
```


### reset stats in memory
```
curl -X DELETE http://localhost:3034/proxythingi/stats | jq
```



## Database stuff

```
CREATE DATABASE bumzack;

\l

\c bumzack; 
```

```
CREATE TABLE IF NOT EXISTS person (
     id serial PRIMARY KEY,
    firstname VARCHAR ( 50 )   NOT NULL,
    lastname VARCHAR ( 50 ) NOT NULL,
created TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);
```

```
\d person;
```

```
INSERT INTO person (firstname, lastname) VALUES ('max', 'mustermann');
```

```
INSERT INTO person (firstname, lastname) VALUES ('Matilde', 'Bernhard');
INSERT INTO person (firstname, lastname) VALUES ('Cruz', 'Stiedemann');
INSERT INTO person (firstname, lastname) VALUES ('Cade', 'Collier');
INSERT INTO person (firstname, lastname) VALUES ('Herman', 'Reichert');
INSERT INTO person (firstname, lastname) VALUES ('Leland', 'Jones'); 
INSERT INTO person (firstname, lastname) VALUES ('Vern', 'Harber');
INSERT INTO person (firstname, lastname) VALUES ('Ottilie', 'Klocko');  
INSERT INTO person (firstname, lastname) VALUES ('Zachery', 'Volkman');
INSERT INTO person (firstname, lastname) VALUES ('Megane', 'Wuckert');
INSERT INTO person (firstname, lastname) VALUES ('Alessandro', 'Wilkinson');
INSERT INTO person (firstname, lastname) VALUES ('Eugenia', 'Jast');
INSERT INTO person (firstname, lastname) VALUES ('Stephania', 'Blanda');
INSERT INTO person (firstname, lastname) VALUES ('Luella', 'O');Conner
INSERT INTO person (firstname, lastname) VALUES ('Miss', 'Aurelio');  
INSERT INTO person (firstname, lastname) VALUES ('Jaydon', 'Terry');
INSERT INTO person (firstname, lastname) VALUES ('Van', 'Schowalter');
INSERT INTO person (firstname, lastname) VALUES ('Muhammad', 'Moen');
INSERT INTO person (firstname, lastname) VALUES ('Luz', 'Auer'); II
INSERT INTO person (firstname, lastname) VALUES ('Queen', 'Kilback');
INSERT INTO person (firstname, lastname) VALUES ('Aliya', 'Nicolas');
INSERT INTO person (firstname, lastname) VALUES ('Jerrold', 'Bailey');
INSERT INTO person (firstname, lastname) VALUES ('Ottis', 'Hane');
INSERT INTO person (firstname, lastname) VALUES ('Connie', 'Mills');
INSERT INTO person (firstname, lastname) VALUES ('Kyra', 'Kling');
INSERT INTO person (firstname, lastname) VALUES ('Manuela', 'Kunze');
INSERT INTO person (firstname, lastname) VALUES ('Shea', 'Watsica');  
INSERT INTO person (firstname, lastname) VALUES ('Grover', 'Gutkowski');  
INSERT INTO person (firstname, lastname) VALUES ('Bettie', 'Altenwerth');
INSERT INTO person (firstname, lastname) VALUES ('Graciela', 'Kulas');  
INSERT INTO person (firstname, lastname) VALUES ('Odell', 'Denesik');
INSERT INTO person (firstname, lastname) VALUES ('Darian', 'Eichmann');
INSERT INTO person (firstname, lastname) VALUES ('Alvah', 'Gusikowski');
INSERT INTO person (firstname, lastname) VALUES ('Maudie', 'Towne');
INSERT INTO person (firstname, lastname) VALUES ('Makenna', 'Schumm');
INSERT INTO person (firstname, lastname) VALUES ('Dangelo', 'Zieme');
INSERT INTO person (firstname, lastname) VALUES ('Hillard', 'Bahringer');
INSERT INTO person (firstname, lastname) VALUES ('Hortense', 'Harris');
INSERT INTO person (firstname, lastname) VALUES ('Tyrell', 'Denesik');
INSERT INTO person (firstname, lastname) VALUES ('Jordon', 'Crist');
INSERT INTO person (firstname, lastname) VALUES ('Isac', 'Treutel');
INSERT INTO person (firstname, lastname) VALUES ('Piper', 'Olson');
INSERT INTO person (firstname, lastname) VALUES ('Lottie', 'Rohan');
INSERT INTO person (firstname, lastname) VALUES ('Ambrose', 'Goldner');
INSERT INTO person (firstname, lastname) VALUES ('Jade', 'Huels');
INSERT INTO person (firstname, lastname) VALUES ('Kavon', 'Wilkinson');
INSERT INTO person (firstname, lastname) VALUES ('Angela', 'Friesen');
INSERT INTO person (firstname, lastname) VALUES ('Zoila', 'Eichmann');
INSERT INTO person (firstname, lastname) VALUES ('Garrison', 'Mitchell');  
INSERT INTO person (firstname, lastname) VALUES ('Junior', 'Murazik');  
INSERT INTO person (firstname, lastname) VALUES ('Susie', 'Larkin');
INSERT INTO person (firstname, lastname) VALUES ('Berniece', 'Zboncak');
INSERT INTO person (firstname, lastname) VALUES ('Ward', 'Bogan');
INSERT INTO person (firstname, lastname) VALUES ('Doug', 'Leffler');  
INSERT INTO person (firstname, lastname) VALUES ('Nona', 'Carroll'); 
INSERT INTO person (firstname, lastname) VALUES ('Lori', 'Homenick'); 
INSERT INTO person (firstname, lastname) VALUES ('Cristian', 'Jerde');
INSERT INTO person (firstname, lastname) VALUES ('Patience', 'Marquardt'); 
INSERT INTO person (firstname, lastname) VALUES ('Aurelia', 'Wunsch');
INSERT INTO person (firstname, lastname) VALUES ('Dion', 'Runolfsson'); 
INSERT INTO person (firstname, lastname) VALUES ('Maxwell', 'Wyman');
INSERT INTO person (firstname, lastname) VALUES ('Casimir', 'Hilpert');
INSERT INTO person (firstname, lastname) VALUES ('Martin', 'Feeney');
INSERT INTO person (firstname, lastname) VALUES ('Dakota', 'Von');
INSERT INTO person (firstname, lastname) VALUES ('Yvonne', 'Veum');
INSERT INTO person (firstname, lastname) VALUES ('Jewell', 'Lemke');
INSERT INTO person (firstname, lastname) VALUES ('Maurine', 'Corwin');
INSERT INTO person (firstname, lastname) VALUES ('Katheryn', 'Cole');  
INSERT INTO person (firstname, lastname) VALUES ('Gonzalo', 'Bednar');
INSERT INTO person (firstname, lastname) VALUES ('Janis', 'Bahringer');
INSERT INTO person (firstname, lastname) VALUES ('Bernadine', 'Sawayn');
INSERT INTO person (firstname, lastname) VALUES ('Rashad', 'Kiehn');
INSERT INTO person (firstname, lastname) VALUES ('Richmond', 'Heathcote');
INSERT INTO person (firstname, lastname) VALUES ('Toy', 'Mohr');
INSERT INTO person (firstname, lastname) VALUES ('Talia', 'OConnell');
INSERT INTO person (firstname, lastname) VALUES ('Luther', 'Lebsack');
INSERT INTO person (firstname, lastname) VALUES ('Rashad', 'Will');  
INSERT INTO person (firstname, lastname) VALUES ('Emma', 'Jerde');  
INSERT INTO person (firstname, lastname) VALUES ('Twila', 'Huels');
INSERT INTO person (firstname, lastname) VALUES ('Mabel', 'Kuhn');
INSERT INTO person (firstname, lastname) VALUES ('Melody', 'Schimmel');
INSERT INTO person (firstname, lastname) VALUES ('Weldon', 'OConnell');
INSERT INTO person (firstname, lastname) VALUES ('Tracey', 'Goodwin');
INSERT INTO person (firstname, lastname) VALUES ('Camren', 'Jaskolski');
INSERT INTO person (firstname, lastname) VALUES ('Isaac', 'Sanford');
INSERT INTO person (firstname, lastname) VALUES ('Toy', 'Emard');
INSERT INTO person (firstname, lastname) VALUES ('Ebony', 'Breitenberg');
INSERT INTO person (firstname, lastname) VALUES ('Jon', 'Kertzmann');  
INSERT INTO person (firstname, lastname) VALUES ('Iliana', 'Rath');
INSERT INTO person (firstname, lastname) VALUES ('Amari', 'Moen');
INSERT INTO person (firstname, lastname) VALUES ('Janice', 'Langosh');
INSERT INTO person (firstname, lastname) VALUES ('Kaylee', 'Trantow');  
INSERT INTO person (firstname, lastname) VALUES ('Karl', 'Lesch');
INSERT INTO person (firstname, lastname) VALUES ('Delilah', 'Olson');
INSERT INTO person (firstname, lastname) VALUES ('Cale', 'Buckridge');
INSERT INTO person (firstname, lastname) VALUES ('Oswaldo', 'Hoppe');
INSERT INTO person (firstname, lastname) VALUES ('Caroline', 'McLaughlin');
INSERT INTO person (firstname, lastname) VALUES ('Flo', 'Wilderman');
INSERT INTO person (firstname, lastname) VALUES ('Esteban', 'Monahan');
INSERT INTO person (firstname, lastname) VALUES ('Cleve', 'Hilll');
INSERT INTO person (firstname, lastname) VALUES ('Celine', 'Beier');
```
