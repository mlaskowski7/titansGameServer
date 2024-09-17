# API ENDPOINTS

## [GET] get all users
test with: `curl -i -X GET http://localhost:8080/api/auth/users`

## [GET] get user
test with: `curl -i -X GET http://localhost:8080/api/auth/user/testuser`

## [POST] register new user
test with: `curl -X POST http://localhost:8080/api/auth/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`

## [POST] login user
test with: `curl -X POST http://localhost:8080/api/auth/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`