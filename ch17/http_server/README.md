# Http Server

Server

```sh
export SERVER_URL="https://httpbin.org/post"
cargo run
```

Client

```sh
curl --location 'http://127.0.0.1:3000/test' \
--header 'Content-Type: application/json' \
--data '{
    "chat_id": 23,
    "input": "what is your name",
    "output": "my name is maxwell",
    "timestamp": 1
}'
```
