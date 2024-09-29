# API ENDPOINTS DOCUMENTATION

## [GET] get all users
test with: `curl -i -X GET http://localhost:8080/api/auth/users`

## [GET] get user
test with: `curl -i -X GET http://localhost:8080/api/auth/user/testuser`

## [POST] register new user
test with: `curl -X POST http://localhost:8080/api/auth/register -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`

## [POST] login user
test with: `curl -X POST http://localhost:8080/api/auth/login -H "Content-Type: application/json" -d '{"username": "testuser", "password": "password123"}'`

## [POST] update user
test with: `curl -X PUT http://localhost:8080/api/auth/user/1 -H "Content-Type: application/json" -d '{"username": "testuser", "character_id": 2}'`

## [GET] check token
test with: `curl -X GET http://localhost:8080/api/auth/checkToken -H "Authorization: Bearer eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiI3IiwiZXhwIjoxNzI2NjYzNTUwfQ.ZrK4Mtl2h6z4hs19FhhqEVpjnaTDD77fNn_cMvu7rxY"`

## [GET] get all characters
test with: `curl -i -X GET http://localhost:8080/api/characters`

## [POST] add friend
test with: `curl -i -X POST http://localhost:8080/api/friends/add -H "Content-Type: application/json" -d '{"user_id": 1, "friend_id": 2}'`

## [POST] remove friend
test with: `curl -i -X POST http://localhost:8080/api/friends/remove -H "Content-Type: application/json" -d '{"user_id": 1, "friend_id": 2}'`

## [POST] create new lobby
test with: `curl -i -X POST http://localhost:8080/api/lobbies -H "Content-Type: application/json" -d '{"name": "test_lobby", "state":0, "max_players":5}'`

## [POST] add user to lobby
test with: `curl -i -X POST http://localhost:8080/api/lobbies/add -H "Content-Type: application/json" -d '{"user_id": 1, "lobby_id": 1}'`

## [GET] get all lobbies
test with: `curl -i -X GET http://localhost:8080/api/lobbies`

## [GET] get lobby by name
test with: `curl -i -X GET http://localhost:8080/api/lobbies/test_lobby`