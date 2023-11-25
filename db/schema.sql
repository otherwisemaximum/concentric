
CREATE TABLE IF NOT EXISTS users (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(15) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password TEXT NOT NULL,
    active BOOLEAN,
    internal_flag BOOLEAN DEFAULT false,
    create_timestamp TIMESTAMP,
    create_user VARCHAR(15),
    update_timestamp TIMESTAMP,
    update_user VARCHAR(15)
);