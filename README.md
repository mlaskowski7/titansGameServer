# API ENDPOINTS

## [GET] get all users
test with: `curl -i -X GET http://localhost:8080/api/auth/users`

## [GET] get user
test with: `curl -i -X GET http://localhost:8080/api/auth/user/testuser`

## [POST] register new user
test with: `curl -X POST http://localhost:8080/api/auth/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`

## [POST] login user
test with: `curl -X POST http://localhost:8080/api/auth/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`

## [GET] check token
test with: `curl -X GET http://localhost:8080/api/auth/checkToken -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI3IiwiZXhwIjoxNzI2NjYzNTUwfQ.ZrK4Mtl2h6z4hs19FhhqEVpjnaTDD77fNn_cMvu7rxY"`

## [GET] get all characters
test with: `curl -i -X GET http://localhost:8080/api/characters`