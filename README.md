# API ENDPOINTS

## [GET] get all users
test with: `curl -i -X GET http://localhost:8080/api/auth/users`

## [POST] register new user
test with: `curl -X POST http://localhost:8080/api/auth/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`
` 