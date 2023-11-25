
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

CREATE TABLE IF NOT EXISTS roles (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    role_name VARCHAR(255) NOT NULL UNIQUE,
    active BOOLEAN DEFAULT true,
    create_timestamp TIMESTAMP,
    create_user VARCHAR(15),
    update_timestamp TIMESTAMP,
    update_user VARCHAR(15)
);

CREATE TABLE IF NOT EXISTS user_role_xrefs (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    role_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL,
    create_timestamp TIMESTAMP,
    create_user VARCHAR(15),
    update_timestamp TIMESTAMP,
    update_user VARCHAR(15)
);

CREATE TABLE IF NOT EXISTS permissions (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    target VARCHAR(255) NOT NULL UNIQUE,
    description TEXT,
    create_timestamp TIMESTAMP,
    create_user VARCHAR(15),
    update_timestamp TIMESTAMP,
    update_user VARCHAR(15)
);


CREATE TABLE IF NOT EXISTS role_permission_xrefs (
    id INTEGER AUTO_INCREMENT PRIMARY KEY,
    role_id INTEGER NOT NULL,
    permssion_id INTEGER NOT NULL,
    create_timestamp TIMESTAMP,
    create_user VARCHAR(15),
    update_timestamp TIMESTAMP,
    update_user VARCHAR(15)
)