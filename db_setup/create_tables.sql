# drop tables if they exist
DROP TABLE users;
DROP TABLE characters;

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

CREATE TABLE users (
   id INT AUTO_INCREMENT PRIMARY KEY,
   username VARCHAR(255) NOT NULL,
   password VARCHAR(255) NOT NULL,
   created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
   times_logged_in INT DEFAULT 1,
   character_id INT DEFAULT 1 NOT NULL,
   FOREIGN KEY (character_id) REFERENCES characters(id)
);

