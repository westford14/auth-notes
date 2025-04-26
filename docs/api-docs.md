# REST API Documentation

## Health Check (Public)

**Endpoint:** `GET /v1/health`

**Description:** Checks the health of the service.

---

## Version Check (Public)

**Endpoint:** `GET /v1/version`

**Description:** Retrieves the version of the service.

---

## Login

**Endpoint:** `POST /v1/auth/login`

**Description:** Authenticates a user and returns tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`

**Request Body:**

```json
{
    "username": "admin",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51"
}
```

---

## Root (Protected)

**Endpoint:** `GET /`

**Description:** Accesses the root endpoint.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## Refresh Tokens

**Endpoint:** `POST /v1/auth/refresh`

**Description:** Refreshes the tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <refresh_token>`

---

## Logout

**Endpoint:** `POST /v1/auth/logout`

**Description:** Logs out the user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <refresh_token>`

---

## Revoke Tokens Issued to the User

**Endpoint:** `POST /v1/auth/revoke-user`

**Description:** Revokes tokens issued to the user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{ "user_id" : "617646a0-7437-48a0-bb03-a7aa830f8f81" }
```

---

## Revoke All Issued Tokens

**Endpoint:** `POST /v1/auth/revoke-all`

**Description:** Revokes all issued tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## Cleanup Revoked Tokens

**Endpoint:** `POST /v1/auth/cleanup`

**Description:** Cleans up revoked tokens.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

---

## List Users

**Endpoint:** `GET /v1/users`

**Description:** Lists all users.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Get User by ID

**Endpoint:** `GET /v1/users/{user_id}`

**Description:** Retrieves a user by ID.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Add a New User

**Endpoint:** `POST /v1/users`

**Description:** Adds a new user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "username": "admin2",
    "email": "admin2@admin.com",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51",
    "password_salt": "pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF"
}
```

---

## Update User

**Endpoint:** `PUT /v1/users/{user_id}`

**Description:** Updates a user.

**Headers:**

- `Content-Type: application/json; charset=utf8`
- `Authorization: Bearer <access_token>`

**Request Body:**

```json
{
    "id": "917646a0-7437-48a0-bb03-a7aa830f8f81",
    "username": "admin21",
    "email": "admin21@admin.com",
    "password_hash": "7c44575b741f02d49c3e988ba7aa95a8fb6d90c0ef63a97236fa54bfcfbd9d51",
    "password_salt": "pjZKk6A8YtC8$9p&UIp62bv4PLwD7@dF"
}
```

---

## Delete User

**Endpoint:** `DELETE /v1/users/{user_id}`

**Description:** Deletes a user.

**Headers:**

- `Authorization: Bearer <access_token>`

---

## Errors

### The possible error codes and description

- `authentication_wrong_credentials`: The provided credentials are incorrect.
- `authentication_missing_credentials`: Required authentication credentials are missing.
- `authentication_token_creation_error`: There was an error creating the authentication token.
- `authentication_invalid_token`: The provided authentication token is invalid.
- `authentication_revoked_tokens_inactive`: The provided token has been revoked and is inactive.
- `authentication_forbidden`: The user does not have permission to access the requested resource.
- `user_not_found`: The specified user was not found.
- `resource_not_found`: The requested resource was not found.
- `api_version_error`: There is an error with the API version.
- `database_error`: There was an error with the database operation.
- `redis_error`: There was an error with the Redis operation.

### The possible error kinds and description

- `authentication_error`: An error occurred during the authentication process.
- `resource_not_found`: The requested resource could not be found.
- `validation_error`: There was a validation error with the provided data.
- `database_error`: An error occurred with the database operation.
- `redis_error`: An error occurred with the Redis operation.

### API error response samples

```json
 {
   "status": 404,
   "errors": [
     {
         "code": "user_not_found",
         "kind": "resource_not_found",
         "message": "user not found: 12345",
         "description": "user with the ID '12345' does not exist in our records",
         "detail": { "user_id": "12345" },
         "reason": "must be an existing user",
         "instance": "/api/v1/users/12345",
         "trace_id": "3d2b4f2d00694354a00522fe3bb86158",
         "timestamp": "2024-01-19T16:58:34.123+0000",
         "help": "please check if the user ID is correct or refer to our documentation at https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md#errors for more information",
         "doc_url": "https://github.com/sheroz/axum-rest-api-sample/blob/main/docs/api-docs.md"
     }
   ]
 }
```
