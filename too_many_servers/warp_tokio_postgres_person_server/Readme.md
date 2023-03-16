#  

```
 curl -vvv -X POST -d '{ "firstname" : "Max", "lastname": "Musterhabara" }' -H "Content-Type: application/json" http://localhost:3050/person
```


```
curl -vvv  http://localhost:3050/person
```



https://stackoverflow.com/questions/70637566/cannot-link-libpq-on-mac-m1

Add to     ~/.cargo/config.toml

```
[target.aarch64-apple-darwin]
rustflags = '-L /opt/homebrew/opt/libpq/lib -L /opt/homebrew/lib'
```
