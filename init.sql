-- Database initialization script
-- This runs when PostgreSQL container starts for the first time

-- Connect to the speedstream_db database
\c speedstream_db;

-- Create the speed table
CREATE TABLE speed
(
    id         SERIAL PRIMARY KEY,
    speed      FLOAT NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);