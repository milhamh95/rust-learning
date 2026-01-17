# User REST API

A simple REST API for user management built with Rust and Actix-web.

## Quick Start

```bash
# Run the server
cargo run

# Server starts on http://127.0.0.1:8080
```

## API Endpoints

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | `/users/` | Create a new user |
| GET | `/users/` | Get all users |
| GET | `/users/{id}` | Get user by ID |
| PUT | `/users/{id}` | Update user |
| DELETE | `/users/{id}` | Delete user |

## Example Usage

```bash
# Create a user
curl -X POST http://localhost:8080/users/ \
  -H "Content-Type: application/json" \
  -d '{"name":"John Doe","email":"john@example.com"}'

# Get all users
curl http://localhost:8080/users/

# Get user by ID
curl http://localhost:8080/users/{id}

# Update user
curl -X PUT http://localhost:8080/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"name":"Jane Doe","email":"jane@example.com"}'

# Delete user
curl -X DELETE http://localhost:8080/users/{id}
```
