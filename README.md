# Actix-web REST API with JWT

A simple CRUD backend app using Actix-web, Diesel and JWT

## Require

- [Rust Stable](https://rustup.rs)
- [Postgres](https://www.postgresql.org/)

Or using [Docker](https://www.docker.com/)

## How to run

### Manual

- Rename `secret.key.sample` to `secret.key` or create your own key by running `head -c16 /dev/urandom > secret.key` in command line (Linux/UNIX only) and copy to `/src` folder
- Create a database in postgres cli or [pgAdmin](https://www.pgadmin.org/) tool
- Rename `.env.sample` to `.env` and update the database connection string in `DATABASE_URL` key.
- Build with release profile: `cargo build --release`
- Run release binary in command line/terminal.
  - Windows: `target/release/actix-web-rest-api-with-jwt.exe`
  - Linux/UNIX: `target/release/actix-web-rest-api-with-jwt`
- Enjoy! 😄

### Docker

- Enter into project directory and run `docker-compose up`
- Enjoy! 😄

## APIs

### Address: **`localhost:8000`**

### `GET /api/ping`: Ping
```bash
curl -X GET -i 'http://127.0.0.1:8000/api/ping'
```
- Response:
    - 200 OK
    ```
    pong!
    ```

### `POST /api/auth/signup`: Signup
```bash
curl -X POST -i 'http://127.0.0.1:8000/api/auth/signup' \
-H "Content-Type: application/json" --data '{"username": "c",
   "email": "c",
   "password": "c" }'
```

  - Request body:
  ```
  {
     "username": string,
     "email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "signup successfully",
       "data": ""
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "User '{username}' is already registered",
       "data": ""
    }
    ```

### `POST /api/auth/login`: Login
```bash
curl -X POST -H 'Content-Type: application/json' -i 'http://127.0.0.1:8000/api/auth/login'  \
 --data '{"username_or_email":"c",  "password":"c"}'
```
  - Request body:
  ```
  {
     "username_or_email": string,
     "password": string       // a raw password
  }
  ```
  - Response
    - 200 OK
    ```
    {
       "message": "login successfully",
       "data": {
         "token": string      // bearer token
       }
    }
    ```
    - 400 Bad Request
    ```
    {
       "message": "wrong username or password, please try again",
       "data": ""
    }
    ```

### `GET /api/address-book`: Get all people information
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": [
          {
            "id": int32,
            "name": string,
            "gender": boolean,      // true for male, false for female
            "age": int32,
            "address": string,
            "phone": string,
            "email": string
          }
       ]
    }
    ```

### `GET /api/address-book/{id}`: Get person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": {
         "id": int32,
         "name": string,
         "gender": boolean,      // true for male, false for female
         "age": int32,
         "address": string,
         "phone": string,
         "email": string
       }
    }
    ```
    - 404 Not Found
    ```
    {
       "message": "person with id {id} not found",
       "data": ""
    }
    ```

### `GET /api/address-book/{query}`: Search for person information by keyword
  - Param path:
    - query: string
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
       "message": "ok",
       "data": [
         {
           "id": int32,
           "name": string,
           "gender": boolean,      // true for male, false for female
           "age": int32,
           "address": string,
           "phone": string,
           "email": string
         }
       ]
    }
    ```

### `POST /api/address-book`: Add person information
  - Header:
    - Authorization: bearer \<token\>
  - Request body:
    ```
    {
      "name": string,
      "gender": boolean,      // true for male, false for female
      "age": int32,
      "address": string,
      "phone": string,
      "email": string
    }
    ```
  - Response
    - 201 Created
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not insert data",
      "data": ""
    }
    ```  

### `PUT /api/address-book/{id}`: Update person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Request body:
  ```
  {
    "name": string,
    "gender": boolean,      // true for male, false for female
    "age": int32,
    "address": string,
    "phone": string,
    "email": string
  }
  ```
  - Response
    - 200 OK
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not update data",
      "data": ""
    }
    ```

### `DELETE /api/address-book/{id}`: Delete person information by id
  - Param path:
    - id: int32
  - Header:
    - Authorization: bearer \<token\>
  - Response
    - 200 OK
    ```
    {
      "message": "ok",
      "data": ""
    }
    ```
    - 500 Internal Server Error
    ```
    {
      "message": "can not delete data",
      "data": ""
    }
    ```

### Errors:
  - Invalid or missing token
    - Status code: 401 Unauthorized
    - Response:
    ```
    {
      "message": "invalid token, please login again",
      "data": ""
    }
    ```
