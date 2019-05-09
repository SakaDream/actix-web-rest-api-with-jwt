# Actix-web REST API

A simple backend app using Actix-web

# Require

- [Rust Stable](https://rustup.rs)

# How to run

- Build and run project: `cargo run`
- Enjoy! 😄

# APIs

## Address: **`localhost:8080`**

### **`GET /api/ping`**: Ping

- Response:
    - 200 OK
    ```
    pong!
    ```

### `GET /api/address-book`: Get all people information
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
