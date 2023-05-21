# Redis Queue

Run a server

```sh
APP_TYPE=server REDIS_URL=redis://localhost:6379 cargo run
```

Run a worker

```sh
APP_TYPE=worker REDIS_URL=redis://localhost:6379 cargo run
```

Client

```sh
curl --location 'http://127.0.0.1:3000/multiply' \
--header 'Content-Type: application/json' \
--data '{
    "one": 3,
    "two": 3
}'

curl --location 'http://127.0.0.1:3000/subtract' \
--header 'Content-Type: application/json' \
--data '{
    "one": 9,
    "two": 4
}'

curl --location 'http://127.0.0.1:3000/add' \
--header 'Content-Type: application/json' \
--data '{
    "one": 4,
    "two": 4
}'
```
