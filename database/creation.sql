CREATE TABLE locations (
    id SERIAL PRIMARY KEY,
    address VARCHAR(225),
    
);

CREATE TABLE users (
    id SERIAL PRIMARY KEY,
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    favorates INT PRIMARY KEY REFERENCES locations(id)
);