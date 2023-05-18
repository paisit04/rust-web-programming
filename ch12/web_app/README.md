# web_app

```sh
cargo run config.yml
```

## Item APIs

Get item list

```javascript
GET http://localhost:8000/v1/item/get HTTP/1.1
```

Create an item

```javascript
POST http://localhost:8000/v1/item/create/washing HTTP/1.1
```

Edit an item

```javascript
POST http://localhost:8000/v1/item/edit HTTP/1.1

{
    "title": "washing",
    "status": "DONE"
}
```

## User APIs

Create a user

```javascript
POST http://localhost:8000/v1/user/create HTTP/1.1

{
    "name": "maxwell",
    "email": "test@gmail.com",
    "password": "test"
}
```

## Auth APIs

Login

```javascript
GET http://localhost:8000/v1/auth/login HTTP/1.1

{
    "username": "maxwell",
    "password": "test"
}
```
