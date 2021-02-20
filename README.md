# Actix-web REST API with JWT

![CI](https://github.com/SakaDream/actix-web-rest-api-with-jwt/workflows/CI/badge.svg)
![Docker CICD](https://github.com/SakaDream/actix-web-rest-api-with-jwt/workflows/Docker%20CICD/badge.svg)

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

- Enter into project directory
- Run `docker-compose -f docker-compose.local.yml up` for local environment or `docker-compose -f docker-compose.prod.yml up` for production environment
- Enjoy! 😄

### Note for yew-address-book-client
- I also made [yew-address-book-client](https://github.com/SakaDream/yew-address-book-client), an Address Book Frontend using yew-rs.
- yew-address-book-client is heavily under in development, currently the web client does not have login/signup page, so if you want to integrate with backend-side, comment this line bellow in `main.rs` to disable authentication middleware
https://github.com/SakaDream/actix-web-rest-api-with-jwt/blob/636d6e548f60d341c05707a0e5d3f4e1ee02e60a/src/main.rs#L70

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
  -H "Content-Type: application/json" \
  --data '{
    "username": "user",
    "email": "user@email.com",
    "password": "4S3cr3tPa55w0rd"
  }'
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
  --data '{"username_or_email":"user",  "password":"4S3cr3tPa55w0rd"}'
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

### `POST /api/auth/logout`: Logout
```bash
curl -X POST -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzcyNTc4NzksImV4cCI6MTU3Nzg2MjY3OSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiYzUxNWE3NTg3NGYzNGVjNGFmNDJmNWE2M2QxMDVjMGYifQ.B9w6FxFdypb5GCRMKXZ9CZWFxQLFjvmPSusMCtcE-Ac' \
  -i 'http://127.0.0.1:8000/api/auth/logout'
```

### `GET /api/address-book`: Get all people information
```bash
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book'
```
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
```bash
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book/2'
```
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
```bash
curl -X GET -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book/user'
```
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
```bash
curl -X POST -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book' \
  --data '{
    "name": "c",
    "gender": true,
    "age": 32,
    "address": "addr",
    "phone": "133",
    "email": "e@q.com"
  }
'
```
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
```bash
curl -X PUT -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book/2' \
  --data '{
    "name": "b",
    "gender": true,
    "age": 32,
    "address": "addr",
    "phone": "133",
    "email": "b@q.com"
  }
'
```
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
```bash
curl -X DELETE -H 'Content-Type: application/json' \
  -H 'Authorization: bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpYXQiOjE1NzU4NzM4MjksImV4cCI6MTU3NjQ3ODYyOSwidXNlciI6ImMiLCJsb2dpbl9zZXNzaW9uIjoiZjU5N2M3MTIxZTExNDBhMGE0ZjE0YmQ4N2NjM2Q4MWUifQ.6qppDfRgOw45eExJ7MUEwpcu3AUXXe9_ifj_mp7k22k' \
  -i 'http://127.0.0.1:8000/api/address-book/2'
```
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

### brower OPTIONS curl request example
```bash
curl -X OPTIONS -i 'http://127.0.0.1:8000/api/login' \
  -H "Origin: http://example.com" -H "Access-Control-Request-Method: POST"
```
  - Response
  ```
  HTTP/1.1 200 OK
  content-length: 0
  access-control-max-age: 3600
  access-control-allow-methods: POST,DELETE,GET,PUT
  access-control-allow-origin: *
  access-control-allow-headers: authorization,content-type,accept
  date: Tue, 07 Jan 2020 15:17:48 GMT
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
