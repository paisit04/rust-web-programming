# web_app

Get item list

```javascript
GET http://localhost:8080/v1/item/get HTTP/1.1
```

Create an item

```javascript
POST http://localhost:8080/v1/item/create/washing HTTP/1.1
```

Edit an item

```javascript
POST http://localhost:8080/v1/item/edit HTTP/1.1

{
    "title": "washing",
    "status": "DONE"
}
```
