# drop tables if they exist
DROP TABLE IF EXISTS friends;
DROP TABLE IF EXISTS characters;
DROP TABLE IF EXISTS users;
DROP TABLE IF EXISTS lobbies;


# create tables
CREATE TABLE characters (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255),
    health INT,
    strength INT
);

INSERT INTO characters (name, health, strength)
VALUES
    ('Warrior', 100, 80),
    ('Mage', 70, 90),
    ('Rogue', 85, 75),
    ('Zombie', 120, 60),
    ('Minotaur', 150, 40);

CREATE TABLE lobbies (
    id INT AUTO_INCREMENT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    state INT NOT NULL,
    max_players INT NOT NULL
);

CREATE TABLE users (
   id INT AUTO_INCREMENT PRIMARY KEY,
   username VARCHAR(255) NOT NULL,
   password VARCHAR(255) NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   times_logged_in INT DEFAULT 1,
   points INT DEFAULT 0,
   character_id INT DEFAULT 1 NOT NULL,
   lobby_id INT,
   FOREIGN KEY (character_id) REFERENCES characters(id)
);

CREATE TABLE friends (
    user_id INT REFERENCES users(id) ON DELETE CASCADE,
    friend_id INT REFERENCES users(id) ON DELETE CASCADE,
    PRIMARY KEY (user_id, friend_id)
);

